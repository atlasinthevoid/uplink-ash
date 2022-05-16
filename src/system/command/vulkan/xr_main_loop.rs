use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use ash::vk::{self, Handle};
use openxr as xr;

use std::time::Duration;

pub const VIEW_COUNT: u32 = 2;
pub const COLOR_FORMAT: vk::Format = vk::Format::R8G8B8A8_SRGB;
use super::Framebuffer;
use super::Swapchain;

const PIPELINE_DEPTH: u32 = 2;

pub async unsafe fn xr_main_loop(
    xr_instance: &xr::Instance,
    running: &Arc<AtomicBool>,
    session: &xr::Session<xr::Vulkan>,
    frame_wait: &mut xr::FrameWaiter,
    frame_stream: &mut xr::FrameStream<xr::Vulkan>,
    environment_blend_mode: &xr::EnvironmentBlendMode,
    system: &xr::SystemId,
    vk_device: &ash::Device,
    render_pass: &ash::vk::RenderPass,
    stage: &xr::Space,
    fences: &Vec<ash::vk::Fence>,
    cmds: &Vec<ash::vk::CommandBuffer>,
    pipeline: &ash::vk::Pipeline,
    action_set: &xr::ActionSet,
    right_space: &xr::Space,
    left_space: &xr::Space,
    right_action: &xr::Action<xr::Posef>,
    left_action: &xr::Action<xr::Posef>,
    queue: &vk::Queue,
) -> Option<Swapchain> {
    // Main loop
    let mut swapchain = None;
    let mut event_storage = xr::EventDataBuffer::new();
    let mut session_running = false;
    // Index of the current frame, wrapped by PIPELINE_DEPTH. Not to be confused with the
    // swapchain image index.
    let mut frame = 0;
    'main_loop: loop {
        if !running.load(Ordering::Relaxed) {
            println!("requesting exit");
            // The OpenXR runtime may want to perform a smooth transition between scenes, so we
            // can't necessarily exit instantly. Instead, we must notify the runtime of our
            // intent and wait for it to tell us when we're actually done.
            match session.request_exit() {
                Ok(()) => {}
                Err(xr::sys::Result::ERROR_SESSION_NOT_RUNNING) => break,
                Err(e) => panic!("{}", e),
            }
        }

        while let Some(event) = xr_instance.poll_event(&mut event_storage).unwrap() {
            use xr::Event::*;
            match event {
                SessionStateChanged(e) => {
                    // Session state change is where we can begin and end sessions, as well as
                    // find quit messages!
                    println!("entered state {:?}", e.state());
                    match e.state() {
                        xr::SessionState::READY => {
                            session.begin(super::VIEW_TYPE).unwrap();
                            session_running = true;
                        }
                        xr::SessionState::STOPPING => {
                            session.end().unwrap();
                            session_running = false;
                        }
                        xr::SessionState::EXITING | xr::SessionState::LOSS_PENDING => {
                            break 'main_loop;
                        }
                        _ => {}
                    }
                }
                InstanceLossPending(_) => {
                    break 'main_loop;
                }
                EventsLost(e) => {
                    println!("lost {} events", e.lost_event_count());
                }
                _ => {}
            }
        }

        if !session_running {
            // Don't grind up the CPU
            std::thread::sleep(Duration::from_millis(100));
            continue;
        }

        // Block until the previous frame is finished displaying, and is ready for another one.
        // Also returns a prediction of when the next frame will be displayed, for use with
        // predicting locations of controllers, viewpoints, etc.
        let xr_frame_state = frame_wait.wait().unwrap();
        // Must be called before any rendering is done!
        frame_stream.begin().unwrap();

        if !xr_frame_state.should_render {
            frame_stream
                .end(
                    xr_frame_state.predicted_display_time,
                    *environment_blend_mode,
                    &[],
                )
                .unwrap();
            continue;
        }

        let swapchain = swapchain.get_or_insert_with(|| {
            // Now we need to find all the viewpoints we need to take care of! This is a
            // property of the view configuration type; in this example we use PRIMARY_STEREO,
            // so we should have 2 viewpoints.
            //
            // Because we are using multiview in this example, we require that all view
            // dimensions are identical.
            let views = xr_instance
                .enumerate_view_configuration_views(*system, super::VIEW_TYPE)
                .unwrap();
            assert_eq!(views.len(), VIEW_COUNT as usize);
            assert_eq!(views[0], views[1]);

            // Create a swapchain for the viewpoints! A swapchain is a set of texture buffers
            // used for displaying to screen, typically this is a backbuffer and a front buffer,
            // one for rendering data to, and one for displaying on-screen.
            let resolution = vk::Extent2D {
                width: views[0].recommended_image_rect_width,
                height: views[0].recommended_image_rect_height,
            };
            let handle = session
                .create_swapchain(&xr::SwapchainCreateInfo {
                    create_flags: xr::SwapchainCreateFlags::EMPTY,
                    usage_flags: xr::SwapchainUsageFlags::COLOR_ATTACHMENT
                        | xr::SwapchainUsageFlags::SAMPLED,
                    format: COLOR_FORMAT.as_raw() as _,
                    // The Vulkan graphics pipeline we create is not set up for multisampling,
                    // so we hardcode this to 1. If we used a proper multisampling setup, we
                    // could set this to `views[0].recommended_swapchain_sample_count`.
                    sample_count: 1,
                    width: resolution.width,
                    height: resolution.height,
                    face_count: 1,
                    array_size: VIEW_COUNT,
                    mip_count: 1,
                })
                .unwrap();

            // We'll want to track our own information about the swapchain, so we can draw stuff
            // onto it! We'll also create a buffer for each generated texture here as well.
            let images = handle.enumerate_images().unwrap();
            Swapchain {
                handle,
                resolution,
                buffers: images
                    .into_iter()
                    .map(|color_image| {
                        let color_image = vk::Image::from_raw(color_image);
                        let color = vk_device
                            .create_image_view(
                                &vk::ImageViewCreateInfo::builder()
                                    .image(color_image)
                                    .view_type(vk::ImageViewType::TYPE_2D_ARRAY)
                                    .format(COLOR_FORMAT)
                                    .subresource_range(vk::ImageSubresourceRange {
                                        aspect_mask: vk::ImageAspectFlags::COLOR,
                                        base_mip_level: 0,
                                        level_count: 1,
                                        base_array_layer: 0,
                                        layer_count: VIEW_COUNT,
                                    }),
                                None,
                            )
                            .unwrap();
                        let framebuffer = vk_device
                            .create_framebuffer(
                                &vk::FramebufferCreateInfo::builder()
                                    .render_pass(*render_pass)
                                    .width(resolution.width)
                                    .height(resolution.height)
                                    .attachments(&[color])
                                    .layers(1), // Multiview handles addressing multiple layers
                                None,
                            )
                            .unwrap();
                        Framebuffer { framebuffer, color }
                    })
                    .collect(),
            }
        });

        // We need to ask which swapchain image to use for rendering! Which one will we get?
        // Who knows! It's up to the runtime to decide.
        let image_index = swapchain.handle.acquire_image().unwrap();

        // Wait until the image is available to render to. The compositor could still be
        // reading from it.
        swapchain.handle.wait_image(xr::Duration::INFINITE).unwrap();

        // Ensure the last use of this frame's resources is 100% done
        vk_device
            .wait_for_fences(&[fences[frame]], true, u64::MAX)
            .unwrap();
        vk_device.reset_fences(&[fences[frame]]).unwrap();

        let cmd = cmds[frame];
        vk_device
            .begin_command_buffer(
                cmd,
                &vk::CommandBufferBeginInfo::builder()
                    .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT),
            )
            .unwrap();
        vk_device.cmd_begin_render_pass(
            cmd,
            &vk::RenderPassBeginInfo::builder()
                .render_pass(*render_pass)
                .framebuffer(swapchain.buffers[image_index as usize].framebuffer)
                .render_area(vk::Rect2D {
                    offset: vk::Offset2D::default(),
                    extent: swapchain.resolution,
                })
                .clear_values(&[vk::ClearValue {
                    color: vk::ClearColorValue {
                        float32: [0.0, 0.0, 0.0, 1.0],
                    },
                }]),
            vk::SubpassContents::INLINE,
        );

        let viewports = [vk::Viewport {
            x: 0.0,
            y: 0.0,
            width: swapchain.resolution.width as f32,
            height: swapchain.resolution.height as f32,
            min_depth: 0.0,
            max_depth: 1.0,
        }];
        let scissors = [vk::Rect2D {
            offset: vk::Offset2D { x: 0, y: 0 },
            extent: swapchain.resolution,
        }];
        vk_device.cmd_set_viewport(cmd, 0, &viewports);
        vk_device.cmd_set_scissor(cmd, 0, &scissors);

        // Draw the scene. Multiview means we only need to do this once, and the GPU will
        // automatically broadcast operations to all views. Shaders can use `gl_ViewIndex` to
        // e.g. select the correct view matrix.
        vk_device.cmd_bind_pipeline(cmd, vk::PipelineBindPoint::GRAPHICS, *pipeline);
        vk_device.cmd_draw(cmd, 3, 1, 0, 0);

        vk_device.cmd_end_render_pass(cmd);
        vk_device.end_command_buffer(cmd).unwrap();

        session.sync_actions(&[(action_set).into()]).unwrap();

        // Find where our controllers are located in the Stage space
        let right_location = right_space
            .locate(&stage, xr_frame_state.predicted_display_time)
            .unwrap();

        let left_location = left_space
            .locate(&stage, xr_frame_state.predicted_display_time)
            .unwrap();

        let mut printed = false;
        if left_action.is_active(&session, xr::Path::NULL).unwrap() {
            print!(
                "Left Hand: ({:0<12},{:0<12},{:0<12}), ",
                left_location.pose.position.x,
                left_location.pose.position.y,
                left_location.pose.position.z
            );
            printed = true;
        }

        if right_action.is_active(&session, xr::Path::NULL).unwrap() {
            print!(
                "Right Hand: ({:0<12},{:0<12},{:0<12})",
                right_location.pose.position.x,
                right_location.pose.position.y,
                right_location.pose.position.z
            );
            printed = true;
        }
        if printed {
            println!();
        }

        // Fetch the view transforms. To minimize latency, we intentionally do this *after*
        // recording commands to render the scene, i.e. at the last possible moment before
        // rendering begins in earnest on the GPU. Uniforms dependent on this data can be sent
        // to the GPU just-in-time by writing them to per-frame host-visible memory which the
        // GPU will only read once the command buffer is submitted.
        let (_, views) = session
            .locate_views(
                super::VIEW_TYPE,
                xr_frame_state.predicted_display_time,
                &stage,
            )
            .unwrap();

        // Submit commands to the GPU, then tell OpenXR we're done with our part.
        vk_device
            .queue_submit(
                *queue,
                &[vk::SubmitInfo::builder().command_buffers(&[cmd]).build()],
                fences[frame],
            )
            .unwrap();
        swapchain.handle.release_image().unwrap();

        // Tell OpenXR what to present for this frame
        let rect = xr::Rect2Di {
            offset: xr::Offset2Di { x: 0, y: 0 },
            extent: xr::Extent2Di {
                width: swapchain.resolution.width as _,
                height: swapchain.resolution.height as _,
            },
        };
        frame_stream
            .end(
                xr_frame_state.predicted_display_time,
                *environment_blend_mode,
                &[
                    &xr::CompositionLayerProjection::new().space(&stage).views(&[
                        xr::CompositionLayerProjectionView::new()
                            .pose(views[0].pose)
                            .fov(views[0].fov)
                            .sub_image(
                                xr::SwapchainSubImage::new()
                                    .swapchain(&swapchain.handle)
                                    .image_array_index(0)
                                    .image_rect(rect),
                            ),
                        xr::CompositionLayerProjectionView::new()
                            .pose(views[1].pose)
                            .fov(views[1].fov)
                            .sub_image(
                                xr::SwapchainSubImage::new()
                                    .swapchain(&swapchain.handle)
                                    .image_array_index(1)
                                    .image_rect(rect),
                            ),
                    ]),
                ],
            )
            .unwrap();
        frame = (frame + 1) % PIPELINE_DEPTH as usize;
    }
    swapchain
}

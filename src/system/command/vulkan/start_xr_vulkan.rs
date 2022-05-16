//! Illustrates rendering using Vulkan with multiview. Supports any Vulkan 1.1 capable environment.
//!
//! Renders a smooth gradient across the entire view, with different colors per eye.
//!
//! This example uses minimal abstraction for clarity. Real-world code should encapsulate and
//! largely decouple its Vulkan and OpenXR components and handle errors gracefully.

use std::{
    io::Cursor,
    sync::{atomic::AtomicBool, Arc},
};

use ash::{
    util::read_spv,
    vk::{self, Handle},
};
use openxr as xr;

use super::pipeline;
use super::render_pass;

use super::xr_main_loop;

pub const COLOR_FORMAT: vk::Format = vk::Format::R8G8B8A8_SRGB;
pub const VIEW_COUNT: u32 = 2;

/// Maximum number of frames in flight
const PIPELINE_DEPTH: u32 = 2;

#[allow(clippy::field_reassign_with_default)] // False positive, might be fixed 1.51
#[cfg_attr(target_os = "android", ndk_glue::main)]
pub async unsafe fn start_xr_vulkan(
    xr_instance: &xr::Instance,
    system: &xr::SystemId,
    environment_blend_mode: &xr::EnvironmentBlendMode,
    running: &Arc<AtomicBool>,
    vk_instance: &ash::Instance,
    vk_device: &ash::Device,
    queue_family_index: &u32,
    vk_physical_device: &ash::vk::PhysicalDevice,
) {
    let queue = vk_device.get_device_queue(*queue_family_index, 0);

    let view_mask = !(!0 << VIEW_COUNT);
    let render_pass = render_pass(&vk_device, &view_mask).await;

    let vert = read_spv(&mut Cursor::new(
        &include_bytes!("../../../shader/fullscreen.vert.spv")[..],
    ))
    .unwrap();
    let frag = read_spv(&mut Cursor::new(
        &include_bytes!("../../../shader/debug_pattern.frag.spv")[..],
    ))
    .unwrap();
    let vert = vk_device
        .create_shader_module(&vk::ShaderModuleCreateInfo::builder().code(&vert), None)
        .unwrap();
    let frag = vk_device
        .create_shader_module(&vk::ShaderModuleCreateInfo::builder().code(&frag), None)
        .unwrap();

    let pipe = pipeline(&vk_device, &vert, &frag, &render_pass).await;
    let pipeline_layout = pipe.0;
    let pipeline = pipe.1;

    vk_device.destroy_shader_module(vert, None);
    vk_device.destroy_shader_module(frag, None);

    // A session represents this application's desire to display things! This is where we hook
    // up our graphics API. This does not start the session; for that, you'll need a call to
    // Session::begin, which we do in 'main_loop below.
    let (session, mut frame_wait, mut frame_stream) = xr_instance
        .create_session::<xr::Vulkan>(
            *system,
            &xr::vulkan::SessionCreateInfo {
                instance: vk_instance.handle().as_raw() as _,
                physical_device: vk_physical_device.as_raw() as _,
                device: vk_device.handle().as_raw() as _,
                queue_family_index: *queue_family_index,
                queue_index: 0,
            },
        )
        .unwrap();

    // Create an action set to encapsulate our actions
    let action_set = xr_instance
        .create_action_set("input", "input pose information", 0)
        .unwrap();

    let right_action = action_set
        .create_action::<xr::Posef>("right_hand", "Right Hand Controller", &[])
        .unwrap();
    let left_action = action_set
        .create_action::<xr::Posef>("left_hand", "Left Hand Controller", &[])
        .unwrap();

    // Bind our actions to input devices using the given profile
    // If you want to access inputs specific to a particular device you may specify a different
    // interaction profile
    xr_instance
        .suggest_interaction_profile_bindings(
            xr_instance
                .string_to_path("/interaction_profiles/khr/simple_controller")
                .unwrap(),
            &[
                xr::Binding::new(
                    &right_action,
                    xr_instance
                        .string_to_path("/user/hand/right/input/grip/pose")
                        .unwrap(),
                ),
                xr::Binding::new(
                    &left_action,
                    xr_instance
                        .string_to_path("/user/hand/left/input/grip/pose")
                        .unwrap(),
                ),
            ],
        )
        .unwrap();

    // Attach the action set to the session
    session.attach_action_sets(&[&action_set]).unwrap();

    // Create an action space for each device we want to locate
    let right_space = right_action
        .create_space(session.clone(), xr::Path::NULL, xr::Posef::IDENTITY)
        .unwrap();
    let left_space = left_action
        .create_space(session.clone(), xr::Path::NULL, xr::Posef::IDENTITY)
        .unwrap();

    // OpenXR uses a couple different types of reference frames for positioning content; we need
    // to choose one for displaying our content! STAGE would be relative to the center of your
    // guardian system's bounds, and LOCAL would be relative to your device's starting location.
    let stage = session
        .create_reference_space(xr::ReferenceSpaceType::STAGE, xr::Posef::IDENTITY)
        .unwrap();

    let cmd_pool = vk_device
        .create_command_pool(
            &vk::CommandPoolCreateInfo::builder()
                .queue_family_index(*queue_family_index)
                .flags(
                    vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER
                        | vk::CommandPoolCreateFlags::TRANSIENT,
                ),
            None,
        )
        .unwrap();
    let cmds = vk_device
        .allocate_command_buffers(
            &vk::CommandBufferAllocateInfo::builder()
                .command_pool(cmd_pool)
                .command_buffer_count(PIPELINE_DEPTH),
        )
        .unwrap();
    let fences = (0..PIPELINE_DEPTH)
        .map(|_| {
            vk_device
                .create_fence(
                    &vk::FenceCreateInfo::builder().flags(vk::FenceCreateFlags::SIGNALED),
                    None,
                )
                .unwrap()
        })
        .collect::<Vec<_>>();

    let swapchain = xr_main_loop(
        xr_instance,
        running,
        &session,
        &mut frame_wait,
        &mut frame_stream,
        environment_blend_mode,
        system,
        vk_device,
        &render_pass,
        &stage,
        &fences,
        &cmds,
        &pipeline,
        &action_set,
        &right_space,
        &left_space,
        &right_action,
        &left_action,
        &queue,
    )
    .await;
    // OpenXR MUST be allowed to clean up before we destroy Vulkan resources it could touch, so
    // first we must drop all its handles.
    drop((
        session,
        frame_wait,
        frame_stream,
        stage,
        action_set,
        left_space,
        right_space,
        left_action,
        right_action,
    ));

    // Ensure all in-flight frames are finished before destroying resources they might use
    vk_device.wait_for_fences(&fences, true, !0).unwrap();
    for fence in fences {
        vk_device.destroy_fence(fence, None);
    }

    if let Some(swapchain) = swapchain {
        for buffer in swapchain.buffers {
            vk_device.destroy_framebuffer(buffer.framebuffer, None);
            vk_device.destroy_image_view(buffer.color, None);
        }
    }

    vk_device.destroy_pipeline(pipeline, None);
    vk_device.destroy_pipeline_layout(pipeline_layout, None);
    vk_device.destroy_command_pool(cmd_pool, None);
    vk_device.destroy_render_pass(render_pass, None);
    vk_device.destroy_device(None);
    vk_instance.destroy_instance(None);
    println!("vr exiting...");
    println!("please close this app with SteamVR to prevent broken xr state");
}

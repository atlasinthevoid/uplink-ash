use ash::vk::{self};

pub const COLOR_FORMAT: vk::Format = vk::Format::R8G8B8A8_SRGB;
pub const VIEW_COUNT: u32 = 2;

pub async unsafe fn render_pass(vk_device: &ash::Device, view_mask: &u32) -> ash::vk::RenderPass {
    let render_pass = vk_device
        .create_render_pass(
            &vk::RenderPassCreateInfo::builder()
                .attachments(&[vk::AttachmentDescription {
                    format: COLOR_FORMAT,
                    samples: vk::SampleCountFlags::TYPE_1,
                    load_op: vk::AttachmentLoadOp::CLEAR,
                    store_op: vk::AttachmentStoreOp::STORE,
                    initial_layout: vk::ImageLayout::UNDEFINED,
                    final_layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
                    ..Default::default()
                }])
                .subpasses(&[vk::SubpassDescription::builder()
                    .color_attachments(&[vk::AttachmentReference {
                        attachment: 0,
                        layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
                    }])
                    .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
                    .build()])
                .dependencies(&[vk::SubpassDependency {
                    src_subpass: vk::SUBPASS_EXTERNAL,
                    dst_subpass: 0,
                    src_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
                    dst_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
                    dst_access_mask: vk::AccessFlags::COLOR_ATTACHMENT_WRITE,
                    ..Default::default()
                }])
                .push_next(
                    &mut vk::RenderPassMultiviewCreateInfo::builder()
                        .view_masks(&[*view_mask])
                        .correlation_masks(&[*view_mask]),
                ),
            None,
        )
        .unwrap();
    render_pass
}

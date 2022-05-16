use ash::vk::{self};

pub const COLOR_FORMAT: vk::Format = vk::Format::R8G8B8A8_SRGB;
pub const VIEW_COUNT: u32 = 2;

pub async unsafe fn pipeline(
    vk_device: &ash::Device,
    vert: &vk::ShaderModule,
    frag: &vk::ShaderModule,
    render_pass: &vk::RenderPass,
) -> (vk::PipelineLayout, vk::Pipeline) {
    let pipeline_layout = vk_device
        .create_pipeline_layout(
            &vk::PipelineLayoutCreateInfo::builder().set_layouts(&[]),
            None,
        )
        .unwrap();
    let noop_stencil_state = vk::StencilOpState {
        fail_op: vk::StencilOp::KEEP,
        pass_op: vk::StencilOp::KEEP,
        depth_fail_op: vk::StencilOp::KEEP,
        compare_op: vk::CompareOp::ALWAYS,
        compare_mask: 0,
        write_mask: 0,
        reference: 0,
    };
    let pipeline = vk_device
        .create_graphics_pipelines(
            vk::PipelineCache::null(),
            &[vk::GraphicsPipelineCreateInfo::builder()
                .stages(&[
                    vk::PipelineShaderStageCreateInfo {
                        stage: vk::ShaderStageFlags::VERTEX,
                        module: *vert,
                        p_name: b"main\0".as_ptr() as _,
                        ..Default::default()
                    },
                    vk::PipelineShaderStageCreateInfo {
                        stage: vk::ShaderStageFlags::FRAGMENT,
                        module: *frag,
                        p_name: b"main\0".as_ptr() as _,
                        ..Default::default()
                    },
                ])
                .vertex_input_state(&vk::PipelineVertexInputStateCreateInfo::default())
                .input_assembly_state(
                    &vk::PipelineInputAssemblyStateCreateInfo::builder()
                        .topology(vk::PrimitiveTopology::TRIANGLE_LIST),
                )
                .viewport_state(
                    &vk::PipelineViewportStateCreateInfo::builder()
                        .scissor_count(1)
                        .viewport_count(1),
                )
                .rasterization_state(
                    &vk::PipelineRasterizationStateCreateInfo::builder()
                        .cull_mode(vk::CullModeFlags::NONE)
                        .polygon_mode(vk::PolygonMode::FILL)
                        .line_width(1.0),
                )
                .multisample_state(
                    &vk::PipelineMultisampleStateCreateInfo::builder()
                        .rasterization_samples(vk::SampleCountFlags::TYPE_1),
                )
                .depth_stencil_state(
                    &vk::PipelineDepthStencilStateCreateInfo::builder()
                        .depth_test_enable(false)
                        .depth_write_enable(false)
                        .front(noop_stencil_state)
                        .back(noop_stencil_state),
                )
                .color_blend_state(
                    &vk::PipelineColorBlendStateCreateInfo::builder().attachments(&[
                        vk::PipelineColorBlendAttachmentState {
                            blend_enable: vk::TRUE,
                            src_color_blend_factor: vk::BlendFactor::ONE,
                            dst_color_blend_factor: vk::BlendFactor::ZERO,
                            color_blend_op: vk::BlendOp::ADD,
                            color_write_mask: vk::ColorComponentFlags::R
                                | vk::ColorComponentFlags::G
                                | vk::ColorComponentFlags::B,
                            ..Default::default()
                        },
                    ]),
                )
                .dynamic_state(
                    &vk::PipelineDynamicStateCreateInfo::builder()
                        .dynamic_states(&[vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR]),
                )
                .layout(pipeline_layout)
                .render_pass(*render_pass)
                .subpass(0)
                .build()],
            None,
        )
        .unwrap()[0];
    (pipeline_layout, pipeline)
}

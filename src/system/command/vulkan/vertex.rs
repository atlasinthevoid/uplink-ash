#[repr(C)]
#[derive(Clone, Copy)]
struct Vertex {
    pos: [f32; 3],
    color: [f32; 3],
}

impl Vertex {
    pub async fn cube(color: [f32; 3]) -> impl Iterator<Item = Self> {
        [1, 0, 3, 2, 6, 0, 4, 1, 5, 3, 7, 6, 5, 4]
            .iter()
            .map(move |i| Self {
                pos: [(i & 1) as f32, ((i >> 1) & 1) as f32, ((i >> 2) & 1) as f32],
                color: color.clone(),
            })
    }
    async fn get_binding_description() -> vk::VertexInputBindingDescription {
        vk::VertexInputBindingDescription::builder()
            .binding(0)
            .stride(mem::size_of::<Self>() as u32)
            .input_rate(vk::VertexInputRate::VERTEX)
            .build()
    }

    async fn get_attribute_descriptions() -> [vk::VertexInputAttributeDescription; 2] {
        let position_desc = vk::VertexInputAttributeDescription::builder()
            .binding(0)
            .location(0)
            .format(vk::Format::R32G32B32_SFLOAT)
            .offset(0)
            .build();
        let color_desc = vk::VertexInputAttributeDescription::builder()
            .binding(0)
            .location(1)
            .format(vk::Format::R32G32B32_SFLOAT)
            .offset(mem::size_of::<[f32; 3]>() as u32)
            .build();
        [position_desc, color_desc]
    }
}
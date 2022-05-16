use ash::vk::{self};

pub struct Framebuffer {
    pub framebuffer: vk::Framebuffer,
    pub color: vk::ImageView,
}

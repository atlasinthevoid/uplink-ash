use super::Framebuffer;
use ash::vk::{self};
use openxr as xr;

pub struct Swapchain {
    pub handle: xr::Swapchain<xr::Vulkan>,
    pub buffers: Vec<Framebuffer>,
    pub resolution: vk::Extent2D,
}

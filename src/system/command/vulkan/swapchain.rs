use ash::{
    vk::{self},
};
use super::Framebuffer;
use openxr as xr;

pub struct Swapchain {
    pub handle: xr::Swapchain<xr::Vulkan>,
    pub buffers: Vec<Framebuffer>,
    pub resolution: vk::Extent2D,
}
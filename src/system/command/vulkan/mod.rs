pub use super::State;
pub use super::Uuid;

pub mod framebuffer;
pub use framebuffer::Framebuffer;

pub mod start_vr;
pub use start_vr::start_vr;

pub mod swapchain;
pub use swapchain::Swapchain;

pub mod start_openxr;
pub use start_openxr::start_openxr;

pub mod vulkan_info;

pub mod start_xr_vulkan;
pub use start_xr_vulkan::start_xr_vulkan;

pub mod xr_main_loop;
pub use xr_main_loop::xr_main_loop;

pub mod vulkan_instance;
pub use vulkan_instance::vulkan_instance;

pub mod render_pass;
pub use render_pass::render_pass;

pub mod pipeline;
pub use pipeline::pipeline;

use openxr as xr;
pub const VIEW_TYPE: xr::ViewConfigurationType = xr::ViewConfigurationType::PRIMARY_STEREO;

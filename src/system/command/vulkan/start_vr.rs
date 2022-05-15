use super::State;
use super::Uuid;
use super::start_openxr;
use super::start_xr_vulkan;
use super::vulkan_instance;

use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

pub fn start_vr(_state: &mut State, _capability: Uuid) {
    // Handle interrupts gracefully
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::Relaxed);
    })
    .expect("setting Ctrl-C handler");

    unsafe {
        let instance = start_openxr();
        let vulkan_instance = vulkan_instance(&instance.0, &instance.1);
        start_xr_vulkan(instance.0, instance.1, instance.2, running, vulkan_instance.0, vulkan_instance.1, vulkan_instance.2, vulkan_instance.3);
    }
    //xr_main_loop(running);
}
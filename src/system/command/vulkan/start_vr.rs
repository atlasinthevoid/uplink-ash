use super::start_openxr;
use super::start_xr_vulkan;
use super::vulkan_instance;
use super::State;
use super::Uuid;

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

// DO NOT KILL THIS PROCESS
// killing the game while this function is running with put SteamVR in an unstable state

pub async fn start_vr(_state: &mut State, _capability: Uuid) {
    // Handle interrupts gracefully
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::Relaxed);
    })
    .expect("setting Ctrl-C handler");

    unsafe {
        init(&running).await;
    }
}

pub async unsafe fn init(running: &Arc<AtomicBool>) {
    match start_openxr().await {
        Ok(instance) => {
            let vulkan_instance = vulkan_instance(&instance.0, &instance.1).await;
            start_xr_vulkan(
                &instance.0,
                &instance.1,
                &instance.2,
                running,
                &vulkan_instance.0,
                &vulkan_instance.1,
                &vulkan_instance.2,
                &vulkan_instance.3,
            )
            .await;
        }
        Err(e) => {
            println!("Fatal error while creating openxr instance: {}", e);
        }
    }
}

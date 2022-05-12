use super::State;
use super::Uuid;
use super::start_openxr;
use super::start_xr_vulkan;

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

    let instance = start_openxr();
    start_xr_vulkan(instance.0, instance.1, instance.2, running);
    //xr_main_loop(running);
}
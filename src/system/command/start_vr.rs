use super::State;
use super::Uuid;
use openxr::Entry;
use openxr::ApplicationInfo;
use openxr::ExtensionSet;
use openxr::Instance;
use openxr::Graphics;

pub fn start_vr(state: &mut State, capability: Uuid) {
    println!("Starting openxr...");
    let app_info = ApplicationInfo {
        application_name: "atlas' uplink",
        application_version: 1,
        engine_name: "uplink",
        engine_version: 1,
    };

    let mut extensions = ExtensionSet::default();
    extensions.khr_vulkan_enable2 = true;

    let layers: &[&str] = Default::default();

    match Entry::load() {
        Ok(entry) => {
            match entry.create_instance(&app_info, &extensions, layers) {
                Ok(instance) => {
                    println!("properties are {:?}", instance.properties());
                }
                Err(e) => {

                }
            }
        }
        Err(e) => {

        }
    }
}
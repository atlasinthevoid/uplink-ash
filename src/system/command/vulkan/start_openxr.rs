use openxr as xr;
use xr::sys;

pub async fn start_openxr(
) -> Result<(xr::Instance, xr::SystemId, xr::EnvironmentBlendMode), sys::Result> {
    #[cfg(feature = "static")]
    let entry = xr::Entry::linked();
    #[cfg(not(feature = "static"))]
    let entry = xr::Entry::load()
        .expect("couldn't find the OpenXR loader; try enabling the \"static\" feature");

    #[cfg(target_os = "android")]
    entry.initialize_android_loader().unwrap();

    // OpenXR will fail to initialize if we ask for an extension that OpenXR can't provide! So we
    // need to check all our extensions before initializing OpenXR with them. Note that even if the
    // extension is present, it's still possible you may not be able to use it. For example: the
    // hand tracking extension may be present, but the hand sensor might not be plugged in or turned
    // on. There are often additional checks that should be made before using certain features!
    let available_extensions = entry.enumerate_extensions().unwrap();

    // If a required extension isn't present, you want to ditch out here! It's possible something
    // like your rendering API might not be provided by the active runtime. APIs like OpenGL don't
    // have universal support.
    assert!(available_extensions.khr_vulkan_enable2);

    // Initialize OpenXR with the extensions we've found!
    let mut enabled_extensions = xr::ExtensionSet::default();
    enabled_extensions.khr_vulkan_enable2 = true;
    #[cfg(target_os = "android")]
    {
        enabled_extensions.khr_android_create_instance = true;
    }

    let xr_instance: xr::Instance;
    match entry.create_instance(
        &xr::ApplicationInfo {
            application_name: "atlas' uplink",
            application_version: 1,
            engine_name: "uplink",
            engine_version: 1,
        },
        &enabled_extensions,
        &[],
    ) {
        Ok(x) => {
            xr_instance = x;
        }
        Err(e) => return Err(e),
    }

    let instance_props = xr_instance.properties().unwrap();
    println!(
        "loaded OpenXR runtime: {} {}",
        instance_props.runtime_name, instance_props.runtime_version
    );

    // Request a form factor from the device (HMD, Handheld, etc.)
    let system = xr_instance
        .system(xr::FormFactor::HEAD_MOUNTED_DISPLAY)
        .unwrap();

    // Check what blend mode is valid for this device (opaque vs transparent displays). We'll just
    // take the first one available!
    let environment_blend_mode = xr_instance
        .enumerate_environment_blend_modes(system, super::VIEW_TYPE)
        .unwrap()[0];

    Ok((xr_instance, system, environment_blend_mode))
}

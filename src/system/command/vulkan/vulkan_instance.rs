use ash::vk::{self, Handle};
use openxr as xr;

pub const COLOR_FORMAT: vk::Format = vk::Format::R8G8B8A8_SRGB;
pub const VIEW_COUNT: u32 = 2;

pub async unsafe fn vulkan_instance(
    xr_instance: &xr::Instance,
    system: &xr::SystemId,
) -> (ash::Instance, ash::Device, u32, ash::vk::PhysicalDevice) {
    // OpenXR wants to ensure apps are using the correct graphics card and Vulkan features and
    // extensions, so the instance and device MUST be set up before Instance::create_session.

    let vk_target_version = vk::make_api_version(0, 1, 1, 0); // Vulkan 1.1 guarantees multiview support
    let vk_target_version_xr = xr::Version::new(1, 1, 0);

    let reqs = xr_instance
        .graphics_requirements::<xr::Vulkan>(*system)
        .unwrap();

    if vk_target_version_xr < reqs.min_api_version_supported
        || vk_target_version_xr.major() > reqs.max_api_version_supported.major()
    {
        panic!(
            "OpenXR runtime requires Vulkan version > {}, < {}.0.0",
            reqs.min_api_version_supported,
            reqs.max_api_version_supported.major() + 1
        );
    }

    let vk_entry = ash::Entry::load().unwrap();

    let vk_app_info = vk::ApplicationInfo::builder()
        .application_version(0)
        .engine_version(0)
        .api_version(vk_target_version);

    let vk_instance = {
        let vk_instance = xr_instance
            .create_vulkan_instance(
                *system,
                std::mem::transmute(vk_entry.static_fn().get_instance_proc_addr),
                &vk::InstanceCreateInfo::builder().application_info(&vk_app_info) as *const _
                    as *const _,
            )
            .expect("XR error creating Vulkan instance")
            .map_err(vk::Result::from_raw)
            .expect("Vulkan error creating Vulkan instance");
        ash::Instance::load(
            vk_entry.static_fn(),
            vk::Instance::from_raw(vk_instance as _),
        )
    };

    let vk_physical_device = vk::PhysicalDevice::from_raw(
        xr_instance
            .vulkan_graphics_device(*system, vk_instance.handle().as_raw() as _)
            .unwrap() as _,
    );

    let vk_device_properties = vk_instance.get_physical_device_properties(vk_physical_device);
    if vk_device_properties.api_version < vk_target_version {
        vk_instance.destroy_instance(None);
        panic!("Vulkan phyiscal device doesn't support version 1.1");
    }

    let queue_family_index = vk_instance
        .get_physical_device_queue_family_properties(vk_physical_device)
        .into_iter()
        .enumerate()
        .find_map(|(queue_family_index, info)| {
            if info.queue_flags.contains(vk::QueueFlags::GRAPHICS) {
                Some(queue_family_index as u32)
            } else {
                None
            }
        })
        .expect("Vulkan device has no graphics queue");

    let vk_device = {
        let vk_device = xr_instance
            .create_vulkan_device(
                *system,
                std::mem::transmute(vk_entry.static_fn().get_instance_proc_addr),
                vk_physical_device.as_raw() as _,
                &vk::DeviceCreateInfo::builder()
                    .queue_create_infos(&[vk::DeviceQueueCreateInfo::builder()
                        .queue_family_index(queue_family_index)
                        .queue_priorities(&[1.0])
                        .build()])
                    .push_next(&mut vk::PhysicalDeviceMultiviewFeatures {
                        multiview: vk::TRUE,
                        ..Default::default()
                    }) as *const _ as *const _,
            )
            .expect("XR error creating Vulkan device")
            .map_err(vk::Result::from_raw)
            .expect("Vulkan error creating Vulkan device");

        ash::Device::load(vk_instance.fp_v1_0(), vk::Device::from_raw(vk_device as _))
    };
    (
        vk_instance,
        vk_device,
        queue_family_index,
        vk_physical_device,
    )
}

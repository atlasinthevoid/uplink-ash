#[derive(Clone, Copy, Debug)]
#[repr(C)]
struct ModelTransform {
    model: Matrix4<f32>,
}

impl ModelTransform {
    fn pose(p: &openxr::Posef) -> Self {
        let pos = Vector3::new(p.position.x, p.position.y, p.position.z);
        let orientation = Quaternion::new(p.orientation.w, p.orientation.x, p.orientation.y, p.orientation.z);
        Self {
            model: Matrix4::from_translation(-pos)
                * Matrix4::from(orientation)
                * Matrix4::from_scale(0.1)
                * Matrix4::from_translation(Vector3::new(-0.5, -0.5, -0.5)),
        }
    }
    fn new() -> Self {
        Self {
            model: Matrix4::from_scale(-1.0),
        }
    }
}
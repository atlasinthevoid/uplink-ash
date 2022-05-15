#[derive(Clone, Copy, Debug)]
#[repr(C)]
struct CameraTransform {
    view_project: Matrix4<f32>,
}

impl CameraTransform {
    fn new(v: &openxr::View) -> Self {
        let orientation = Quaternion::new(v.pose.orientation.w, v.pose.orientation.x, v.pose.orientation.y, v.pose.orientation.z);
        let basis = cgmath::Basis3::from_quaternion(&orientation);

        let (dir, up) = (basis.as_ref().z, basis.as_ref().y);
        let eye = Vector3::new(v.pose.position.x, v.pose.position.y, v.pose.position.x);
        let view = Matrix4::look_to_rh(Point3::new(0.0, 0.0, 0.0) - eye, dir, up);

        use cgmath::Rad;
        let near = 0.01;
        let project = cgmath::Perspective {
            left: Rad(v.fov.angle_left).tan() * near,
            right: Rad(v.fov.angle_right).tan() * near,
            bottom: Rad(v.fov.angle_down).tan() * near,
            top: Rad(v.fov.angle_up).tan() * near,
            near,
            far: 100.0,
        };

        Self {
            view_project: Matrix4::from(project) * view,
        }
    }
}
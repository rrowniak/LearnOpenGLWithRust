use ultraviolet::*;

pub enum CamMovement {
    Forward,
    Backward,
    Left,
    Right,
    Up,
    Down,
}
pub struct Camera {
    // camera Attributes
    pub position: Vec3,
    pub front: Vec3,
    up: Vec3,
    right: Vec3,
    world_up: Vec3,
    // euler Angles
    yaw: f32,
    pitch: f32,
    // camera options
    pub movement_speed: f32,
    pub mouse_sensitivity: f32,
    pub zoom: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self::new()
    }
}

impl Camera {
    pub fn new() -> Self {
        let mut c = Camera {
            position: Vec3::new(0.0, 0.0, 0.0),
            front: Vec3::new(0.0, 0.0, 0.0),
            up: Vec3::default(),
            right: Vec3::default(),
            world_up: Vec3::new(0.0, 1.0, 0.0),
            yaw: -90.0,
            pitch: 0.0,
            movement_speed: 2.5,
            mouse_sensitivity: 0.2,
            zoom: 45.0,
        };
        c.update_camera_vectors();
        c
    }

    pub fn from(pos: Vec3, up: Vec3, yaw: f32, pitch: f32) -> Self {
        let mut cam = Self::new();
        cam.position = pos;
        cam.world_up = up;
        cam.yaw = yaw;
        cam.pitch = pitch;
        cam.update_camera_vectors();
        cam
    }

    pub fn get_view_matrix(&self) -> Mat4 {
        Mat4::look_at(self.position, self.position + self.front, self.up)
    }

    pub fn process_keyboard(&mut self, direction: CamMovement, delta_time: f32) {
        let velocity = self.movement_speed * delta_time;

        match direction {
            CamMovement::Forward => self.position += self.front * velocity,
            CamMovement::Backward => self.position -= self.front * velocity,
            CamMovement::Left => self.position -= self.right * velocity,
            CamMovement::Right => self.position += self.right * velocity,
            CamMovement::Up => self.position += self.up * velocity,
            CamMovement::Down => self.position -= self.up * velocity,
        }
    }

    pub fn process_mouse_movement(&mut self, x_offset: f32, y_offset: f32, constrain_pitch: bool) {
        let x_offset = x_offset * self.mouse_sensitivity;
        let y_offset = y_offset * self.mouse_sensitivity;

        self.yaw += x_offset;
        self.pitch += y_offset;

        if constrain_pitch {
            let limit = 89.0;
            self.pitch = self.pitch.clamp(-limit, limit);
        }

        self.update_camera_vectors();
    }

    pub fn process_mouse_scroll(&mut self, y_offset: f32) {
        self.zoom -= y_offset;
        self.zoom = self.zoom.clamp(1.0, 45.0);
    }

    fn update_camera_vectors(&mut self) {
        self.front.x = self.yaw.to_radians().cos() * self.pitch.to_radians().cos();
        self.front.y = self.pitch.to_radians().sin();
        self.front.z = self.yaw.to_radians().sin() * self.pitch.to_radians().cos();
        self.front.normalize();

        self.right = self.front.clone().cross(self.world_up);
        self.right.normalize();

        self.up = self.right.clone().cross(self.front);
        self.up.normalize();
    }
}

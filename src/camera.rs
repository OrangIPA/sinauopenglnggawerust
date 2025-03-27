use gl::types::GLboolean;
use nalgebra_glm as glm;

const YAW: f32 = -90.0;
const PITCH: f32 = 0.0;
const SPEED: f32 = 2.5;
const SENSITIVITY: f32 = 0.1;
const ZOOM: f32 = 45.0;

#[derive(Clone, Copy)]
pub enum CameraMovement {
    Forward,
    Backward,
    Left,
    Right,
}

pub struct Camera {
    pub pos: glm::Vec3,
    pub front: glm::Vec3,
    pub up: glm::Vec3,
    pub right: glm::Vec3,
    pub world_up: glm::Vec3,

    pub yaw: f32,
    pub pitch: f32,
    pub movement_speed: f32,
    pub mouse_sensitivity: f32,
    pub zoom: f32,
}

impl Camera {
    pub fn new(
        pos: Option<glm::Vec3>,
        up: Option<glm::Vec3>,
        yaw: Option<f32>,
        pitch: Option<f32>,
    ) -> Self {
        let pos = pos.unwrap_or(glm::Vec3::zeros());
        let up = up.unwrap_or(glm::vec3(0.0, 1.0, 0.0));
        let yaw = yaw.unwrap_or(YAW);
        let pitch = pitch.unwrap_or(PITCH);

        let mut ret = Self {
            front: glm::vec3(0.0, 0.0, -1.0),
            movement_speed: SPEED,
            mouse_sensitivity: SENSITIVITY,
            zoom: ZOOM,
            pos,
            up,
            world_up: up,
            yaw,
            pitch,
            right: glm::Vec3::zeros(),
        };
        ret.update_camera_vectors();
        ret
    }

    pub fn get_view_matrix(&self) -> glm::Mat4 {
        glm::look_at(&self.pos, &(&self.pos + &self.front), &self.up)
    }

    pub fn process_keyboard(&mut self, direction: CameraMovement, delta_time: f32) {
        let velocity = self.movement_speed * delta_time;

        match direction {
            CameraMovement::Forward => self.pos += self.front * velocity,
            CameraMovement::Backward => self.pos -= self.front * velocity,
            CameraMovement::Left => self.pos -= self.right * velocity,
            CameraMovement::Right => self.pos += self.right * velocity,
        };
    }

    pub fn process_mouse_movement(&mut self, x_offset: f32, y_offset: f32, constrain_pitch: Option<GLboolean>) {
        let x_offset = x_offset * self.mouse_sensitivity;
        let y_offset = y_offset * self.mouse_sensitivity;

        self.yaw += x_offset;
        self.pitch += y_offset;

        if constrain_pitch.unwrap_or(gl::TRUE) != 0 {
            self.pitch = self.pitch.clamp(-89.0, 89.0);
        }

        self.update_camera_vectors();
    }

    pub fn process_mouse_scroll(&mut self, y_offset: f32) {
        self.zoom -= y_offset;
        self.zoom = self.zoom.clamp(1.0, 45.);
    }

    fn update_camera_vectors(&mut self) {
        self.front = glm::vec3(
            self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
            self.pitch.to_radians().sin(),
            self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
        )
        .normalize();

        self.right = self.front.cross(&self.world_up).normalize();
        self.up = self.right.cross(&self.front).normalize();
    }
}

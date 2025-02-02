use glam::{DMat4, DQuat, DVec2, DVec3, DVec4};
use math::decimal_vector_3d::DecimalVector3d;

#[derive(Debug, Clone)]
pub struct FrustumCone {
    pub top_left: DVec3,
    pub bottom_left: DVec3,
    pub top_right: DVec3,
    pub bottom_right: DVec3,
}

#[derive(Debug, Clone)]
pub struct CameraDirections {
    pub up: DVec3,
    pub down: DVec3,
    pub left: DVec3,
    pub right: DVec3,
    pub forwards: DVec3,
    pub backwards: DVec3,
}

#[derive(Debug, Clone)]
pub struct Camera {
    pub projection_matrix: DMat4,
    pub view_matrix: DMat4,
    pub position: DecimalVector3d,
    pub orientation: DQuat,
    pub frustum_cone: FrustumCone,
    pub directions: CameraDirections,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            projection_matrix: DMat4::IDENTITY.clone(),
            view_matrix: DMat4::IDENTITY.clone(),
            position: DecimalVector3d::zero(),
            orientation: DQuat::IDENTITY.clone(),
            frustum_cone: FrustumCone {
                bottom_left: DVec3::new(0.0, 0.0, -1.0),
                bottom_right: DVec3::new(0.0, 0.0, -1.0),
                top_left: DVec3::new(0.0, 0.0, -1.0),
                top_right: DVec3::new(0.0, 0.0, -1.0),
            },
            directions: CameraDirections {
                up: DVec3::new(0.0, 0.0, -1.0),
                down: DVec3::new(0.0, 0.0, -1.0),
                left: DVec3::new(0.0, 0.0, -1.0),
                right: DVec3::new(0.0, 0.0, -1.0),
                forwards: DVec3::new(0.0, 0.0, -1.0),
                backwards: DVec3::new(0.0, 0.0, -1.0),
            },
        }
    }

    pub fn set_perspective(&mut self, fov: f64, aspect: f64, near: f64, far: f64) {
        self.projection_matrix = DMat4::perspective_rh_gl(fov, aspect, near, far);
    }

    pub fn set_orthographic(
        &mut self,
        left: f64,
        right: f64,
        top: f64,
        bottom: f64,
        near: f64,
        far: f64,
    ) {
        self.projection_matrix = DMat4::orthographic_rh_gl(left, right, bottom, top, near, far);
    }

    pub fn update(&mut self) {
        self.view_matrix = DMat4::from_quat(self.orientation);
        self.frustum_cone = self.get_frustum_cone();
        self.directions = self.get_directions();
    }

    fn get_frustum_cone(&self) -> FrustumCone {
        let inverse = self
            .projection_matrix
            .clone()
            .mul_mat4(&self.view_matrix)
            .inverse();
        FrustumCone {
            bottom_left: Self::get_dir(DVec2::new(-1.0, -1.0), inverse),
            bottom_right: Self::get_dir(DVec2::new(1.0, -1.0), inverse),
            top_left: Self::get_dir(DVec2::new(-1.0, 1.0), inverse),
            top_right: Self::get_dir(DVec2::new(1.0, 1.0), inverse),
        }
    }

    fn get_dir(uv: DVec2, inverse_proj_view_matrix: DMat4) -> DVec3 {
        let mut clip = DVec4::new(uv.x, uv.y, 0.1, 1.0);
        clip = inverse_proj_view_matrix * clip;
        (DVec3::new(clip.x, clip.y, clip.z) / clip.w).normalize()
    }

    fn get_directions(&self) -> CameraDirections {
        let inverse = self.orientation.clone().inverse();
        CameraDirections {
            forwards: inverse * DVec3::new(0.0, 0.0, -1.0),
            backwards: inverse * DVec3::new(0.0, 0.0, 1.0),
            up: inverse * DVec3::new(0.0, 1.0, 0.0),
            down: inverse * DVec3::new(0.0, -1.0, 0.0),
            left: inverse * DVec3::new(-1.0, 0.0, 0.0),
            right: inverse * DVec3::new(1.0, 0.0, 0.0),
        }
    }
}

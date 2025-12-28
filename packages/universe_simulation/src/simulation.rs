use crate::body_definitions::{BodyCelestialBodyDefinition, BodyMotion};
use dashu_float::DBig;
use dashu_float::ops::SquareRoot;
use math::decimal_matrix_3d::DecimalMatrix3d;
use math::decimal_vector_3d::DecimalVector3d;
use math::sin_cos::PIMUL2;
use renderer_common::errors::RenderingError;
use renderer_common::resolution_config::ResolutionConfig;
use std::str::FromStr;
use std::sync::LazyLock;

static G_CONSTANT: LazyLock<DBig> = LazyLock::new(|| DBig::from_str("0.0000000000667408").unwrap());

#[derive(Debug, Clone)]
pub struct SimulatedBody {
    id: i32,
    pub body: BodyCelestialBodyDefinition,
    pub position: DecimalVector3d,
    pub velocity: DecimalVector3d,
    pub orientation: DecimalMatrix3d,
    parent: Option<i32>, // -1 means no
    satellites: Vec<i32>,
}

pub struct Simulation {
    pub bodies: Vec<SimulatedBody>,
    id_counter: i32,
}

impl Default for Simulation {
    fn default() -> Self {
        Self::new()
    }
}

impl Simulation {
    pub fn new() -> Self {
        Simulation {
            bodies: vec![],
            id_counter: 0,
        }
    }

    pub fn add_hierarchy(
        &mut self,
        config: &ResolutionConfig,
        body: &BodyCelestialBodyDefinition,
        parent: Option<i32>,
    ) -> Result<i32, RenderingError> {
        let new_id = self.id_counter;
        self.id_counter += 1;

        let mut simulated_body = SimulatedBody {
            id: new_id,
            parent,
            satellites: vec![],
            body: body.clone(),
            position: DecimalVector3d::zero(),
            velocity: DecimalVector3d::zero(),
            orientation: DecimalMatrix3d::identity(),
        };
        simulated_body.body.dynamics.rotation_axis.normalize();
        match &mut simulated_body.body.dynamics.motion {
            BodyMotion::Static(_) => (),
            BodyMotion::Orbiting(data) => data.orbit_plane_normal.normalize(),
        }
        for i in 0..body.dynamics.satellites.len() {
            simulated_body.satellites.push(self.add_hierarchy(
                config,
                &body.dynamics.satellites[i],
                Some(new_id),
            )?);
        }
        self.bodies.push(simulated_body);
        Ok(new_id)
    }

    fn get_body_by_name(&self, name: &str) -> Option<&SimulatedBody> {
        for i in 0..self.bodies.len() {
            if self.bodies[i].body.name == name {
                return Some(&self.bodies[i]);
            }
        }

        None
    }

    fn get_body_by_id(&self, id: i32) -> Option<&SimulatedBody> {
        for i in 0..self.bodies.len() {
            if self.bodies[i].id == id {
                return Some(&self.bodies[i]);
            }
        }

        None
    }

    fn get_mut_body_by_id(&mut self, id: i32) -> Option<&mut SimulatedBody> {
        for i in 0..self.bodies.len() {
            if self.bodies[i].id == id {
                return Some(&mut self.bodies[i]);
            }
        }

        None
    }

    fn resolve_hierarchy_up(&self, body: &SimulatedBody) -> Vec<&SimulatedBody> {
        /* how this will look like for example for the moon,
          moon gets into this function, we don't want to add it
          its parent is earth, it gets found, is added to the moon-result
          then earth gets into this function,
          its parent is the sun, it gets added to the earth-result
          then sun gets into this function, but doesn't have a parent, so sun-result is []
          then sun result [] gets appended to earth result [sun] results in [sun]
          then earth result [sun] gets appended to moon result [earth] results in [earth, sun]
          so it goes upward from the body
        */
        let mut result: Vec<&SimulatedBody> = vec![];
        match body.parent {
            None => (),
            Some(parent) => {
                match self.get_body_by_id(parent) {
                    None => (),
                    Some(parent) => {
                        result.push(parent);
                        let mut sub_result = self.resolve_hierarchy_up(parent);
                        result.append(&mut sub_result);
                    }
                };
            }
        }
        result
    }

    fn resolve_hierarchy_down(&self, body: &SimulatedBody) -> Vec<&SimulatedBody> {
        /* how this will look like for example for the sun,
        sun gets into this function, its satellites are iterated, lets simplify to Venus, Earth, and Mars
        to sun result first added is [Venus]
        then venus gets into this function, but has no satellites, so nothing gets added
        then to sun result [Earth] is added
        then earth gets into this function, results in [Moon], this is appended to
        then [Mars] is added
        so in final it will look like [Venus, Earth, Moon, Mars] it's not optimal,
        but it's good for this purpose here
        */
        let mut result: Vec<&SimulatedBody> = vec![];
        for i in 0..body.satellites.len() {
            match self.get_body_by_id(body.satellites[i]) {
                None => (),
                Some(sat) => {
                    result.push(sat);
                    let mut sub_result = self.resolve_hierarchy_down(sat);
                    result.append(&mut sub_result);
                }
            };
        }
        result
    }

    fn get_body_position(&self, time: &DBig, body: &SimulatedBody) -> DecimalVector3d {
        match body.clone().body.dynamics.motion {
            BodyMotion::Static(dynamics) => dynamics.position,
            BodyMotion::Orbiting(dynamics) => {
                let parent = self.get_body_by_id(body.parent.unwrap()).unwrap(); // panic if not fulfilled
                let orbit_progression = (time / dynamics.orbit_period).fract();
                let angle = &*PIMUL2 * orbit_progression;
                let rotation_matrix =
                    DecimalMatrix3d::axis_angle(&dynamics.orbit_plane_normal, angle);
                rotation_matrix.apply(&DecimalVector3d::new(
                    dynamics.orbit_radius,
                    DBig::ZERO,
                    DBig::ZERO,
                )) + &parent.position
            }
        }
    }

    fn get_body_orientation(time: &DBig, body: &SimulatedBody) -> DecimalMatrix3d {
        let rotation_progression = (time / body.body.dynamics.rotation_period).fract();
        let angle = &*PIMUL2 * rotation_progression;
        DecimalMatrix3d::axis_angle(&body.body.dynamics.rotation_axis, angle)
    }

    pub fn update(&mut self, camera_position: &DecimalVector3d, time: &DBig) {
        let mut schedule: Vec<i32> = vec![];
        for i in 0..self.bodies.len() {
            let body = &self.bodies[i];
            match body.body.dynamics.motion {
                BodyMotion::Static(_) => {
                    let hierarchy = self.resolve_hierarchy_down(body);
                    schedule.push(body.id);
                    for body in hierarchy {
                        schedule.push(body.id);
                    }
                }
                BodyMotion::Orbiting(_) => (),
            }
        }
        for item in schedule {
            let body_immutable = self.get_body_by_id(item).unwrap();

            let position = self.get_body_position(time, body_immutable);
            let pos_second_ago = self.get_body_position(&(time - DBig::ONE), body_immutable);
            let velocity = &position - pos_second_ago;
            let orientation = Self::get_body_orientation(time, body_immutable);

            let body = self.get_mut_body_by_id(item).unwrap();
            body.position = position;
            body.velocity = velocity;
            body.orientation = orientation;
        }
        for i in 0..self.bodies.len() {
            let body = &self.bodies[i];
            let _closest_static = match &body.body.dynamics.motion {
                BodyMotion::Static(_) => body,
                BodyMotion::Orbiting(_) => self.find_closest_static(&body.position),
            };
        }

        self.bodies.sort_by(|a, b| {
            a.position
                .distance_to(camera_position)
                .partial_cmp(&b.position.distance_to(camera_position))
                .unwrap()
        });
    }

    pub fn get_body(&self, body_name: &str) -> &SimulatedBody {
        self.get_body_by_name(body_name).unwrap()
    }

    pub fn get_surface_velocity(
        &self,
        body_name: &str,
        relative_point: &DecimalVector3d,
    ) -> DecimalVector3d {
        let body = self.get_body_by_name(body_name).unwrap();
        let axis = &body.body.dynamics.rotation_axis;
        let angular_body_vel = &*PIMUL2 / body.body.dynamics.rotation_period;
        let angular_velocity_vector = axis * angular_body_vel;
        angular_velocity_vector.cross(relative_point)
    }

    pub fn find_closest_static(&self, point: &DecimalVector3d) -> &SimulatedBody {
        let mut min_distance = DBig::INFINITY;
        let mut closest = &self.bodies[0];
        for i in 0..self.bodies.len() {
            match self.bodies[i].body.dynamics.motion {
                BodyMotion::Static(_) => {
                    let distance = self.bodies[i].position.distance_to(point);
                    if distance < min_distance {
                        closest = &self.bodies[i];
                        min_distance = distance;
                    }
                }
                BodyMotion::Orbiting(_) => (),
            }
        }
        closest
    }

    pub fn find_closest_hierarchy(&self, point: &DecimalVector3d) -> Vec<&SimulatedBody> {
        let closest_static = self.find_closest_static(point);
        let mut hierarchy = self.resolve_hierarchy_down(closest_static);
        hierarchy.splice(..0, vec![closest_static]);
        hierarchy
    }

    pub fn find_closest_body(&self, point: &DecimalVector3d) -> &SimulatedBody {
        let closest_static = self.find_closest_static(point);
        let down_hierarchy = self.resolve_hierarchy_down(closest_static);
        if down_hierarchy.is_empty() {
            return closest_static;
        }
        let mut min_distance = down_hierarchy[0].position.distance_to(point);
        let mut closest = &down_hierarchy[0];
        for item in &down_hierarchy {
            let distance = item.position.distance_to(point);
            if distance < min_distance {
                closest = item;
                min_distance = distance;
            }
        }
        closest
    }

    pub fn calculate_gravity_flux(&self, point: &DecimalVector3d) -> DecimalVector3d {
        let closest_static = self.find_closest_static(point);
        let mut flux = DecimalVector3d::zero();
        let mut hierarchy = self.resolve_hierarchy_down(closest_static);
        hierarchy.push(closest_static);

        for item in hierarchy {
            let body = item;
            let relative = &body.position - point;
            let length_squared = relative.length_squared();
            let length = length_squared.sqrt();
            let strength = &*G_CONSTANT * &body.body.dynamics.mass / length_squared;
            flux += (relative * (&DBig::ONE / length * strength));
        }
        flux
    }
}

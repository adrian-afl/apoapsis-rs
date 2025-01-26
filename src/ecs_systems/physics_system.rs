use crate::celestial_rendering::errors::ECSError;
use crate::celestial_rendering::scene::mesh::Mesh;
use crate::component_types;
use crate::core::game_state::GameState;
use crate::ecs::component_trait::component_type;
use crate::ecs::ecs_world::ECSWorld;
use crate::ecs::entity::Entity;
use crate::ecs::system_trait::SystemTrait;
use crate::ecs_components::common::transform_component::TransformComponent;
use crate::ecs_components::physics::real_physics_component::RealPhysicsComponent;
use crate::ecs_components::physics::simple_physics_component::SimplePhysicsComponent;
use crate::ecs_components::player::is_player_component::IsPlayerComponent;
use crate::math::decimal_vector_3d::DecimalVector3d;
use crate::math::sin_cos::f64_to_dbig;
use crate::simulation::real_physics_system::RealPhysicsSystem;
use crate::simulation::simulation::Simulation;
use dashu_float::DBig;
use glam::{DQuat, DVec3};
use rapier3d_f64::prelude::RigidBodyHandle;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

struct SimulatedBody {
    rigid_body: RigidBodyHandle,
}

struct PlayerTemporaryData {
    position: DecimalVector3d,
    linear_velocity: DecimalVector3d,
}

pub struct PhysicsSystem {
    universe_simulation: Arc<Mutex<Simulation>>,
    currently_simulated_bodies: Arc<Mutex<HashMap<u64, SimulatedBody>>>,
    real_physics_system: Arc<Mutex<RealPhysicsSystem>>,
    real_simulation_cutoff: f64,
    player_temporary_data: PlayerTemporaryData,
}

impl PhysicsSystem {
    pub fn new(universe_simulation: Arc<Mutex<Simulation>>) -> Self {
        Self {
            universe_simulation,
            real_physics_system: Arc::new(Mutex::from(RealPhysicsSystem::new())),
            currently_simulated_bodies: Arc::new(Mutex::from(HashMap::new())),
            real_simulation_cutoff: 100.0,
            player_temporary_data: PlayerTemporaryData {
                position: DecimalVector3d::zero(),
                linear_velocity: DecimalVector3d::zero(),
            },
        }
    }

    fn phase0(&mut self, ecs: Arc<Mutex<ECSWorld>>, delta_time: f64) -> Result<(), ECSError> {
        let decimal_delta_time = f64_to_dbig(delta_time);
        let decimal_half_delta_time = f64_to_dbig(delta_time * 0.5);

        let mut ecs = ecs.lock().unwrap();
        let player = ecs.find_first_by_components(component_types!(
            IsPlayerComponent,
            SimplePhysicsComponent,
            TransformComponent
        ));
        {
            // scope to now screw up with shadowing
            let player = match player {
                Ok(player) => player,
                Err(err) => {
                    println!("Player entity not found, Relativity cannot continue");
                    return Err(err);
                }
            };

            let transform = player.get_first_component::<TransformComponent>().unwrap();
            let simple_physics = player
                .get_first_component::<SimplePhysicsComponent>()
                .unwrap();

            self.player_temporary_data
                .position
                .assign(&transform.position);
            self.player_temporary_data
                .linear_velocity
                .assign(&simple_physics.linear_velocity);
        }

        ecs.process_all_by_components_mut(
            component_types!(SimplePhysicsComponent, TransformComponent),
            |entity| {
                let mut transform = entity
                    .get_first_component::<TransformComponent>()
                    .unwrap()
                    .clone();

                let mut simple_physics = entity
                    .get_first_component::<SimplePhysicsComponent>()
                    .unwrap()
                    .clone();

                let real_physics = entity.get_first_component::<RealPhysicsComponent>();

                if real_physics.is_some() {
                    let mut real_physics = real_physics.unwrap().clone();

                    *entity
                        .get_first_component_mut::<RealPhysicsComponent>()
                        .unwrap() = real_physics;
                } else {
                    self.update_simple_physics(
                        &mut simple_physics,
                        &mut transform,
                        delta_time,
                        &decimal_delta_time,
                        &decimal_half_delta_time,
                    )
                }

                *entity
                    .get_first_component_mut::<TransformComponent>()
                    .unwrap() = transform;

                *entity
                    .get_first_component_mut::<SimplePhysicsComponent>()
                    .unwrap() = simple_physics;
            },
        );

        Ok(())
    }

    fn update_simple_physics(
        &self,
        simple_physics: &mut SimplePhysicsComponent,
        transform: &mut TransformComponent,
        delta_time: f64,
        decimal_delta_time: &DBig,
        decimal_half_delta_time: &DBig,
    ) {
        transform.position =
            &transform.position + &simple_physics.linear_velocity * decimal_half_delta_time;

        if simple_physics.mass > DBig::ZERO {
            let gravity_impulse = self
                .universe_simulation
                .lock()
                .unwrap()
                .calculate_gravity_flux(&transform.position)
                * decimal_delta_time;

            simple_physics.linear_velocity = &simple_physics.linear_velocity + &gravity_impulse;
        }

        transform.position =
            &transform.position + &simple_physics.linear_velocity * decimal_half_delta_time;

        let angular_velocity_dvec3 = simple_physics.angular_velocity.to_dvec3();
        let rotation_approximation = DQuat::from_axis_angle(
            DVec3::new(1.0, 0.0, 0.0),
            angular_velocity_dvec3.x * delta_time,
        ) * DQuat::from_axis_angle(
            DVec3::new(0.0, 1.0, 0.0),
            angular_velocity_dvec3.y * delta_time,
        ) * DQuat::from_axis_angle(
            DVec3::new(0.0, 0.0, 1.0),
            angular_velocity_dvec3.z * delta_time,
        );

        transform.orientation *= rotation_approximation;
    }

    fn handle_start_stop(
        &mut self,
        transform: &TransformComponent,
        simple_physics: &SimplePhysicsComponent,
        real_physics: &RealPhysicsComponent,
    ) -> Option<u64> {
    }
}

impl SystemTrait for PhysicsSystem {
    fn update(&mut self, game_state: Arc<Mutex<GameState>>, ecs: Arc<Mutex<ECSWorld>>) {
        let ecs = ecs.lock().unwrap();
        ecs.process_all_by_components(
            component_types!(SimplePhysicsComponent, TransformComponent),
            |entity| {},
        );
    }
}

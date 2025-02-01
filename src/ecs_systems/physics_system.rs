use crate::core::game_state::GameState;
use crate::ecs::component_trait::Components;
use crate::ecs::ecs_world::ECSWorld;
use crate::ecs::system_trait::SystemTrait;
use crate::ecs_components::common::transform_component::TransformComponent;
use crate::ecs_components::physics::real_physics_component::RealPhysicsComponent;
use crate::ecs_components::physics::simple_physics_component::SimplePhysicsComponent;
use crate::math::decimal_vector_3d::DecimalVector3d;
use crate::math::sin_cos::f64_to_dbig;
use crate::simulation::real_physics_system::{RealPhysicsSystem, SetRealPhysicsBodyKinematics};
use crate::simulation::simulation::Simulation;
use dashu_float::DBig;
use glam::{DQuat, DVec3};
use rapier3d_f64::prelude::{RigidBodyBuilder, RigidBodyHandle};
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};

struct SimulatedBody {
    rigid_body: RigidBodyHandle,
    phase_1_relative_position: DVec3,
    phase_1_relative_linear_velocity: DVec3,
}

struct PlayerTemporaryData {
    position: DecimalVector3d,
    linear_velocity: DecimalVector3d,
}

pub struct PhysicsSystem {
    universe_simulation: Arc<RwLock<Simulation>>,
    currently_simulated_bodies: RwLock<HashMap<u64, SimulatedBody>>,
    real_physics_system: RwLock<RealPhysicsSystem>,
    real_simulation_cutoff: f64,
    player_temporary_data: PlayerTemporaryData,
}

impl PhysicsSystem {
    pub fn new(universe_simulation: Arc<RwLock<Simulation>>) -> Self {
        Self {
            universe_simulation,
            real_physics_system: RwLock::new(RealPhysicsSystem::new()),
            currently_simulated_bodies: RwLock::new(HashMap::new()),
            real_simulation_cutoff: 100.0,
            player_temporary_data: PlayerTemporaryData {
                position: DecimalVector3d::zero(),
                linear_velocity: DecimalVector3d::zero(),
            },
        }
    }

    fn phase0(&mut self, ecs: Arc<Mutex<ECSWorld>>) {
        println!("PhysicsSystem / phase0");

        let ecs = ecs.lock().unwrap();
        let player = ecs.find_first_by_components(&[
            &Components::IsPlayer,
            &Components::SimplePhysics,
            &Components::Transform,
        ]);

        if let Some(player) = player {
            let transform = player.components.transform.as_ref().unwrap();
            let simple_physics = player.components.simple_physics.as_ref().unwrap();

            self.player_temporary_data
                .position
                .assign(&transform.position);
            self.player_temporary_data
                .linear_velocity
                .assign(&simple_physics.linear_velocity);
        } else {
            println!("Player entity not found, Relativity can behave weird");
        }
    }

    fn phase1(&mut self, ecs: Arc<Mutex<ECSWorld>>, delta_time: f64) {
        println!("PhysicsSystem / phase1");

        let decimal_delta_time = f64_to_dbig(delta_time);
        let decimal_half_delta_time = f64_to_dbig(delta_time * 0.5);

        let mut ecs = ecs.lock().unwrap();

        ecs.parallel_process_all_by_components_mut(
            &[&Components::SimplePhysics, &Components::Transform],
            |entity| {
                let transform = entity.components.transform.as_mut().unwrap();
                let simple_physics = entity.components.simple_physics.as_mut().unwrap();
                let real_physics = entity.components.real_physics.as_ref();

                if real_physics.is_some() {
                    let real_physics = real_physics.unwrap();

                    let handle_or_none = self.handle_real_physics_simulation_start_stop(
                        transform,
                        simple_physics,
                        real_physics,
                    );

                    match handle_or_none {
                        None => self.update_simple_physics(
                            simple_physics,
                            transform,
                            delta_time,
                            &decimal_delta_time,
                            &decimal_half_delta_time,
                        ),
                        Some(id) => {
                            let mut relative_position = (&transform.position
                                - &self.player_temporary_data.position)
                                .to_dvec3();
                            let relative_linear_velocity = (&simple_physics.linear_velocity
                                - &self.player_temporary_data.linear_velocity)
                                .to_dvec3();

                            {
                                let mut map = self.currently_simulated_bodies.try_write().unwrap();
                                let simulated_object = map.get_mut(&id).unwrap();
                                simulated_object.phase_1_relative_position = relative_position;
                                simulated_object.phase_1_relative_linear_velocity =
                                    relative_linear_velocity;
                            } // unlocks

                            if simple_physics.mass > DBig::ZERO {
                                let mut current_linear_velocity =
                                    DecimalVector3d::from_dvec3(relative_position);

                                let universe_simulation =
                                    self.universe_simulation.try_read().unwrap();
                                let gravity_impulse = universe_simulation
                                    .calculate_gravity_flux(&transform.position)
                                    * &decimal_delta_time;

                                current_linear_velocity =
                                    &current_linear_velocity + &gravity_impulse;

                                relative_position = current_linear_velocity.to_dvec3()
                            }

                            let map = self.currently_simulated_bodies.try_read().unwrap();
                            let simulated_object = map.get(&id).unwrap();
                            let mut real_physics_system =
                                self.real_physics_system.try_write().unwrap();
                            real_physics_system
                                .set_body_kinematics(
                                    simulated_object.rigid_body,
                                    SetRealPhysicsBodyKinematics {
                                        linear_velocity: Some(relative_linear_velocity),
                                        angular_velocity: None,
                                        position: Some(relative_position),
                                        orientation: None,
                                        wake_up: true,
                                    },
                                )
                                .unwrap();

                            // this is suspicious
                            transform.position = &transform.position
                                + &simple_physics.linear_velocity * &decimal_delta_time;
                        }
                    }
                } else {
                    self.update_simple_physics(
                        simple_physics,
                        transform,
                        delta_time,
                        &decimal_delta_time,
                        &decimal_half_delta_time,
                    )
                }
            },
        );
    }

    fn phase2(&mut self, ecs: Arc<Mutex<ECSWorld>>) {
        println!("PhysicsSystem / phase2");

        let mut ecs = ecs.lock().unwrap();

        ecs.parallel_process_all_by_components_mut(
            &[&Components::SimplePhysics, &Components::Transform],
            |entity| {
                let real_physics = entity.components.real_physics.as_ref();
                if real_physics.is_some() {
                    let real_physics = real_physics.unwrap();

                    let map = self.currently_simulated_bodies.try_read().unwrap();
                    let simulated = map.get(&real_physics.id);

                    if simulated.is_some() {
                        let simulated = simulated.unwrap();

                        let real_physics_system = self.real_physics_system.try_read().unwrap();
                        let kinematics = real_physics_system
                            .get_body_kinematics(simulated.rigid_body)
                            .unwrap();

                        let transform = entity.components.transform.as_mut().unwrap();
                        let simple_physics = entity.components.simple_physics.as_mut().unwrap();

                        let diff_relative_position =
                            kinematics.position - simulated.phase_1_relative_position;
                        let diff_relative_linear_velocity =
                            kinematics.linear_velocity - simulated.phase_1_relative_linear_velocity;

                        transform.position = &transform.position
                            + DecimalVector3d::from_dvec3(diff_relative_position);
                        transform.orientation = kinematics.orientation;

                        simple_physics.linear_velocity = &simple_physics.linear_velocity
                            + DecimalVector3d::from_dvec3(diff_relative_linear_velocity);
                        simple_physics.angular_velocity = kinematics.angular_velocity;
                    }
                }
            },
        );
    }

    fn update_simple_physics(
        &self,
        simple_physics: &mut SimplePhysicsComponent,
        transform: &mut TransformComponent,
        delta_time: f64,
        decimal_delta_time: &DBig,
        decimal_half_delta_time: &DBig,
    ) {
        let universe_simulation = self.universe_simulation.try_read().unwrap();
        transform.position =
            &transform.position + &simple_physics.linear_velocity * decimal_half_delta_time;

        if simple_physics.mass > DBig::ZERO {
            let gravity_impulse = universe_simulation.calculate_gravity_flux(&transform.position)
                * decimal_delta_time;

            simple_physics.linear_velocity = &simple_physics.linear_velocity + &gravity_impulse;
        }

        transform.position =
            &transform.position + &simple_physics.linear_velocity * decimal_half_delta_time;

        let rotation_approximation = DQuat::from_axis_angle(
            DVec3::new(1.0, 0.0, 0.0),
            simple_physics.angular_velocity.x * delta_time,
        ) * DQuat::from_axis_angle(
            DVec3::new(0.0, 1.0, 0.0),
            simple_physics.angular_velocity.y * delta_time,
        ) * DQuat::from_axis_angle(
            DVec3::new(0.0, 0.0, 1.0),
            simple_physics.angular_velocity.z * delta_time,
        );

        transform.orientation *= rotation_approximation;
    }

    fn handle_real_physics_simulation_start_stop(
        &self,
        transform: &TransformComponent,
        simple_physics: &SimplePhysicsComponent,
        real_physics: &RealPhysicsComponent,
    ) -> Option<u64> {
        let relative_position =
            (&transform.position - &self.player_temporary_data.position).to_dvec3();

        let should_simulate = relative_position.length() < self.real_simulation_cutoff;

        let mut exists = self
            .currently_simulated_bodies
            .try_read()
            .unwrap()
            .contains_key(&real_physics.id);

        if !should_simulate && exists {
            let mut real_physics_system = self.real_physics_system.try_write().unwrap();
            let mut currently_simulated_bodies =
                self.currently_simulated_bodies.try_write().unwrap();
            // unload
            println!("PSY UNLOAD {}", real_physics.id);
            Self::stop_real_physics_sim(
                &mut real_physics_system,
                &mut currently_simulated_bodies,
                real_physics,
            );
            exists = false;
        } else if should_simulate && !exists {
            let mut real_physics_system = self.real_physics_system.try_write().unwrap();
            let mut currently_simulated_bodies =
                self.currently_simulated_bodies.try_write().unwrap();
            // load
            println!("PSY LOAD {}", real_physics.id);
            Self::start_real_physics_sim(
                &mut real_physics_system,
                &mut currently_simulated_bodies,
                transform,
                simple_physics,
                real_physics,
            );
            exists = true;
        }

        if exists {
            Some(real_physics.id)
        } else {
            None
        }
    }

    fn start_real_physics_sim(
        real_physics_system: &mut RealPhysicsSystem,
        currently_simulated_bodies: &mut HashMap<u64, SimulatedBody>,
        transform: &TransformComponent,
        simple_physics: &SimplePhysicsComponent,
        real_physics: &RealPhysicsComponent,
    ) {
        let rigid_body_builder =
            RigidBodyBuilder::dynamic().additional_mass(simple_physics.mass.to_f64().unwrap());
        let body_collider_tuple = real_physics_system.add_body_with_collider(
            rigid_body_builder.build(),
            real_physics.build_collider().build(),
        );

        let simulated = SimulatedBody {
            rigid_body: body_collider_tuple.0,
            phase_1_relative_position: DVec3::new(0.0, 0.0, 0.0),
            phase_1_relative_linear_velocity: DVec3::new(0.0, 0.0, 0.0),
        };

        real_physics_system
            .set_body_kinematics(
                body_collider_tuple.0,
                SetRealPhysicsBodyKinematics {
                    position: None,
                    orientation: Some(transform.orientation),
                    angular_velocity: Some(simple_physics.angular_velocity),
                    linear_velocity: None,
                    wake_up: false,
                },
            )
            .unwrap();

        currently_simulated_bodies.insert(real_physics.id, simulated);
    }

    fn stop_real_physics_sim(
        real_physics_system: &mut RealPhysicsSystem,
        currently_simulated_bodies: &mut HashMap<u64, SimulatedBody>,
        real_physics: &RealPhysicsComponent,
    ) {
        let simulated_body = currently_simulated_bodies.get(&real_physics.id).unwrap();

        real_physics_system.remove_body(simulated_body.rigid_body);

        currently_simulated_bodies.remove(&real_physics.id);
    }
}

impl SystemTrait for PhysicsSystem {
    fn update(&mut self, game_state: Arc<Mutex<GameState>>, ecs: Arc<Mutex<ECSWorld>>) {
        println!("PhysicsSystem / update");

        let delta_time = game_state.lock().unwrap().delta_time;

        self.phase0(ecs.clone());
        self.phase1(ecs.clone(), delta_time);

        self.real_physics_system
            .try_write()
            .unwrap()
            .step(delta_time);

        self.phase2(ecs.clone());
    }
}

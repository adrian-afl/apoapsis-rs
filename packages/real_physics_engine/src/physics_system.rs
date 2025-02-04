use crate::build_collider::build_collider;
use crate::real_physics_system::{RealPhysicsSystem, SetRealPhysicsBodyKinematics};
use dashu_float::DBig;
use ecs::component_trait::Components;
use ecs::components::common::transform_component::TransformComponent;
use ecs::components::physics::real_physics_component::RealPhysicsComponent;
use ecs::components::physics::simple_physics_component::SimplePhysicsComponent;
use ecs::ecs_world::ECSWorld;
use glam::{DQuat, DVec3};
use math::decimal_vector_3d::DecimalVector3d;
use math::sin_cos::f64_to_dbig;
use rapier3d_f64::prelude::{RigidBodyBuilder, RigidBodyHandle};
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use std::collections::HashMap;
use std::sync::{Mutex, RwLock};
use universe_simulation::simulation::Simulation;

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
    currently_simulated_bodies: RwLock<HashMap<u64, SimulatedBody>>,
    real_physics_system: RwLock<RealPhysicsSystem>,
    real_simulation_cutoff: f64,
    player_temporary_data: PlayerTemporaryData,
}

impl PhysicsSystem {
    pub fn new() -> Self {
        Self {
            real_physics_system: RwLock::new(RealPhysicsSystem::new()),
            currently_simulated_bodies: RwLock::new(HashMap::new()),
            real_simulation_cutoff: 100.0,
            player_temporary_data: PlayerTemporaryData {
                position: DecimalVector3d::zero(),
                linear_velocity: DecimalVector3d::zero(),
            },
        }
    }

    fn phase0(&mut self, ecs: &ECSWorld) -> bool {
        println!("PhysicsSystem / phase0");

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
            true
        } else {
            // println!("Player entity not found, Relativity can behave weird");
            false
        }
    }

    fn phase1(&mut self, ecs: &mut ECSWorld, universe_simulation: &Simulation, delta_time: f64) {
        println!("PhysicsSystem / phase1");

        let decimal_delta_time = f64_to_dbig(delta_time);
        let decimal_half_delta_time = f64_to_dbig(delta_time * 0.5);

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
                            universe_simulation,
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
                        universe_simulation,
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

    fn phase2(&mut self, ecs: &mut ECSWorld) {
        println!("PhysicsSystem / phase2");

        // this list here is so that if entity disappears, the element is cleaned up
        let detected_element_real_physics_ids = Mutex::new(vec![]);

        ecs.parallel_process_all_by_components_mut(
            &[&Components::SimplePhysics, &Components::Transform],
            |entity| {
                let real_physics = entity.components.real_physics.as_ref();
                if real_physics.is_some() {
                    let real_physics = real_physics.unwrap();
                    detected_element_real_physics_ids
                        .lock()
                        .unwrap()
                        .push(real_physics.id);

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

        // TODO this should clear up elements that are removed from the ECS completely
        // does it work? maybe
        let locked_map = self.currently_simulated_bodies.try_write().unwrap();
        let detected_mesh_component_ids = detected_element_real_physics_ids.lock().unwrap();
        let keys: Vec<u64> = locked_map.keys().map(|x| *x).collect();
        drop(locked_map);

        keys.par_iter().for_each(|key| {
            if !detected_mesh_component_ids.contains(key) {
                let mut real_physics_system = self.real_physics_system.try_write().unwrap();
                let mut currently_simulated_bodies =
                    self.currently_simulated_bodies.try_write().unwrap();
                // unload
                println!("PSY UNLOAD {}", key);
                Self::stop_real_physics_sim(
                    &mut real_physics_system,
                    &mut currently_simulated_bodies,
                    *key,
                );
            }
        });
    }

    fn update_simple_physics(
        &self,
        universe_simulation: &Simulation,
        simple_physics: &mut SimplePhysicsComponent,
        transform: &mut TransformComponent,
        delta_time: f64,
        decimal_delta_time: &DBig,
        decimal_half_delta_time: &DBig,
    ) {
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
                real_physics.id,
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
            build_collider(&real_physics.shape_description).build(),
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
        real_physics_component_id: u64,
    ) {
        let simulated_body = currently_simulated_bodies
            .get(&real_physics_component_id)
            .unwrap();

        real_physics_system.remove_body(simulated_body.rigid_body);

        currently_simulated_bodies.remove(&real_physics_component_id);
    }

    pub fn update(
        &mut self,
        ecs: &mut ECSWorld,
        universe_simulation: &Simulation,
        delta_time: f64,
    ) {
        println!("PhysicsSystem / update");

        let should_continue = self.phase0(ecs);
        if should_continue {
            self.phase1(ecs, universe_simulation, delta_time);

            self.real_physics_system
                .try_write()
                .unwrap()
                .step(delta_time);

            self.phase2(ecs);
        }
    }
}

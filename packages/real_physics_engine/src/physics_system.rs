use crate::build_shape::build_shape;
use crate::real_physics_system::{RealPhysicsSystem, SetRealPhysicsBodyKinematics};
use celestial_renderer::geometry::common_icosphere::{
    ICO_BASE_SUBDIVISION, ICO_LEVEL_SUBDIVISIONS, calculate_base_icosphere_parts_count,
};
use celestial_renderer::rendering_system::RenderingSystem;
use common_util::profile;
use dashu_float::DBig;
use ecs::component_trait::Components;
use ecs::components::common::transform_component::TransformComponent;
use ecs::components::physics::is_celestial_body_surface_component::IsCelestialBodySurfaceComponent;
use ecs::components::physics::real_physics_component::{
    ColliderDescription, ColliderShape, RealPhysicsComponent,
};
use ecs::components::physics::simple_physics_component::SimplePhysicsComponent;
use ecs::ecs_world::ECSWorld;
use ecs::entity::Entity;
use glam::{DQuat, DVec3};
use katana_physics::colliders::katana_collider::KatanaCollider;
use katana_physics::katana_rigid_body::KatanaRigidBody;
use math::decimal_vector_3d::DecimalVector3d;
use math::sin_cos::f64_to_dbig;
use media_provider::generic_cache::GenericCache;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ops::Add;
use std::sync::{Mutex, RwLock};
use ts_rs::TS;
use universe_simulation::simulation::Simulation;

struct SimulatedBody {
    rigid_body: u64,
    phase_1_relative_position: DVec3,
    phase_1_relative_linear_velocity: DVec3,
}

struct PlayerTemporaryData {
    position: DecimalVector3d,
    linear_velocity: DVec3,
}

pub struct PhysicsSystem {
    currently_simulated_bodies: RwLock<HashMap<u64, SimulatedBody>>,
    real_physics_system: RwLock<RealPhysicsSystem>,
    real_simulation_cutoff: f64,
    player_temporary_data: PlayerTemporaryData,
}

impl Default for PhysicsSystem {
    fn default() -> Self {
        Self::new()
    }
}

struct PhysicsUpdateContext<'a> {
    ecs: &'a mut ECSWorld,
    universe_simulation: &'a Simulation,
    rendering_system: &'a RenderingSystem,
    cache: &'a GenericCache<f64>,
    delta_time: f64,
}

pub enum FindStorePlayerFrameDataResult {
    Continue,
    Stop,
}

impl PhysicsSystem {
    pub fn new() -> Self {
        Self {
            real_physics_system: RwLock::new(RealPhysicsSystem::new()),
            currently_simulated_bodies: RwLock::new(HashMap::new()),
            real_simulation_cutoff: 100.0,
            player_temporary_data: PlayerTemporaryData {
                position: DecimalVector3d::zero(),
                linear_velocity: DVec3::new(0.0, 0.0, 0.0),
            },
        }
    }

    fn find_store_player_frame_data(
        &mut self,
        context: &PhysicsUpdateContext,
    ) -> FindStorePlayerFrameDataResult {
        // println!("PhysicsSystem / phase0");

        let player = context.ecs.find_first_by_components(&[
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
            self.player_temporary_data.linear_velocity = simple_physics.linear_velocity;

            let gravity_force = context
                .universe_simulation
                .calculate_gravity_flux(&transform.position)
                .to_dvec3_with_precision(8);

            self.real_physics_system
                .write()
                .unwrap()
                .gravity_plugin
                .lock()
                .unwrap()
                .gravity = gravity_force;

            FindStorePlayerFrameDataResult::Continue
        } else {
            // println!("Player entity not found, Relativity can behave weird");
            FindStorePlayerFrameDataResult::Stop
        }
    }

    fn phase1(&mut self, context: &mut PhysicsUpdateContext) {
        profile!("glue handling", {
            // this could be another system, in future
            context.ecs.parallel_process_all_by_components_mut(
                &[
                    &Components::SimplePhysics,
                    &Components::Transform,
                    &Components::GlueToCelestialBody,
                ],
                |entity| {
                    let transform = entity.components.transform.as_mut().unwrap();
                    let simple_physics = entity.components.simple_physics.as_mut().unwrap();
                    let glue_to_body = entity.components.glue_to_celestial_body.as_ref().unwrap();

                    let body = context
                        .universe_simulation
                        .get_body(&glue_to_body.body_name);

                    let world_orientation = glue_to_body.orientation * body.orientation_f64;
                    // let world_offset = body.orientation_f64 * glue_to_body.offset;
                    let world_offset = body.orientation_f64 * glue_to_body.offset;

                    let new_pos = &body.position + &DecimalVector3d::from_dvec3(world_offset);

                    transform.position = new_pos;
                    transform.orientation = world_orientation;

                    // simple_physics.linear_velocity =
                    //     context.universe_simulation.get_surface_velocity_f64(
                    //         &glue_to_body.body_name,
                    //         body.orientation.as_dquat().mul_vec3(glue_to_body.offset), // TODO
                    //                                                                    // glue_to_body.offset,
                    //     )
                },
            );
        });
        context.ecs.parallel_process_all_by_components_mut(
            &[&Components::SimplePhysics, &Components::Transform],
            |entity| {
                let has_glue = entity.components.glue_to_celestial_body.is_some();

                let transform = entity.components.transform.as_mut().unwrap();
                let simple_physics = entity.components.simple_physics.as_mut().unwrap();

                let real_physics = entity.components.real_physics.as_ref();

                let has_real_physics = real_physics.is_some();

                if has_real_physics {
                    let real_physics = real_physics.unwrap();

                    let handle_or_none = self.handle_real_physics_simulation_start_stop(
                        entity.id,
                        transform,
                        simple_physics,
                        real_physics,
                        context.cache,
                    );

                    match handle_or_none {
                        None => {
                            if !has_glue {
                                self.update_simple_physics(
                                    context.universe_simulation,
                                    context.delta_time,
                                    simple_physics,
                                    transform,
                                )
                            }
                        }
                        Some(id) => {
                            let relative_position = (&transform.position
                                - &self.player_temporary_data.position)
                                .to_dvec3_with_precision(8);
                            let mut relative_linear_velocity = simple_physics.linear_velocity
                                - self.player_temporary_data.linear_velocity;

                            let simulated_object_rigid_body = {
                                let mut map = self.currently_simulated_bodies.write().unwrap();
                                let simulated_object = map.get_mut(&id).unwrap();
                                simulated_object.phase_1_relative_position = relative_position;
                                simulated_object.phase_1_relative_linear_velocity =
                                    relative_linear_velocity;
                                simulated_object.rigid_body
                            }; // unlocks

                            let mut real_physics_system = self.real_physics_system.write().unwrap();

                            // if real_physics
                            //     .collider_descriptions
                            //     .iter()
                            //     .fold(0.0, |p, c| p + c.mass)
                            //     > 0.0
                            //     && !has_glue
                            // {
                            //     let gravity_force = context
                            //         .universe_simulation
                            //         .calculate_gravity_flux(&transform.position)
                            //         .to_dvec3_with_precision(5)
                            //         * context.delta_time;
                            //
                            //     relative_linear_velocity += gravity_force
                            //
                            //     // real_physics_system
                            //     //     .apply_force(simulated_object_rigid_body, gravity_force)
                            //     //     .unwrap();
                            //
                            //     // dbg!(relative_linear_velocity);
                            // }

                            real_physics_system
                                .set_body_kinematics(
                                    simulated_object_rigid_body,
                                    SetRealPhysicsBodyKinematics {
                                        linear_velocity: Some(relative_linear_velocity),
                                        angular_velocity: Some(simple_physics.angular_velocity),
                                        position: Some(relative_position),
                                        orientation: None,
                                        wake_up: false,
                                    },
                                )
                                .unwrap();
                        }
                    }
                } else {
                    if !has_glue {
                        self.update_simple_physics(
                            context.universe_simulation,
                            context.delta_time,
                            simple_physics,
                            transform,
                        )
                    }
                }
            },
        );
    }

    fn phase2(&mut self, context: &mut PhysicsUpdateContext) {
        // println!("PhysicsSystem / phase2");

        // this list here is so that if entity disappears, the element is cleaned up
        let detected_element_real_physics_ids = Mutex::new(vec![]);

        context.ecs.parallel_process_all_by_components_mut(
            &[
                &Components::RealPhysics,
                &Components::SimplePhysics,
                &Components::Transform,
            ],
            |entity| {
                let has_glue = entity.components.glue_to_celestial_body.is_some();

                let real_physics = entity.components.real_physics.as_ref().unwrap();
                detected_element_real_physics_ids
                    .lock()
                    .unwrap()
                    .push(real_physics.id);
                if has_glue {
                    return;
                }

                let map = self.currently_simulated_bodies.read().unwrap();
                let simulated = map.get(&real_physics.id);

                if let Some(simulated) = simulated {
                    let simple_physics = entity.components.simple_physics.as_mut().unwrap();
                    if real_physics
                        .collider_descriptions
                        .iter()
                        .fold(0.0, |p, c| p + c.mass)
                        > 0.0
                    {
                        let real_physics_system = self.real_physics_system.read().unwrap();
                        let kinematics = real_physics_system
                            .get_body_kinematics(simulated.rigid_body)
                            .unwrap();

                        let transform = entity.components.transform.as_mut().unwrap();
                        let position_diff = // todo this NEEDS to be done because collisions resolve moves shit around
                            kinematics.position - simulated.phase_1_relative_position;
                        let linvel_diff =
                            kinematics.linear_velocity - simulated.phase_1_relative_linear_velocity;

                        // if simple_physics.mass.gt(&DBig::ZERO) {
                        //     dbg!(kinematics.position.to_string());
                        //     dbg!(kinematics.linear_velocity.to_string());
                        //     // dbg!(linvel_diff.to_string());
                        //     // dbg!(kinematics.linear_velocity.to_string());
                        //     // dbg!(simulated.phase_1_relative_linear_velocity.to_string());
                        // }

                        let half_delta_time = context.delta_time * 0.5;

                        transform.position =
                            &transform.position + DecimalVector3d::from_dvec3(position_diff);

                        transform.position = &transform.position
                            + &DecimalVector3d::from_dvec3(
                                simple_physics.linear_velocity * half_delta_time,
                            );

                        simple_physics.linear_velocity += linvel_diff;

                        transform.position = &transform.position
                            + &DecimalVector3d::from_dvec3(
                                simple_physics.linear_velocity * half_delta_time,
                            );

                        transform.orientation = kinematics.orientation;
                        simple_physics.angular_velocity = kinematics.angular_velocity;
                    }
                }
            },
        );

        // TODO this should clear up elements that are removed from the ECS completely
        // does it work? maybe
        let locked_map = self.currently_simulated_bodies.write().unwrap();
        let detected_mesh_component_ids = detected_element_real_physics_ids.lock().unwrap();
        let keys: Vec<u64> = locked_map.keys().copied().collect();
        drop(locked_map);

        keys.par_iter().for_each(|key| {
            if !detected_mesh_component_ids.contains(key) {
                let mut real_physics_system = self.real_physics_system.write().unwrap();
                let mut currently_simulated_bodies =
                    self.currently_simulated_bodies.write().unwrap();
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
        delta_time: f64,
        simple_physics: &mut SimplePhysicsComponent,
        transform: &mut TransformComponent,
    ) {
        let half_delta_time = delta_time * 0.5;
        transform.position = &transform.position
            + &DecimalVector3d::from_dvec3(simple_physics.linear_velocity * half_delta_time);

        if !simple_physics.is_static {
            let gravity_impulse = universe_simulation
                .calculate_gravity_flux(&transform.position)
                .to_dvec3_with_precision(5)
                * delta_time;

            simple_physics.linear_velocity += gravity_impulse;
        }

        transform.position = &transform.position
            + &DecimalVector3d::from_dvec3(simple_physics.linear_velocity * half_delta_time);

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
        entity_id: u64,
        transform: &TransformComponent,
        simple_physics: &SimplePhysicsComponent,
        real_physics: &RealPhysicsComponent,
        cache: &GenericCache<f64>,
    ) -> Option<u64> {
        let relative_position = (&transform.position - &self.player_temporary_data.position);

        let cutoff = match real_physics.override_real_simulation_cutoff {
            None => self.real_simulation_cutoff,
            Some(cutoff) => cutoff,
        };
        let should_simulate = relative_position.length() < f64_to_dbig(cutoff);

        let mut exists = {
            self.currently_simulated_bodies
                .read()
                .unwrap()
                .contains_key(&real_physics.id)
        };

        if !should_simulate && exists {
            let mut real_physics_system = self.real_physics_system.write().unwrap();
            let mut currently_simulated_bodies = self.currently_simulated_bodies.write().unwrap();
            // unload
            println!("PSY UNLOAD {}", real_physics.id);
            Self::stop_real_physics_sim(
                &mut real_physics_system,
                &mut currently_simulated_bodies,
                real_physics.id,
            );
            exists = false;
        } else if should_simulate && !exists {
            let mut real_physics_system = self.real_physics_system.write().unwrap();
            let mut currently_simulated_bodies = self.currently_simulated_bodies.write().unwrap();
            // load
            println!("PSY LOAD {}", real_physics.id);
            Self::start_real_physics_sim(
                &mut real_physics_system,
                &mut currently_simulated_bodies,
                entity_id,
                transform,
                simple_physics,
                real_physics,
                cache,
            );
            exists = true;
        }

        if exists { Some(real_physics.id) } else { None }
    }

    fn start_real_physics_sim(
        real_physics_system: &mut RealPhysicsSystem,
        currently_simulated_bodies: &mut HashMap<u64, SimulatedBody>,
        entity_id: u64,
        transform: &TransformComponent,
        simple_physics: &SimplePhysicsComponent,
        real_physics: &RealPhysicsComponent,
        cache: &GenericCache<f64>,
    ) {
        let mut rigid_body = KatanaRigidBody::new();
        rigid_body.user_data = entity_id as u128;

        let mut colliders = real_physics
            .collider_descriptions
            .iter()
            .map(|description| {
                let shape = build_shape(description, cache);
                let mut collider = KatanaCollider::new(
                    shape,
                    description.offset,
                    description.orientation,
                    description.mass,
                );
                collider.user_data = entity_id as u128;
                collider
            });

        for collider in colliders {
            rigid_body.add_collider(collider);
        }

        let body_id = real_physics_system.add_body(rigid_body);

        let simulated = SimulatedBody {
            rigid_body: body_id,
            phase_1_relative_position: DVec3::new(0.0, 0.0, 0.0),
            phase_1_relative_linear_velocity: DVec3::new(0.0, 0.0, 0.0),
        };

        real_physics_system
            .set_body_kinematics(
                body_id,
                SetRealPhysicsBodyKinematics {
                    position: None,
                    orientation: Some(transform.orientation),
                    angular_velocity: Some(simple_physics.angular_velocity),
                    linear_velocity: None,
                    wake_up: true,
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

    // TODO not camera but player
    pub fn raycast_real(&self, camera_relative_origin: DVec3, direction: DVec3) -> Option<f64> {
        self.real_physics_system
            .try_write()
            .unwrap()
            .raycast(camera_relative_origin, direction)
    }

    fn update_celestial_body_surfaces(context: &mut PhysicsUpdateContext) {
        let all_bodies = context
            .universe_simulation
            .bodies
            .iter()
            .map(|x| x.body.name.clone());

        let segments_count = calculate_base_icosphere_parts_count(ICO_BASE_SUBDIVISION);

        let existing_entities = context
            .ecs
            .find_all_ids_by_components(&[&Components::IsCelestialBodySurface]);
        // println!("existing_entities {:?}", existing_entities);
        let mut currently_simulated: HashMap<(String, u16), u64> = HashMap::new();
        for existing_id in existing_entities {
            // println!("existing_id {existing_id}");
            let existing = &context.ecs[existing_id];
            // let glue = existing.components.glue_to_celestial_body.as_ref().unwrap(
            let is_celestial_body_surface = existing
                .components
                .is_celestial_body_surface
                .as_ref()
                .unwrap();
            // println!("detected {} {}", shape.body_name.clone(), shape.index);
            currently_simulated.insert(
                (
                    is_celestial_body_surface.body_name.clone(),
                    is_celestial_body_surface.index,
                ),
                existing_id,
            );
        }

        for body in all_bodies {
            for segment in 0..segments_count {
                let should_have_physics = context
                    .rendering_system
                    .should_have_physics_terrain_water(&body, segment);
                let existing_entity_id = currently_simulated.get(&(body.clone(), segment));
                let already_has_physics = existing_entity_id.is_some();

                // println!(
                //     "body {}, segment {}, should_have_physics {}, already_has_physics {}",
                //     body, segment, should_have_physics.0, already_has_physics
                // );

                // and now, the fun

                // only terrain! why would water need this i dont know, but might later
                // everything supports it for some reason so it can be done but its usage
                // will be different
                if should_have_physics.0 && !already_has_physics {
                    // should have, but doesn't, time to add it to ECS
                    let mut entity = Entity::new();
                    let terrain_physics_data = context
                        .rendering_system
                        .get_terrain_physics_components(&body, segment)
                        .unwrap();
                    entity.components.is_celestial_body_surface =
                        Some(IsCelestialBodySurfaceComponent::new(
                            terrain_physics_data.1.body_name.clone(),
                            segment,
                        ));
                    entity.components.real_physics = Some(terrain_physics_data.0.clone());
                    entity.components.glue_to_celestial_body = Some(terrain_physics_data.1.clone());
                    entity.components.simple_physics = Some(SimplePhysicsComponent::new_static());
                    entity.components.transform = Some(TransformComponent::default());
                    // println!("adding {body} {segment}");
                    context.ecs.add(entity);
                } else if !should_have_physics.0 && already_has_physics {
                    // shouldn't have, but has, time to remove it from ECS
                    let existing_entity_id = existing_entity_id.unwrap();
                    println!("removing {body} {segment}");
                    context.ecs.remove_by_id(*existing_entity_id);
                }
            }
        }
    }

    pub fn update_part_1(
        &mut self,
        ecs: &mut ECSWorld,
        universe_simulation: &Simulation,
        rendering_system: &RenderingSystem,
        cache: &GenericCache<f64>,
        delta_time: f64,
    ) -> FindStorePlayerFrameDataResult {
        let mut context = PhysicsUpdateContext {
            ecs,
            universe_simulation,
            rendering_system,
            cache,
            delta_time,
        };

        // println!("PhysicsSystem / update");
        profile!("update_celestial_body_surfaces", {
            Self::update_celestial_body_surfaces(&mut context);
        });

        let should_continue = profile!("phase0", { self.find_store_player_frame_data(&context) });
        if let FindStorePlayerFrameDataResult::Continue = should_continue {
            profile!("phase1", {
                self.phase1(&mut context);
            });
        }

        should_continue
    }

    pub fn update_part_2_physics_step(&mut self, delta_time: f64) {
        profile!("real_physics_system step", {
            self.real_physics_system.write().unwrap().step(delta_time);
        });
    }

    pub fn update_part_3(
        &mut self,
        ecs: &mut ECSWorld,
        universe_simulation: &Simulation,
        rendering_system: &RenderingSystem,
        cache: &GenericCache<f64>,
        delta_time: f64,
    ) {
        let mut context = PhysicsUpdateContext {
            ecs,
            universe_simulation,
            rendering_system,
            cache,
            delta_time,
        };

        profile!("phase2", {
            self.phase2(&mut context);
        });
    }
}

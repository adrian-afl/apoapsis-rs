use crate::simulation::Simulation;
use ecs::component_trait::Components;
use ecs::ecs_world::ECSWorld;

pub struct UniverseSimulationUpdaterSystem {}

impl Default for UniverseSimulationUpdaterSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl UniverseSimulationUpdaterSystem {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&mut self, simulation: &mut Simulation, ecs: &mut ECSWorld, delta_time: f64) {
        println!("UniverseSimulationUpdaterSystem / update");

        let camera_entity =
            ecs.find_first_by_components(&[&Components::CameraFocus, &Components::Transform]);
        if camera_entity.is_none() {
            println!(
                "CameraFocus + Transform entity not found in ECS World, cannot update the universe"
            );
            return;
        }

        let camera_entity = camera_entity.unwrap();
        let camera_transform = camera_entity.components.transform.as_ref().unwrap();
        let camera_position = &camera_transform.position.clone();

        let clock_entity = ecs.find_first_by_components_mut(&[&Components::UniverseClock]);
        if clock_entity.is_none() {
            println!("Universe Clock not found in ECS World, cannot update the universe");
            return;
        }

        let clock_entity = clock_entity.unwrap();

        let clock = clock_entity.components.universe_clock.as_mut().unwrap();

        simulation.update(camera_position, &clock.time);

        clock.advance(delta_time);
    }
}

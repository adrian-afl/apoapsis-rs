use crate::component_types;
use crate::ecs::component_trait::component_type;
use crate::ecs::ecs_world::ECSWorld;
use crate::ecs::system_trait::SystemTrait;
use crate::ecs_components::camera::camera_focus_component::CameraFocusComponent;
use crate::ecs_components::common::transform_component::TransformComponent;
use crate::simulation::simulation::Simulation;
use std::sync::{Arc, Mutex};

pub struct UniverseSimulationUpdaterSystem {
    universe: Arc<Mutex<Simulation>>,
    ecs: Arc<Mutex<ECSWorld>>,
}

impl UniverseSimulationUpdaterSystem {
    pub fn new(universe: Arc<Mutex<Simulation>>, ecs: Arc<Mutex<ECSWorld>>) -> Self {
        Self { universe, ecs }
    }
}

impl SystemTrait for UniverseSimulationUpdaterSystem {
    fn update(ecs: &mut ECSWorld, delta_time: f64) {
        let camera_focus = ecs
            .find_first_by_components(component_types!(CameraFocusComponent, TransformComponent));
    }
}

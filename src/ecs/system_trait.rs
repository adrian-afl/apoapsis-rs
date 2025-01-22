use crate::ecs::ecs_world::ECSWorld;

pub trait SystemTrait {
    fn update(ecs: &mut ECSWorld, delta_time: f64);
}

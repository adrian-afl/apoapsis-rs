use ecs::ecs_world::ECSWorld;

// @api_command reset_world(): void
pub fn reset_world(_payload: &str, ecs: &mut ECSWorld) -> Result<Option<String>, String> {
    ecs.clear();

    Ok(None)
}

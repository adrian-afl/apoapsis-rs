The currently-two fold physics engine
There might be option for three fold in future

How it works

globals for the system:

- currently_simulated_bodies: HashMap<u64, SimulatedBody>, // TODO what is this u64? Make a wrapper type
- player_temporary_data: PlayerTemporaryData,

Steps:

- update_celestial_body_surfaces
  - this manages the components for planet surfaces colliders, but it shouldn't interfere with the rest of here. All
    that it does is add or remove entities within ecs.

- phase0

  Purpose of this function is to find the player entity and read its position and linear velocity.
  This is because the whole real physics engine gets re-centered every frame on the player entity
  But the player entity is also a physics object most of the time, so it gets updated too.
  This means the original beginning position must be cached somewhere and this is the place where it's done.

  - Find first entity that has all following components: **IsPlayer**, **SimplePhysics**, **Transform**
  - If found
    - **player_temporary_data** -> **position** is set to found entity's **Transform** position
    - **player_temporary_data** -> **linear_velocity** is set to found entity's **SimplePhysics** position
    - function returns **true** indicating that physics update **SHOULD** continue

  - Else if not found
    - function returns **false** indicating that physics update **SHOULD NOT** continue

  - TODO here:
    - make phase0 return an enum indicating what it means

- phase1
  - foreach parallel on entities that have all following components: **SimplePhysics**, **Transform**, as entity
    - additional 2 checks for optional components: **RealPhysics**, **GlueToCelestialBody**
    - if **GlueToCelestialBody** is found
      - get the glue target body kinematics
      - set the entity's **Transform** -> **position** to body kinematics position
      - set the entity's **Transform** -> **orientation** to body kinematics orientation
      - set the entity's **SimplePhysics** -> **linear_velocity** to body surface speed at glue point
      - TODO - angular velocity is omitted, might be necessary, but also very negligible
    - if **RealPhysics** is found:
      - handle_real_physics_simulation_start_stop
        - this ...
      - if handle returned None
        - run update_simple_physics
      - else if handle returned an id (TODO: of what?)

- step
  - this runs the step update of the real physics system
  - at this moment the re-centering is done, everything is happening in camera frame
  - after this runs, the positions and velocities of simulated bodies inside physics system will update
  - next step will read the updated values and set them in the ecs

- phase2

        Self::update_celestial_body_surfaces(ecs, universe_simulation, rendering_system);

        let should_continue = self.phase0(ecs);
        if should_continue {
            self.phase1(ecs, universe_simulation, rendering_system, delta_time);

            self.real_physics_system.write().unwrap().step(delta_time);

            self.phase2(ecs);
        }
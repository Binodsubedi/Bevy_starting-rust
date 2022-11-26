use bevy::{math::Vec3Swizzles,prelude::*, time::FixedTimestep};

const TIME_STEP:f32 = 1.0/60.0;
const BOUNDS:Vec2 = Vec2::new(1200.0, 640.0);


fn main(){

    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system_set(

            SystemSet::new()
            .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
            .with_system(player_movement)
            )
        .add_system(bevy::window::close_on_esc)
        .run()
}


/// player component
#[derive(Component)]
struct Player {
    /// linear speed in meters per second
    movement_speed: f32,
    /// rotation speed in radians per second
    rotation_speed: f32,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
   
    let ship = asset_server.load("textures/ship.png");
      // 2D orthographic camera
    commands.spawn(Camera2dBundle::default());

    let horizontal_margin = BOUNDS.x / 4.0;
    let vertical_margin = BOUNDS.y / 4.0;

    // player controlled ship
    commands.spawn((
        SpriteBundle {
            texture: ship,
            ..default()
        },
        Player {
            movement_speed: 500.0,                  // metres per second
            rotation_speed: f32::to_radians(360.0), // degrees per second
        },
    ));



}


//Applying movement controls for the player

fn player_movement(keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Transform)> 
    ){


    let(ship, mut transform) = query.single_mut();

    let mut rotation_factor = 0.00;
    let mut movement_factor = 0.00;

    
    if keyboard_input.pressed(KeyCode::Left) {
        rotation_factor += 1.0;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        rotation_factor -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::Up) {
        movement_factor += 1.0;
    }

    // update the ship rotation around the Z axis (perpendicular to the 2D plane of the screen)
    transform.rotate_z(rotation_factor * ship.rotation_speed * TIME_STEP);

    // get the ship's forward vector by applying the current rotation to the ships initial facing vector
    let movement_direction = transform.rotation * Vec3::Y;
    // get the distance the ship will move based on direction, the ship's movement speed and delta time
    let movement_distance = movement_factor * ship.movement_speed * TIME_STEP;
    // create the change in translation using the new movement direction and distance
    let translation_delta = movement_direction * movement_distance;
    // update the ship translation with our new translation delta
    transform.translation += translation_delta;

    // bound the ship within the invisible level bounds
    let extents = Vec3::from((BOUNDS / 2.0, 0.0));
    transform.translation = transform.translation.min(extents).max(-extents);



}




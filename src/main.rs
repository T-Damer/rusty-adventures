use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod camera;
pub use camera::*;

struct Player;
struct Jumper {
    jump_impulse: f32,
    is_jumping: bool,
}

fn player_jumps(
    keyboard_input: Res<Input<KeyCode>>,
    mut players: Query<(&Jumper, &mut RigidBodyVelocity), With<Player>>,
) {
    for (jumper, mut velocity) in players.iter_mut() {
        if keyboard_input.pressed(KeyCode::Up)
            || keyboard_input.pressed(KeyCode::Space) && !jumper.is_jumping
        {
            velocity.linvel = Vec2::new(0., jumper.jump_impulse).into();
        }
    }
}

fn set_jumping_false_if_touching_floor(
    entity: Entity,
    jumper: &mut Jumper,
    contact_event: &ContactEvent,
) {
    if let ContactEvent::Started(h1, h2) = contact_event {
        if h1.entity() == entity || h2.entity() == entity {
            jumper.is_jumping = false
        }
    }
}

fn jump_reset(
    mut query: Query<(Entity, &mut Jumper)>,
    mut contact_events: EventReader<ContactEvent>,
) {
    for contact_event in contact_events.iter() {
        for (entity, mut jumper) in query.iter_mut() {
            set_jumping_false_if_touching_floor(entity, &mut jumper, contact_event);
        }
    }
}

fn spawn_player(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    let rigid_body = RigidBodyBundle {
        mass_properties: RigidBodyMassPropsFlags::ROTATION_LOCKED.into(),
        activation: RigidBodyActivation::cannot_sleep(),
        ccd: RigidBodyCcd {
            ccd_enabled: true,
            ..Default::default()
        },
        ..Default::default()
    };
    let collider = ColliderBundle {
        shape: ColliderShape::cuboid(0.5, 0.5),
        flags: ColliderFlags {
            active_events: ActiveEvents::CONTACT_EVENTS,
            ..Default::default()
        },
        ..Default::default()
    };

    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
            sprite: Sprite::new(Vec2::new(1.0, 1.0)),
            ..Default::default()
        })
        .insert_bundle(rigid_body)
        .insert_bundle(collider)
        .insert(Jumper {
            jump_impulse: 3.,
            is_jumping: false,
        })
        .insert(RigidBodyPositionSync::Discrete)
        .insert(Player);
}

fn spawn_floor(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    let width = 10.;
    let height = 1.;
    let rigid_body = RigidBodyBundle {
        position: Vec2::new(0.0, -2.).into(),
        body_type: RigidBodyType::Static,
        ..Default::default()
    };
    let collider = ColliderBundle {
        shape: ColliderShape::cuboid(width / 2., height / 2.),
        ..Default::default()
    };
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(0.4, 0.4, 0.4).into()),
            sprite: Sprite::new(Vec2::new(width, height)),
            ..Default::default()
        })
        .insert_bundle(rigid_body)
        .insert_bundle(collider)
        .insert(RigidBodyPositionSync::Discrete);
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(camera_2d());
}

// main
fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Rusty adventures".to_string(),
            width: 640.0,
            height: 400.0,
            vsync: true,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_startup_system(setup.system())
        .add_startup_stage("player_setup", SystemStage::single(spawn_player.system()))
        .add_system(player_jumps.system())
        .add_system(jump_reset.system())
        .add_startup_stage("floor_setup", SystemStage::single(spawn_floor.system()))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(DefaultPlugins)
        .run();
}

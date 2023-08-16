use ambient_api::{
    core::{
        app::components::main_scene,
        camera::{
            components::aspect_ratio_from_window,
            concepts::make_perspective_infinite_reverse_camera,
        },
        physics::components::{physics_controlled, plane_collider},
        player::components::{is_player, user_id},
        primitives::components::{cube, quad},
        rendering::components::color,
        transform::{
            components::{lookat_target, translation},
            concepts::make_transformable,
        },
    },
    prelude::*,
};

use embers::tutorial::messages::Input;

#[main]
pub fn main() {
    Entity::new()
        .with_merge(make_perspective_infinite_reverse_camera())
        .with(aspect_ratio_from_window(), EntityId::resources())
        .with(main_scene(), ())
        .with(translation(), Vec3::ONE * 5.)
        .with(lookat_target(), vec3(0., 0., 0.))
        .spawn();

    Entity::new()
        .with(quad(), ())
        .with(plane_collider(), ())
        .spawn();

    spawn_query(is_player()).bind(|r| {
        for (e, _) in r {
            entity::add_components(
                e,
                Entity::new()
                    .with(translation(), vec3(0.0, 0.0, 3.0))
                    .with(cube(), ())
                    .with(color(), random::<Vec3>().extend(0.8)),
            );
        }
    });

    Input::subscribe(|source, msg| {
        let Some(player_id) = source.client_entity_id() else { return; };
        println!("Got input info {:?} from client id {:?}", msg, player_id);
        let pos = entity::get_component(player_id, translation()).unwrap();
        entity::set_component(
            player_id,
            translation(),
            pos + msg.direction.extend(0.0) * 0.1,
        );
    });
}

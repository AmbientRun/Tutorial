use ambient_api::{
    core::{
        app::components::main_scene,
        camera::{
            components::aspect_ratio_from_window,
            concepts::make_perspective_infinite_reverse_camera,
        },
        physics::components::{
            character_controller_height, character_controller_radius, physics_controlled,
            plane_collider,
        },
        player::components::is_player,
        prefab::components::prefab_from_url,
        primitives::components::{cube, quad},
        rendering::components::color,
        transform::{
            components::{lookat_target, translation},
            concepts::make_transformable,
        },
    },
    prelude::*,
};

use embers::tutorial::{assets, components::player_direction, messages::Input};

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
                    .with(prefab_from_url(), assets::url("X Bot.fbx"))
                    .with(player_direction(), Vec2::ZERO)
                    .with(physics_controlled(), ())
                    .with(character_controller_height(), 2.0)
                    .with(character_controller_radius(), 0.5),
            );
        }
    });

    Input::subscribe(|source, msg| {
        let Some(player_id) = source.client_entity_id() else { return; };
        entity::add_component(player_id, player_direction(), msg.direction);
    });

    query(player_direction()).each_frame(move |players| {
        for (player_id, direction) in players {
            let speed = 0.1;
            let displace = (direction * speed).extend(-0.1); // extend the Z axis to fake a gravity
            physics::move_character(player_id, displace, 0.01, delta_time());
        }
    });
}

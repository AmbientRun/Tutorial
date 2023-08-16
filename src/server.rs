use ambient_api::{
    animation::{AnimationPlayer, BlendNode, PlayClipFromUrlNode},
    core::{
        animation::components::apply_animation_player,
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

use embers::tutorial::{assets, components::*, messages::Input};

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
            // must put in the loop so that we create animation player for each player
            let idle = PlayClipFromUrlNode::new(assets::url(
                "Rifle Aiming Idle.fbx/animations/mixamo.com.anim",
            ));
            let run =
                PlayClipFromUrlNode::new(assets::url("Run Forward.fbx/animations/mixamo.com.anim"));
            let blend = BlendNode::new(&idle, &run, 0.0);
            let anim_player = AnimationPlayer::new(&blend);

            entity::add_components(
                e,
                Entity::new()
                    .with(translation(), vec3(0.0, 0.0, 3.0))
                    .with(prefab_from_url(), assets::url("X Bot.fbx"))
                    .with(player_direction(), Vec2::ZERO)
                    .with(physics_controlled(), ())
                    .with(character_controller_height(), 2.0)
                    .with(character_controller_radius(), 0.5)
                    .with(blend_node_id(), blend.0.get_entity_id())
                    .with(apply_animation_player(), anim_player.0),
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
            if direction != Vec2::ZERO {
                let blend_node_id = entity::get_component(player_id, blend_node_id()).unwrap();
                let blend = BlendNode::from_entity(blend_node_id);
                blend.set_weight(1.0);
            } else {
                let blend_node_id = entity::get_component(player_id, blend_node_id()).unwrap();
                let blend = BlendNode::from_entity(blend_node_id);
                blend.set_weight(0.0);
            }
            physics::move_character(player_id, displace, 0.01, delta_time());
        }
    });
}

use std::f32::consts::{PI, TAU};

use ambient_api::{
    animation::{AnimationPlayer, BlendNode, PlayClipFromUrlNode},
    core::{
        animation::components::apply_animation_player,
        app::components::main_scene,
        camera::{
            components::aspect_ratio_from_window,
            concepts::make_perspective_infinite_reverse_camera,
        },
        ecs::components::{children, parent},
        physics::components::{
            character_controller_height, character_controller_radius, physics_controlled,
            plane_collider,
        },
        player::components::is_player,
        prefab::components::prefab_from_url,
        primitives::components::{cube, quad},
        rendering::components::color,
        transform::{
            components::{
                local_to_parent, local_to_world, lookat_target, rotation, scale, translation,
            },
            concepts::make_transformable,
        },
    },
    prelude::*,
};

use embers::tutorial::{assets, components::*, messages::Input};

#[main]
pub fn main() {
    Entity::new()
        .with_merge(make_transformable())
        .with(quad(), ())
        .with(plane_collider(), ())
        .with(scale(), vec3(10.0, 10.0, 1.0))
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
            let cam = Entity::new()
                .with_merge(make_perspective_infinite_reverse_camera())
                .with(aspect_ratio_from_window(), EntityId::resources())
                .with(main_scene(), ())
                .with(translation(), vec3(0.0, 2.0, 3.0))
                .with(parent(), e)
                .with(local_to_parent(), Mat4::default())
                .with(rotation(), Quat::from_rotation_x(std::f32::consts::PI / 2.))
                .spawn();

            let model = Entity::new()
                .with(prefab_from_url(), assets::url("X Bot.fbx"))
                .with(apply_animation_player(), anim_player.0)
                .with(local_to_parent(), Mat4::default())
                .with(parent(), e)
                .with(rotation(), Quat::from_rotation_z(std::f32::consts::PI))
                .spawn();
            entity::add_components(
                e,
                Entity::new()
                    .with(translation(), vec3(0.0, 0.0, 3.0))
                    .with(player_direction(), Vec2::ZERO)
                    .with(physics_controlled(), ())
                    .with(character_controller_height(), 2.0)
                    .with(character_controller_radius(), 0.5)
                    .with(blend_node_id(), blend.0.get_entity_id())
                    .with(children(), vec![cam, model])
                    .with(player_model_ref(), model)
                    .with(player_cam_ref(), cam)
                    .with(player_pitch(), 0.0)
                    .with(player_yaw(), 0.0)
                    .with(local_to_world(), Mat4::default()),
            );
        }
    });

    Input::subscribe(|source, msg| {
        let Some(player_id) = source.client_entity_id() else { return; };
        entity::add_component(player_id, player_direction(), msg.direction);

        if msg.shoot {
            println!("shoot");
            let result = physics::raycast_first(msg.ray_origin, msg.ray_dir);
            if let Some(hit) = result {
                println!("hit: {:?}", hit);
            }
        }

        // for the camera and movement
        let yaw = entity::mutate_component(player_id, player_yaw(), |yaw| {
            *yaw = (*yaw + msg.mouse_delta.x * 0.01) % TAU;
        })
        .unwrap_or_default();
        let pitch = entity::mutate_component(player_id, player_pitch(), |pitch| {
            *pitch = (*pitch + msg.mouse_delta.y * 0.01).clamp(-PI / 3., PI / 3.);
        })
        .unwrap_or_default();
        entity::set_component(player_id, rotation(), Quat::from_rotation_z(yaw));
        if let Some(cam) = entity::get_component(player_id, player_cam_ref()) {
            entity::set_component(cam, rotation(), Quat::from_rotation_x(PI / 2. + pitch));
        } else {
            println!("No cam");
        }
    });

    query((player_direction(), rotation())).each_frame(move |players| {
        for (player_id, (direction, rot)) in players {
            let speed = 0.1;
            let displace = rot * (direction * speed).extend(-0.1); // extend the Z axis to fake a gravity
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

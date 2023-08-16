use ambient_api::{
    animation::{get_bone_by_bind_id, BindId},
    core::{
        messages::Frame,
        model::components::model_loaded,
        player::components::is_player,
        prefab::components::prefab_from_url,
        rendering::components::color,
        transform::{
            components::{local_to_parent, reset_scale, rotation, scale, translation},
            concepts::make_transformable,
        },
    },
    prelude::*,
};
use embers::tutorial::messages::Input;

use crate::embers::tutorial::assets;
#[main]
pub fn main() {
    spawn_query(is_player()).bind(move |results| {
        for (model, _) in results {
            run_async(async move {
                entity::wait_for_component(model, model_loaded()).await;
                println!("___model loaded___waiting for binding__");
                let hand = get_bone_by_bind_id(model, &BindId::RightHand);
                if hand.is_none() {
                    return;
                }
                let hand = hand.unwrap();
                let gun = Entity::new()
                    .with_merge(make_transformable())
                    .with(prefab_from_url(), assets::url("red.glb"))
                    .with(translation(), vec3(-0.06, 0.2, 0.0))
                    .with(
                        rotation(),
                        Quat::from_rotation_y(-std::f32::consts::FRAC_PI_2)
                            * Quat::from_rotation_z(std::f32::consts::FRAC_PI_2),
                    )
                    .with(scale(), Vec3::ONE * 0.3)
                    .with(local_to_parent(), Mat4::default())
                    .with(reset_scale(), ())
                    .spawn();

                entity::add_child(hand, gun);
            });
        }
    });

    Frame::subscribe(move |_| {
        let input = input::get();

        let mut direction = Vec2::ZERO;
        if input.keys.contains(&KeyCode::W) {
            direction.y -= 1.0;
        }
        if input.keys.contains(&KeyCode::S) {
            direction.y += 1.0;
        }
        if input.keys.contains(&KeyCode::A) {
            direction.x -= 1.0;
        }
        if input.keys.contains(&KeyCode::D) {
            direction.x += 1.0;
        }

        Input {
            direction,
            mouse_delta: input.mouse_delta,
        }
        .send_server_unreliable();
    });
}

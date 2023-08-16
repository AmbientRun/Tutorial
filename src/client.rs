use ambient_api::{
    animation::{get_bone_by_bind_id, BindId},
    core::{
        app::components::window_logical_size,
        messages::Frame,
        model::components::model_loaded,
        player::components::is_player,
        prefab::components::prefab_from_url,
        rect::components::{background_color, line_from, line_to, line_width},
        rendering::components::color,
        transform::{
            components::{local_to_parent, reset_scale, rotation, scale, translation},
            concepts::make_transformable,
        },
    },
    prelude::*,
};
use embers::tutorial::{
    components::{player_cam_ref, player_model_ref},
    messages::Input,
};

use crate::embers::tutorial::assets;

#[element_component]
pub fn App(_hooks: &mut Hooks) -> Element {
    Crosshair.el()
}

#[element_component]
fn Crosshair(hooks: &mut Hooks) -> Element {
    let size = hooks.use_window_logical_resolution();
    let center_x = size.x as f32 / 2.;
    let center_y = size.y as f32 / 2.;

    Group::el([
        Line.el()
            .with(line_from(), vec3(center_x - 10., center_y, 0.))
            .with(line_to(), vec3(center_x + 10., center_y, 0.))
            .with(line_width(), 2.),
        Line.el()
            .with(line_from(), vec3(center_x, center_y - 10., 0.))
            .with(line_to(), vec3(center_x, center_y + 10., 0.))
            .with(line_width(), 2.),
    ])
}

#[main]
pub fn main() {
    App.el().spawn_interactive();

    spawn_query((is_player(), player_model_ref())).bind(move |results| {
        for (_player_id, (_is_player, model)) in results {
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

    let mut last_shot = game_time();
    let mut is_shooting = false;

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

        let mut shoot = false;
        if input.mouse_buttons.contains(&MouseButton::Left) {
            if is_shooting {
                if game_time() - last_shot > Duration::from_millis(200) {
                    shoot = true;
                    last_shot = game_time();
                }
            } else {
                shoot = true;
                is_shooting = true;
                last_shot = game_time();
            }
        } else {
            is_shooting = false;
        }

        let player_id = player::get_local();
        let cam = entity::get_component(player_id, player_cam_ref());
        if cam.is_none() {
            return;
        }

        let cam = cam.unwrap();
        let window_size =
            entity::get_component(entity::resources(), window_logical_size()).unwrap();
        let ray = camera::screen_position_to_world_ray(
            cam,
            vec2(window_size.x as f32 / 2., window_size.y as f32 / 2.),
        );

        Input {
            direction,
            mouse_delta: input.mouse_delta,
            shoot,
            ray_origin: ray.origin,
            ray_dir: ray.dir,
        }
        .send_server_unreliable();
    });
}

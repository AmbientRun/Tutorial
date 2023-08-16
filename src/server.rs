use ambient_api::{
    core::{
        app::components::main_scene,
        camera::{
            components::aspect_ratio_from_window,
            concepts::make_perspective_infinite_reverse_camera,
        },
        player::components::{is_player, user_id},
        primitives::components::cube,
        rendering::components::color,
        transform::{
            components::{lookat_target, translation},
            concepts::make_transformable,
        },
    },
    prelude::*,
};

#[main]
pub async fn main() {
    Entity::new()
        .with_merge(make_perspective_infinite_reverse_camera())
        .with(aspect_ratio_from_window(), EntityId::resources())
        .with(main_scene(), ())
        .with(translation(), Vec3::ONE * 5.)
        .with(lookat_target(), vec3(0., 0., 0.))
        .spawn();

    spawn_query((is_player(), user_id())).bind(|result| {
        for (e, states) in result {
            println!("Player: {:?}", e);
            println!("States: {:?}", states);
            Entity::new()
                .with_merge(make_transformable())
                .with(cube(), ())
                .with(color(), random::<Vec3>().extend(0.8)) // with extend, Vec3 becomes Quat
                .with(translation(), random::<Vec3>() * 3.)
                .spawn();
        }
    });
}

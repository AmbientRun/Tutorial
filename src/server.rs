use ambient_api::{
    core::{
        app::components::main_scene,
        camera::{
            components::aspect_ratio_from_window,
            concepts::make_perspective_infinite_reverse_camera,
        },
        physics::components::{cube_collider, dynamic, plane_collider},
        primitives::components::{cube, quad},
        transform::{
            components::{lookat_target, rotation, translation},
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

    Entity::new()
        .with_merge(make_transformable())
        .with(quad(), ())
        .with(plane_collider(), ())
        .spawn();

    let e = Entity::new()
        .with_merge(make_transformable())
        .with(cube(), ())
        .with(cube_collider(), Vec3::ONE)
        .with(dynamic(), true)
        .with(translation(), vec3(0., 0., 2.))
        .with(
            rotation(),
            Quat::from_rotation_x(std::f32::consts::FRAC_PI_3),
        )
        .spawn();

    run_async(async move {
        loop {
            sleep(5.0).await;
            entity::set_component(e, translation(), vec3(0., 0., 2.));
            entity::set_component(
                e,
                rotation(),
                Quat::from_rotation_x(std::f32::consts::FRAC_PI_3),
            );
        }
    });
}

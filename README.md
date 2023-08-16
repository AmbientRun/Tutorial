# Chapter 2 Colliders and async

In the previous chapter, we left a challenge to create a cube and a plane. Now we will add colliders to them and make the cube move.

First, we will create a camera and it's identical to the previous chapter.

Then, we will create the plane. It's mostly the same as the previous chapter but we will add a collider to it.

```rust
Entity::new()
    .with_merge(make_transformable())
    .with(quad(), ())
    .with(plane_collider(), ())
    .spawn();
```

Finally, we create a cube and add a collider to it.

```rust
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
```

To make it automatically moving by setting `dynamic()` to `true`. So the cube will move while the plane will be static.

> it's similat to `passive` and `active` in Blender.

Note that the collider of the cube is different from the collider of the plane. The plane is an infinite one, while the cube collider has a size of `Vec3`.

## Reset the cube

We can use `run_async` to schedule things. Note how the `await` and `async` is used inside the `run_async` and don't forget the make the `main` function `async` as well.

`entity::set_component` is used here to reset the position and the rotation of the cube.

# Chapter 9 Bind a gun

To bind a gun, we need to find a gun model online e.g. Sketchfab or make a simple one in software like Blender.

In this example, we will import `red.glb` with `ambient assets import`.

> The `red.glb` is made in Blender with Python script, and it is a simple red gun.

Add these in the `client.rs` and then the gun will be attached to the right hand of the player.

`local_to_parent()` means the translation is relative to the parent node (otherwise the `translation()` will mean absolute position in the world), and `reset_scale()` means the scale of the gun will not be affected by the scale of the parent node.

Fine-tune the scale, position(`translation()`) and rotation of the gun to make it look better.

```rust

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
```

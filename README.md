# Chapter 8 Animation

Previously, our model is downloaded from the mixamo with a T-pose.

Now, we want to add some animations to it. We will download from mixamo `without skin` as the skin is in the T-pose FBX.

To use animation, we use `ambient assets import` again to import the animation fbx or glb as usual.

The command line tool will examine the file and report the relevant `anim` files.

The detailed usage of animtion can be found [here](https://ambientrun.github.io/Ambient/reference/animations.html).

To simplify things, we only use two states: walking and idle.

We will create a `Blend` and then use it with the `query`.

```rust

let idle = PlayClipFromUrlNode::new(assets::url(
    "Rifle Aiming Idle.fbx/animations/mixamo.com.anim",
));
let run =
    PlayClipFromUrlNode::new(assets::url("Run Forward.fbx/animations/mixamo.com.anim"));
let blend = BlendNode::new(&idle, &run, 0.0);
let anim_player = AnimationPlayer::new(&blend);

```

We need to put this in the spawn query. Think about it in this way: each animation is also an entity, so we need to have different ones for different players as these players all acts differently in the world.

Let's create a component and bind it to the player.

```toml
[components]
# ...
# other components definition
# ...
blend_node = { type = "EnityId", attributes = ["Debuggable", "Networked"] }

```

```rust

entity::add_components(
                e,
                Entity::new()
                    .with(translation(), vec3(0.0, 0.0, 3.0))
                    .with_default(physics_controlled())
                    .with(prefab_from_url(), assets::url("X Bot.fbx"))
                    .with_default(player_movement_direction())
                    .with(character_controller_height(), 2.)
                    .with(character_controller_radius(), 0.5)
                    .with(blend_node_id(), blend.0.get_entity_id())
                    .with(apply_animation_player(), anim_player.0),
            );
```

Finally, inside the `query`, we can use the movement info to determine the animation:

```rust
if direction != Vec2::ZERO {
    let blend_node_id = entity::get_component(player_id, blend_node_id()).unwrap();
    let blend = BlendNode::from_entity(blend_node_id);
    blend.set_weight(1.0); // running animation
} else {
    let blend_node_id = entity::get_component(player_id, blend_node_id()).unwrap();
    let blend = BlendNode::from_entity(blend_node_id);
    blend.set_weight(0.0); // idle animation
}
```

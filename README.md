# Chapter 11 UI

Ambient's UI system is heavily inspired by React (with hooks), and follows many of the same patterns.
Take a look at the [React documentation](https://react.dev/reference/react) to learn how hooks work in general.

[See all UI examples here](https://github.com/AmbientRun/Ambient/tree/main/guest/rust/examples/ui).

In this example, we just created a simple CrossHair for a better shooting experience:

```rust
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
```

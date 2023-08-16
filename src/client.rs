use ambient_api::{core::messages::Frame, prelude::*};
use embers::tutorial::messages::Input;
#[main]
pub fn main() {
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

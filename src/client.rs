use ambient_api::{core::rendering::components::color, prelude::*};
use embers::tutorial::messages::Hello;
#[main]
pub fn main() {
    Hello::subscribe(|source, msg| {
        println!("Hello from {:?}. The msg is {:?}", source, msg);
        let c = entity::get_component(msg.cube_id, color()).unwrap();
        println!("The color is {:?}", c);
    });
}

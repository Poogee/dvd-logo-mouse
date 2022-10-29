use mouse_rs::{Mouse, types::Point};
use std::{thread, time};

fn main() {
    let mouse = Mouse::new();
    let mut position = mouse.get_position().unwrap();
    let mut x = 1; let mut y = 1;

    loop{
        thread::sleep(time::Duration::from_millis(5));
        mouse.move_to(position.x + x, position.y + y).expect("Failed to move mouse");
        did_it_hit_border(&mut position, &mut x, &mut y, &mouse)
    }
}

fn did_it_hit_border(position :&mut Point, x: &mut i32, y: &mut i32, mouse: &Mouse) -> () {
    if (*position).x == (*mouse).get_position().unwrap().x {
        *x *= -1;
    }
    if (*position).y == (*mouse).get_position().unwrap().y {
        *y *= -1;
    }

    *position = (*mouse).get_position().unwrap();
}
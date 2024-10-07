use std::io::{self, Write};
use std::fs::File;
use rdev::{listen, Event, EventType, Key};
use std::cell::RefCell;
use std::rc::Rc;

fn main() -> io::Result<()> {
    let input = Rc::new(RefCell::new(String::new()));
    let input_clone = Rc::clone(&input);
    if let Err(error) = listen(move |event| {
        callback(event, &input_clone);
    }) {
        println!("Error: {:?}", error);
    }

    println!("Your text has been written to output.txt");

    Ok(())
}fn callback(event: Event, string: &Rc<RefCell<String>>) {
    if event.event_type == EventType::KeyPress(Key::F4) {
        println!("Exiting...");
        let input_borrow = string.borrow();
        let mut file = File::create("/home/quote/Coding/Rust/keylog/output.txt").expect("Failed to create file");
        file.write_all(input_borrow.as_bytes()).expect("Failed to write to file");
        std::process::exit(0);
    }
    let eventtype: String = match event.event_type {
        EventType::KeyPress(key) => format!("{:?}", key),
        EventType::KeyRelease(key) => format!("{:?}", key),
        EventType::MouseMove {x, y} => format!("MouseMove({},{})\n", x, y),
        EventType::ButtonPress(button) => format!("Mouse button pressed: {:?}\n", button),
        EventType::ButtonRelease(button) => format!("Mouse button released: {:?}\n", button),
        EventType::Wheel { delta_x, delta_y } => format!("Mouse wheel scrolled: ({}, {})\n", delta_x, delta_y),
    };
    let mut input_push = string.borrow_mut();
    input_push.push_str(&eventtype);
}
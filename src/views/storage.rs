use cursive::views::{TextContent, TextView};
use log::debug;
use std::sync::Arc;
use std::thread::sleep;
use std::time;

pub fn setup() -> cursive::views::TextView {
    let storage_content = TextContent::new("Hello from storage view!");
    let storage_view = TextView::new_with_content(storage_content.clone());

    let storage_content = Arc::new(Box::new(storage_content));
    std::thread::spawn(move || update_content(storage_content.clone()));

    return storage_view;
}

fn beep_boop(input: bool) -> String {
    if input {
        return String::from("beep");
    } else {
        return String::from("boop");
    };
}

fn update_content(storage_content: Arc<Box<TextContent>>) {
    //https://github.com/mfs/rust-df/blob/master/src/main.rs
    debug!["hello from udpate_storage_content"];
    let mut flipper = false;

    loop {
        debug!["Beep booper: {}", beep_boop(flipper)];

        sleep(time::Duration::from_secs(1));
        storage_content.set_content(format!("Storage view: {}", beep_boop(flipper)));
        flipper = !flipper;
    }
}

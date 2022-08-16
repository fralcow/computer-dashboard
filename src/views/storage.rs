use crate::beepboop::BeepBooper;
use cursive::views::{TextContent, TextView};
use std::sync::Arc;
use std::thread::sleep;
use std::time;

pub fn setup() -> cursive::views::TextView {
    let initial_message = String::from("Storage view: ");
    let storage_content = TextContent::new(initial_message);
    let storage_view = TextView::new_with_content(storage_content.clone());

    let storage_content = Arc::new(Box::new(storage_content));
    std::thread::spawn(move || update_content(storage_content.clone()));

    return storage_view;
}

fn update_content(storage_content: Arc<Box<TextContent>>) {
    let msg = storage_content.get_content().source().to_owned();
    let mut beeper = BeepBooper::new();

    loop {
        sleep(time::Duration::from_secs(1));
        storage_content.set_content(format!("{}{}", msg, beeper.beep()));
    }
}

#[test]
fn setup_storage_view() {
    let storage_view = setup();
    println!("Storage content: {}", storage_view.get_content().source());
    assert!(storage_view
        .get_content()
        .source()
        .contains("Storage view:"));
}

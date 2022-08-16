use crate::beepboop::BeepBooper;
use cursive::views::{TextContent, TextView};
use std::sync::Arc;
use std::thread::sleep;
use std::time;

pub fn setup() -> cursive::views::TextView {
    let initial_message = String::from("Network view: ");
    let network_content = TextContent::new(initial_message);
    let network_view = TextView::new_with_content(network_content.clone());

    let content_2 = Arc::new(Box::new(network_content));
    std::thread::spawn(move || update_content(content_2.clone()));

    return network_view;
}

fn update_content(network_content: Arc<Box<TextContent>>) {
    let base_message = network_content.get_content().source().to_owned();
    let mut beeper = BeepBooper::new();

    loop {
        sleep(time::Duration::from_secs(1));
        network_content.set_content(format!("{}{}", base_message, beeper.beep()));
    }
}

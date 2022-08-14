use cursive::views::{TextContent, TextView};
use log::debug;
use std::sync::Arc;
use std::thread::sleep;
use std::time;

pub fn setup() -> cursive::views::TextView {
    let network_content = TextContent::new("Network view counter: ");
    let network_view = TextView::new_with_content(network_content.clone());

    let content_2 = Arc::new(Box::new(network_content));
    std::thread::spawn(move || update_content(content_2.clone()));

    return network_view;
}

fn update_content(shared_content: Arc<Box<TextContent>>) {
    debug!["hello from update_text_view"];

    let mut counter: i32 = 0;
    loop {
        debug!["Counter: {}", counter];

        sleep(time::Duration::from_secs(1));
        shared_content.set_content(format!("Network view counter: {}", counter.to_string()));
        counter += 1;
    }
}

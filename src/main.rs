use cursive::views::{DummyView, LinearLayout, TextContent, TextContentRef, TextView};
use env_logger::{builder, Target};
use log::debug;
use std::sync::Arc;
use std::thread::sleep;
use std::time;

fn main() {
    let mut builder = builder();
    builder.target(Target::Stderr).init();

    debug!["hello from main"];

    let mut siv = cursive::default();

    siv.add_global_callback('q', |s| s.quit());

    let tv1 = TextView::new("Hello cursive! Press <q> to quit.");
    let content_2 = TextContent::new("Counter: ");
    let tv2 = TextView::new_with_content(content_2.clone());
    siv.add_layer(
        LinearLayout::vertical()
            .child(tv1)
            .child(DummyView)
            .child(tv2),
    );

    let content_2 = Arc::new(Box::new(content_2));
    std::thread::spawn(move || update_text_view(content_2.clone()));

    siv.set_autorefresh(true);
    siv.run();
}

fn update_text_view(shared_content: Arc<Box<TextContent>>) {
    debug!["hello from update_text_view"];

    let mut counter: i32 = 0;
    loop {
        debug!["Counter: {}", counter];

        sleep(time::Duration::from_secs(1));
        shared_content.set_content(counter.to_string());
        counter += 1;
    }
}

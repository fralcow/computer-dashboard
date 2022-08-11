use cursive::views::{LinearLayout, PaddedView, Panel, TextContent, TextView};
use env_logger::{builder, Target};
use log::debug;
use std::sync::Arc;
use std::thread::sleep;
use std::time;

fn main() {
    log_setup();

    let mut siv = cursive::default();
    siv.add_global_callback('q', |s| s.quit());

    let info_content = TextView::new("Press q to quit");
    let info_view = prettify_text_view(info_content);

    let storage_content = TextView::new("Hello from storage view!");
    let storage_view = prettify_text_view(storage_content);

    let network_content = setup_network_view();
    let network_view = prettify_text_view(network_content);

    let ram_content = TextView::new("Hello from ram view!");
    let ram_view = prettify_text_view(ram_content);

    let cpu_content = TextView::new("Hello from cpu view!");
    let cpu_view = prettify_text_view(cpu_content);

    let info_view = LinearLayout::vertical().child(info_view);

    let top_view = LinearLayout::horizontal()
        .child(storage_view)
        .child(network_view);
    let bottow_view = LinearLayout::horizontal().child(ram_view).child(cpu_view);

    let widgets_view = LinearLayout::vertical().child(top_view).child(bottow_view);

    siv.add_fullscreen_layer(
        LinearLayout::vertical()
            .child(info_view)
            .child(widgets_view),
    );

    siv.set_autorefresh(true);
    siv.run();
}

fn log_setup() {
    let mut builder = builder();
    builder.target(Target::Stderr).init();

    debug!["hello from main"];
}

fn update_network_view_content(shared_content: Arc<Box<TextContent>>) {
    debug!["hello from update_text_view"];

    let mut counter: i32 = 0;
    loop {
        debug!["Counter: {}", counter];

        sleep(time::Duration::from_secs(1));
        shared_content.set_content(format!("Network view counter: {}", counter.to_string()));
        counter += 1;
    }
}

fn setup_network_view() -> cursive::views::TextView {
    let network_content = TextContent::new("Network view counter: ");
    let network_view = TextView::new_with_content(network_content.clone());

    let content_2 = Arc::new(Box::new(network_content));
    std::thread::spawn(move || update_network_view_content(content_2.clone()));

    return network_view;
}

fn prettify_text_view(text_view: TextView) -> Panel<PaddedView<TextView>> {
    let view = PaddedView::new(cursive::view::Margins::lrtb(2, 2, 2, 2), text_view);
    let mut view = Panel::new(view);
    view.set_title("View title");

    return view;
}

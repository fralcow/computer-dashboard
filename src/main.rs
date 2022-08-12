use cursive::views::{LinearLayout, PaddedView, Panel, TextContent, TextView};
//use env_logger::{builder, Target};
use chrono;
use log::debug;
use std::sync::Arc;
use std::thread::sleep;
use std::time;

fn main() {
    let result = setup_logger();
    match result {
        Err(e) => panic!("{}", e),
        _ => (),
    }

    let mut siv = cursive::default();
    siv.add_global_callback('q', |s| s.quit());

    let info_content = TextView::new("Press q to quit");
    let info_view = prettify_text_view(info_content);

    let storage_content = setup_storave_view();
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

fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .level_for("cursive_core", log::LevelFilter::Error)
        .chain(std::io::stderr())
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
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

fn setup_storave_view() -> cursive::views::TextView {
    let storage_content = TextContent::new("Hello from storage view!");
    let storage_view = TextView::new_with_content(storage_content.clone());

    let storage_content = Arc::new(Box::new(storage_content));
    std::thread::spawn(move || update_storage_content(storage_content.clone()));

    return storage_view;
}

fn beep_boop(input: bool) -> String {
    if input {
        return String::from("beep");
    } else {
        return String::from("boop");
    };
}

fn update_storage_content(storage_content: Arc<Box<TextContent>>) {
    debug!["hello from udpate_storage_content"];
    let mut flipper = false;

    loop {
        debug!["Beep booper: {}", beep_boop(flipper)];

        sleep(time::Duration::from_secs(1));
        storage_content.set_content(format!("Storage view: {}", beep_boop(flipper)));
        flipper = !flipper;
    }
}

fn prettify_text_view(text_view: TextView) -> Panel<PaddedView<TextView>> {
    let view = PaddedView::new(cursive::view::Margins::lrtb(2, 2, 2, 2), text_view);
    let mut view = Panel::new(view);
    view.set_title("View title");

    return view;
}

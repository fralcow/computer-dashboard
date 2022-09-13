use crate::views::{cpu, network, storage};
use cursive::views::{LinearLayout, PaddedView, Panel, ResizedView, TextView};
use cursive::CursiveRunnable;

pub fn setup_views() -> CursiveRunnable {
    let mut cursive_runnable = cursive::default();
    cursive_runnable.add_global_callback('q', |s| s.quit());

    let info_content = TextView::new("Press q to quit");
    let info_view = prettify_text_view(info_content, String::from("Dashboard"), (0, 0, 0, 0));

    let storage_content = storage::setup();
    let storage_view = prettify_text_view(storage_content, String::from("Storage"), (1, 1, 0, 0));

    let network_content = network::setup();
    let network_view = prettify_text_view(network_content, String::from("Network"), (1, 1, 0, 0));

    let ram_content = TextView::new("Hello from ram view!");
    let ram_view = prettify_text_view(ram_content, String::from("Ram"), (1, 1, 0, 0));

    let cpu_content = cpu::setup();
    let cpu_view = prettify_text_view(cpu_content, String::from("Cpu"), (1, 1, 0, 0));

    let info_view = LinearLayout::vertical().child(info_view);

    let top_view = LinearLayout::horizontal()
        .child(storage_view)
        .child(network_view);
    let bottow_view = LinearLayout::horizontal().child(ram_view).child(cpu_view);

    let widgets_view = LinearLayout::vertical().child(top_view).child(bottow_view);

    cursive_runnable.add_fullscreen_layer(
        LinearLayout::vertical()
            .child(info_view)
            .child(widgets_view),
    );

    cursive_runnable
}

type PrettyView = Panel<ResizedView<PaddedView<TextView>>>;
/// Converts a TextView into a padded view with the given margins
///
/// # Arguments
///
/// * `text_view` - Initial text content of the view
///
/// * `title` - Initial title of the view
///
/// * `margins` - Margins of the view: left, right, top, bottom
fn prettify_text_view(
    text_view: TextView,
    title: String,
    margins: (usize, usize, usize, usize),
) -> PrettyView {
    let view = PaddedView::new(
        cursive::view::Margins::lrtb(margins.0, margins.1, margins.2, margins.2),
        text_view,
    );
    let view = ResizedView::with_max_height(12, view);
    let mut view = Panel::new(view);
    view.set_title(title);

    return view;
}

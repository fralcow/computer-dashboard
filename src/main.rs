use crate::log::setup_logger;
use crate::views::setup::setup_views;

mod beepboop;
mod log;
mod views;

fn main() {
    setup_logger().expect("Failed to start logger");

    let mut cursive_runnable = setup_views();

    cursive_runnable.set_autorefresh(true);
    cursive_runnable.run();
}

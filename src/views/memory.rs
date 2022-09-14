use ::log::debug;
use cursive::views::{TextContent, TextView};
use procfs::Meminfo;
use std::sync::Arc;
use std::thread::sleep;
use std::time;

pub fn setup() -> cursive::views::TextView {
    let memory_content = TextContent::new("");
    let memory_view = TextView::new_with_content(memory_content.clone());

    let content_2 = Arc::new(Box::new(memory_content));
    std::thread::spawn(move || update_content(content_2.clone()));

    return memory_view;
}

fn update_content(memory_content: Arc<Box<TextContent>>) {
    loop {
        let meminfo = Meminfo::new().unwrap();
        let mem_total_mb = meminfo.mem_total / 1024 / 1024;
        let mem_free_mb = meminfo.mem_available.unwrap() / 1024 / 1024;
        let swap_total_mb = meminfo.swap_total / 1024 / 1024;
        let swap_free_mb = meminfo.swap_free / 1024 / 1024;

        let display_string = format!(
            "\nRam total {:>10}MB\nRam used {:>11}MB\nSwap total {:>9}MB\nSwap used {:>10}MB",
            mem_total_mb,
            mem_total_mb - mem_free_mb,
            swap_total_mb,
            swap_total_mb - swap_free_mb
        );

        debug!("{}", display_string);
        memory_content.set_content(display_string);

        sleep(time::Duration::from_secs(1));
    }
}

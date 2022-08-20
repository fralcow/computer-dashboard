use cursive::views::{TextContent, TextView};
use log::error;
use procfs::net::DeviceStatus;
use std::collections::HashMap;
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
    let mut prev_stats = match procfs::net::dev_status() {
        Ok(stats) => match Some(stats) {
            Some(stats) => stats,
            None => {
                network_content.set_content(format!("Failed to initialize network view"));
                return;
            }
        },
        Err(e) => {
            error!("{}", e);
            network_content.set_content(format!("Failed to initialize network view"));
            return;
        }
    };

    let mut prev_now = std::time::Instant::now();

    loop {
        sleep(time::Duration::from_secs(1));

        let now = std::time::Instant::now();
        let dev_stats = procfs::net::dev_status().unwrap();

        network_content.set_content(format!(
            "{}",
            get_network_stat(now, prev_now, prev_stats, &dev_stats)
        ));

        prev_stats = dev_stats;
        prev_now = now;
    }
}

fn get_network_stat(
    now: std::time::Instant,
    prev_now: std::time::Instant,
    prev_stats: HashMap<String, DeviceStatus>,
    dev_stats: &HashMap<String, DeviceStatus>,
) -> String {
    // calculate diffs from previous
    let dt = (now - prev_now).as_millis() as f32 / 1000.0;

    let mut stats: Vec<_> = dev_stats.values().collect();
    stats.sort_by_key(|s| &s.name);
    let mut result = format!(
        "{:>16}: {:<20}               {:<20}\n",
        "Interface", "bytes recv", "bytes sent"
    );
    result.push_str(&format!(
        "{:>16}  {:<20}               {:<20}\n",
        "================", "====================", "===================="
    ));
    for stat in stats {
        result.push_str(&format!(
            "{:>16}: {:<20}  {:>6.1} kbps  {:<20}  {:>6.1} kbps\n",
            stat.name,
            stat.recv_bytes,
            (stat.recv_bytes - prev_stats.get(&stat.name).unwrap().recv_bytes) as f32 / dt / 1000.0,
            stat.sent_bytes,
            (stat.sent_bytes - prev_stats.get(&stat.name).unwrap().sent_bytes) as f32 / dt / 1000.0
        ));
    }

    result
}

#[test]
fn setup_network_view() {
    let network_view = setup();
    println!("Storage content: {}", network_view.get_content().source());
    assert!(network_view
        .get_content()
        .source()
        .contains("Network view:"));
}

use cursive::views::{TextContent, TextView};
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
    let mut network_stats_getter = new_stats_getter();
    loop {
        sleep(time::Duration::from_secs(1));
        network_content.set_content(format!("{}", network_stats_getter.get_stats()));
    }
}

type NetworkStats = HashMap<String, DeviceStatus>;

struct NetworkStatsGetter {
    time: std::time::Instant,
    stats: NetworkStats,
}

fn new_stats_getter() -> NetworkStatsGetter {
    let dev_stats = procfs::net::dev_status().unwrap();

    let nsg = NetworkStatsGetter {
        time: std::time::Instant::now(),
        stats: dev_stats,
    };

    nsg
}

impl NetworkStatsGetter {
    fn get_stats(&mut self) -> String {
        // calculate diffs from previous
        let now = std::time::Instant::now();
        let dt = (now - self.time).as_millis() as f32 / 1000.0;

        let dev_status = procfs::net::dev_status().unwrap();
        let mut stats: Vec<_> = dev_status.values().collect();

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
                (stat.recv_bytes - self.stats.get(&stat.name).unwrap().recv_bytes) as f32
                    / dt
                    / 1000.0,
                stat.sent_bytes,
                (stat.sent_bytes - self.stats.get(&stat.name).unwrap().sent_bytes) as f32
                    / dt
                    / 1000.0
            ));
        }

        self.stats = dev_status;

        result
    }
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

use ::log::{debug, error};
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
    let mut network_stats_getter = match new() {
        Ok(nst) => nst,
        Err(e) => {
            error!("{}", e);
            network_content.set_content(format!("Failed to initialize network widget"));
            return;
        }
    };

    loop {
        let network_stats = network_stats_getter.get();
        debug!("\n{}", network_stats);
        network_content.set_content(format!("{}", network_stats));

        sleep(time::Duration::from_secs(1));
    }
}

struct NetworkStatsGetter {
    time: std::time::Instant,
    device_statuses: HashMap<String, DeviceStatus>,
}

fn new() -> Result<NetworkStatsGetter, &'static str> {
    let dev_stats = match procfs::net::dev_status() {
        Ok(stats) => Ok(stats),
        _ => Err("some error"),
    };

    match dev_stats {
        Ok(s) => Ok(NetworkStatsGetter {
            time: std::time::Instant::now(),
            device_statuses: s,
        }),
        Err(e) => Err(e),
    }
}

impl NetworkStatsGetter {
    fn get(&mut self) -> String {
        // calculate diffs from previous
        let now = std::time::Instant::now();
        let time_delta = (now - self.time).as_millis() as f32 / 1000.0;

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
                (stat.recv_bytes - self.device_statuses.get(&stat.name).unwrap().recv_bytes) as f32
                    / time_delta
                    / 1000.0,
                stat.sent_bytes,
                (stat.sent_bytes - self.device_statuses.get(&stat.name).unwrap().sent_bytes) as f32
                    / time_delta
                    / 1000.0
            ));
        }

        self.device_statuses = dev_status;

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

use ::log::debug;
use cursive::views::{TextContent, TextView};
use fstab::FsTab;
use nix::sys::statvfs;
use std::path::Path;
use std::sync::Arc;
use std::thread::sleep;
use std::time;

const MTAB_PATH: &str = "/etc/mtab";

pub fn setup() -> cursive::views::TextView {
    let initial_message = String::from("Storage view: ");
    let storage_content = TextContent::new(initial_message);
    let storage_view = TextView::new_with_content(storage_content.clone());

    let storage_content = Arc::new(Box::new(storage_content));
    std::thread::spawn(move || update_content(storage_content.clone()));

    return storage_view;
}

#[test]
fn setup_storage_view() {
    let storage_view = setup();
    println!("Storage content: {}", storage_view.get_content().source());
    assert!(storage_view
        .get_content()
        .source()
        .contains("Storage view:"));
}

fn update_content(storage_content: Arc<Box<TextContent>>) {
    loop {
        let result = get_storage_stats();

        debug!("\n{}", result);
        storage_content.set_content(result);

        sleep(time::Duration::from_secs(1));
    }
}

fn get_storage_stats() -> String {
    let statvfs_vec: Vec<(String, statvfs::Statvfs)> = FsTab::new(Path::new(MTAB_PATH))
        .get_entries()
        .unwrap()
        .into_iter()
        .filter(|d| d.fs_spec.contains("/dev/"))
        .filter(|d| !d.fs_spec.contains("mapper"))
        .map(|path| {
            debug!("path.mountpoint: {:?}", path.mountpoint.to_str().unwrap());
            (
                path.fs_spec.clone(),
                statvfs::statvfs(path.mountpoint.to_str().unwrap()).unwrap(),
            )
        })
        .collect();

    let mut result: String = String::from(format!(
        "{:<15}{:<15}{:<15}{:<15}\n",
        "name", "total_size", "used", "used %"
    ));

    for stat in statvfs_vec {
        let total_size = stat.1.blocks() * stat.1.block_size();
        let used = total_size - stat.1.blocks_free() * stat.1.block_size();

        let (multiplier, postfix): (u64, &str) = match total_size {
            total_size if total_size < 10_u64.pow(3) => (1, "B"),
            total_size if total_size < 10_u64.pow(6) => (10_u64.pow(3), "KB"),
            total_size if total_size < 10_u64.pow(9) => (10_u64.pow(6), "MB"),
            _ => (10_u64.pow(9), "GB"),
        };

        let total_size_scaled = (total_size / multiplier) as f32;
        let used_scaled = (used / multiplier) as f32;

        result.push_str(
            format!(
                "{:<15}{:>6.2}{postfix}{:>12.2}{postfix}{:>13.2}%\n",
                stat.0,
                total_size_scaled,
                used_scaled,
                used_scaled * 100.0 / total_size_scaled
            )
            .as_str(),
        )
    }

    debug!("result: {:?}", result);
    result
}

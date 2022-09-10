use ::log::debug;
use cursive::views::{TextContent, TextView};
use procstat::ProcStat;
use procstat::CPU;
use std::collections::HashMap;
use std::sync::Arc;
use std::thread::sleep;
use std::time;

pub fn setup() -> cursive::views::TextView {
    let cpu_content = TextContent::new("");
    let cpu_view = TextView::new_with_content(cpu_content.clone());

    let content_2 = Arc::new(Box::new(cpu_content));
    std::thread::spawn(move || update_content(content_2.clone()));

    return cpu_view;
}

//println!("cpu pressure: {:#?}", procfs::CpuPressure::new());
fn update_content(storage_content: Arc<Box<TextContent>>) {
    let mut cpu_monitor = new_cpu_monitor();
    loop {
        sleep(time::Duration::from_secs(1));

        let cpu_usage: String = cpu_monitor
            .get_cpu_usage()
            .iter()
            .map(|&record| format!("CPU{}: {:.2}%\n", record.0, record.1))
            .collect();

        debug!("\n{}", cpu_usage);
        storage_content.set_content(format!("{}", cpu_usage));
    }
}

struct CpuMonitor {
    cpus: HashMap<usize, CPU>,
}

impl CpuMonitor {
    fn get_cpu_usage(&mut self) -> Vec<(usize, f32)> {
        let mut result: Vec<(usize, f32)> = Vec::new();

        let new_cpus = ProcStat::read().cpus;

        for (key, cpu) in new_cpus.iter() {
            // https://rosettacode.org/wiki/Linux_CPU_utilization
            let old_cpu = self.cpus.get(key).unwrap();

            let delta_idle_time = (cpu.idle - old_cpu.idle) as f32;
            let delta_total_time = (total_cpu_time(cpu) - total_cpu_time(old_cpu)) as f32;

            let cpu_usage = (1.0 - delta_idle_time / delta_total_time) * 100.0;

            result.push((*key, cpu_usage));
        }

        self.cpus = new_cpus;

        result.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        result
    }
}

fn new_cpu_monitor() -> CpuMonitor {
    let cpus = ProcStat::read().cpus;
    CpuMonitor { cpus }
}

fn total_cpu_time(cpu: &CPU) -> u64 {
    cpu.user
        + cpu.nice
        + cpu.system
        + cpu.idle
        + cpu.iowait
        + cpu.irq
        + cpu.softirq
        + cpu.steal
        + cpu.guest
        + cpu.guest_nice
}

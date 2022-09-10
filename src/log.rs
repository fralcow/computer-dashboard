use chrono;
use std::env;

pub fn setup_logger() -> Result<(), fern::InitError> {
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
        .level(|| -> log::LevelFilter {
            match env::var("LOG_LEVEL")
                .unwrap_or_else(|_| String::from("error"))
                .to_lowercase()
                .as_str()
            {
                "trace" => log::LevelFilter::Trace,
                "debug" => log::LevelFilter::Debug,
                "info" => log::LevelFilter::Info,
                "warn" => log::LevelFilter::Warn,
                _ => log::LevelFilter::Error,
            }
        }())
        .level_for("cursive_core", log::LevelFilter::Error)
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}

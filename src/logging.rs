use colored::Colorize;
use fern::Dispatch;
use log::LevelFilter;
use std::env;

pub fn init_logging() -> Result<(), fern::InitError> {
    // Read the desired log level from the environment variable `LOG_LEVEL`
    // Default to "info" if not set.
    let log_level = env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string());
    // Parse the environment variable into a LevelFilter. If parsing fails, default to Info.
    let level_filter = log_level
        .parse::<LevelFilter>()
        .unwrap_or(LevelFilter::Info);

    Dispatch::new()
        .level(level_filter)
        .format(|out, message, record| {
            let level = match record.level() {
                log::Level::Error => format!("{:>12}", "Error").red().bold(),
                log::Level::Warn => format!("{:>12}", "Warn").yellow().bold(),
                log::Level::Info => format!("{:>12}", "Info").purple().bold(),
                log::Level::Debug => format!("{:>12}", "Debug").white().bold(),
                log::Level::Trace => format!("{:>12}", "Trace").normal().bold(),
            };
            out.finish(format_args!("{} {}", level, message))
        })
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}

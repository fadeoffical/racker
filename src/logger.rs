use crate::cli::Cli;
use chrono::Local;
use fern::colors::Color;
use fern::Dispatch;

pub(crate) fn init(cli: &Cli) {
    let mut colors = fern::colors::ColoredLevelConfig::new();
    colors.info = Color::Green;
    colors.warn = Color::Yellow;
    colors.error = Color::Red;
    colors.debug = Color::Blue;
    colors.trace = Color::Magenta;

    Dispatch::new()
        .format(move |out, message, record| {
            let time = Local::now().format("%Y-%m-%d %H:%M:%S");
            let level = colors.color(record.level());

            let file = match record.file() {
                Some(file) => {
                    let file = file.split('/').last().unwrap_or(file);

                    &file[..file.len() - 3]
                }
                None => "?",
            };
            let module = record
                .module_path()
                .unwrap()
                .split("::")
                .next()
                .unwrap_or("?");
            let line = record.line().unwrap_or(0);

            let module = format!("{module}::{file}:{line}");

            out.finish(format_args!(
                "{}: {message}",
                format_args!("[{time}] [{level}] [{module}]")
            ))
        })
        .filter(|metadata| metadata.target().starts_with("racker"))
        .level(cli.log_level.unwrap_or(log::LevelFilter::Info))
        .chain(std::io::stdout())
        .apply()
        .unwrap();
}

use chrono::Local;
use fern::colors::Color;
use fern::Dispatch;


pub(crate) fn init() {
    let mut colors = fern::colors::ColoredLevelConfig::new();
    colors.info = Color::Green;
    colors.warn = Color::Yellow;
    colors.error = Color::Red;
    colors.debug = Color::Blue;
    colors.trace = Color::Magenta;

    Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{}] [{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                colors.color(record.level()),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .apply()
        .unwrap();
}

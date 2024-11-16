use fern::colors::{Color, ColoredLevelConfig};

pub fn logger_init() {
    let colors = ColoredLevelConfig::new()
        .trace(Color::Yellow)
        .debug(Color::Blue)
        .info(Color::Green)
        .warn(Color::Magenta)
        .error(Color::Red);

    #[cfg(debug_assertions)]
    let _res = fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{} {}] {}",
                colors.color(record.level()),
                record.target(),
                message
            ))
        })
        // .filter(|metadata| metadata.target().starts_with("tracker_"))
        .chain(std::io::stderr())
        // .chain(fern::log_file("output.log")?)
        .apply();

    #[cfg(test)]
    let _res = fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{} {}] {}",
                colors.color(record.level()),
                record.target(),
                message
            ))
        })
        // .filter(|metadata| metadata.target().starts_with("tracker_"))
        .chain(std::io::stdout())
        // .chain(fern::log_file("output.log")?)
        .apply();

    #[cfg(not(debug_assertions))]
    let _res = fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{}] {}",
                colors.color(record.level()),
                message
            ))
        })
        // .filter(|metadata| metadata.target().starts_with("tracker_backend"))
        .chain(std::io::stderr())
        // .chain(fern::log_file("output.log")?)
        .apply();
}

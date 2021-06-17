pub fn init() {
  fern::Dispatch::new()
    .format(|out, message, record| {
      out.finish(format_args!(
        "{}.{} {} : {}",
        record.target(),
        record.level(),
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
        message
      ))
    })
    .level(log::LevelFilter::Warn)
    .level_for("rmw", log::LevelFilter::Trace)
    .chain(std::io::stdout())
    // .chain(fern::log_file("output.log")?)
    .apply()
    .unwrap();
}

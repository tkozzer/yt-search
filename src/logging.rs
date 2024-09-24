use log::LevelFilter;
use log4rs::{
    append::console::ConsoleAppender,
    append::file::FileAppender,
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
};
use std::sync::Once;

static INIT: Once = Once::new();

pub fn init_logger(debug: bool) {
    INIT.call_once(|| {
        let level = if debug {
            LevelFilter::Debug
        } else {
            LevelFilter::Info
        };
        let file_path = "youtube_search.log";

        let pattern = "{d(%Y-%m-%d %H:%M:%S)} - {l} - {f}:{L} - {m}\n";

        let file_appender = FileAppender::builder()
            .encoder(Box::new(PatternEncoder::new(pattern)))
            .build(file_path)
            .unwrap();

        let console_appender = ConsoleAppender::builder()
            .encoder(Box::new(PatternEncoder::new(pattern)))
            .build();

        let config = Config::builder()
            .appender(Appender::builder().build("file", Box::new(file_appender)))
            .appender(Appender::builder().build("console", Box::new(console_appender)))
            .build(
                Root::builder()
                    .appender("file")
                    .appender("console")
                    .build(level),
            )
            .unwrap();

        if let Err(e) = log4rs::init_config(config) {
            eprintln!("Failed to initialize logger: {}", e);
        }
    });
}

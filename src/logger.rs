use std::io::Write;
pub use log::*;

use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Logger, Root};
use log4rs::encode::pattern::PatternEncoder;

pub struct Setup {
    output_file: String,
    flush: bool
}

impl Default for Setup {
    fn default() -> Self {
        Self {
            output_file: "log/requests.log".into(),
            flush: true,
        }
    }
}

pub fn init(setup: Setup) {
    // Add a new line in logger file
    if setup.flush {
        let _ = std::fs::File::create(&setup.output_file);
    } else {
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(&setup.output_file)
            .unwrap();
        file.write("\n\n>> new <<\n".as_bytes()).unwrap();
    }

    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d(%Y-%m-%d %H:%M:%S)} {({T} {h({level})}):>15.60} - {m}{n}")))
        .build();

    let requests = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} {file}:{line} {T} {l} - {m}{n}")))
        .build(&setup.output_file)
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("requests", Box::new(requests)))
        .logger(Logger::builder().build("app::backend::db", LevelFilter::Info))
        .build(
            Root::builder()
                .appender("requests")
                .appender("stdout")
                .build(LevelFilter::Info),
        )
        .unwrap();

    let _handle = log4rs::init_config(config).unwrap();
}
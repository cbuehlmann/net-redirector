use std::path::Path;

use log::LogLevelFilter;

use log4rs;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Config, Root};

pub fn init_logging(logfile_path: &str) {

    let logger_config_file = Path::new(logfile_path);

    if logfile_path.len() > 2 && logger_config_file.exists() {
        match log4rs::init_file(logger_config_file, Default::default()) {
            Err(e) => println!("Failed to initialize log4rs with configuration from {}: {}", logfile_path, e),
            _ => {},
        }
    }
    else {
        // dummy logger to console
        init_default_logger();
    }
}

/**
 * Initialize a console appender on level Warn
 */
fn init_default_logger() {

    let stdout = ConsoleAppender::builder().build();

//    let requests = FileAppender::builder()
//        .encoder(Box::new(PatternEncoder::new("{d} - {m}{n}")))
//        .build("log/requests.log")
//        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
//        .appender(Appender::builder().build("requests", Box::new(requests)))
//        .logger(Logger::builder().build("app::backend::db", LogLevelFilter::Info))
//        .logger(Logger::builder()
//        .appender("requests")
//        .additive(false)
//        .build("app::requests", LogLevelFilter::Info))
        .build(Root::builder().appender("stdout").build(LogLevelFilter::Warn))
        .unwrap();

    log4rs::init_config(config).unwrap();
}


/* Unittests
 * These tests need to be called one by one, since we cannot reset/uninitialize/clean
 * the log:: modules internal state.
 * Use `cargo test [testname]`
 */
#[cfg(test)]
mod tests {

    use std::env::current_dir;
    use logging;

    #[test]
    fn aa_corrupt_yaml_config() {
        logging::init_logging("config/log4rs-test-error.yaml");
        error!("fatal");
        warn!("warn");
        info!("info");
        debug!("debug");
        trace!("trace");
    }

    #[test]
    fn yaml_file_config() {
        // see http://docs.maidsafe.net/crust/master/log4rs/index.html for syntax
        logging::init_logging("config/log4rs-test.yaml");
        error!("running in {:?}", current_dir().unwrap().display());
        error!("fatal");
        warn!("warn");
        info!("info");
        debug!("debug");
        trace!("trace");
    }


    #[test]
    fn fallback_config() {
        logging::init_logging(".");
        error!("running in {:?}", current_dir().unwrap().display());
        error!("fatal");
        warn!("warn");
        info!("info");
    }

}

use std::path::Path;

use log::LogLevelFilter;

use log4rs;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Config, Root};

pub fn init_logging(logfile_path: &str) {

    let logger_config_file = Path::new(logfile_path);

    if logfile_path.len() > 2 && logger_config_file.exists() {
        match log4rs::init_file(logger_config_file, Default::default()) {
            Err(e) => println!("Failed to initialize form file {}: {}", logfile_path, e),
            any =>  println!("Failed to initialize form file {}: {:?}", logfile_path, any),
        }

    }
    else {
        // dummy logger to console
        init_default_logger();
    }
}

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


// Unittests
#[cfg(test)]
mod tests {

    use std::env::current_dir;
    use logging;

    #[test]
    fn default_logging() {
        logging::init_logging(".");
        error!("running in {:?}", current_dir().unwrap().display());
        error!("fatal");
        warn!("warn");
        info!("warn");
    }

    #[test]
    fn yaml_configured_logging() {
        eprintln!("loggin with configured logger");
        // see http://docs.maidsafe.net/crust/master/log4rs/index.html for syntax
        logging::init_logging("config/log4rs-test.yaml");
        error!("running in {:?}", current_dir().unwrap().display());
        error!("fatal");
        warn!("warn");
        info!("warn");
        debug!("debug");
        trace!("trace");
    }

}

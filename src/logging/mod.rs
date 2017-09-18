use std::path::Path;

use log::LogLevelFilter;
use log4rs;
use log4rs::append::console::ConsoleAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root};

pub fn init_logging(logfile_path: &str) {

    let logger_config_file = Path::new(logfile_path);

    if logfile_path.len() > 2 && logger_config_file.exists() {
        match log4rs::init_file(logger_config_file, Default::default()) {
            // this error message may appear on in the BACKTRACE :-(
            Err(e) => warn!("attempt to initialize log4rs subsystem with configuration from {}: {}", logfile_path, e),
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

    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d(%Y-%m-%d %H:%M:%S%.3f)} {T} [{t}] {m}{n}")))
        .build();

    let config = Config::builder()
        .appender(Appender::builder()
            .build("stdout", Box::new(stdout)))
        .build(Root::builder()
            .appender("stdout")
            .build(LogLevelFilter::Warn))
        .unwrap();

    log4rs::init_config(config).unwrap();
}

use std::sync::atomic::{AtomicBool, Ordering};

lazy_static! {
    static ref UNITTEST_LOGGING_SUBSYSTEM_INITIALIZED: AtomicBool = <AtomicBool>::new(false);
}

/**
 * Initialize log subsystem for unittests
 */
pub fn unittestlogger() {
    if UNITTEST_LOGGING_SUBSYSTEM_INITIALIZED.load(Ordering::SeqCst) == false {
        init_logging("config/log4rs-test.yaml");
        UNITTEST_LOGGING_SUBSYSTEM_INITIALIZED.store(true, Ordering::SeqCst);
    }
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
    use std::sync::atomic::{Ordering};

    #[test]
    fn aa_corrupt_yaml_config() {
        logging::unittestlogger();
        logging::init_logging("config/log4rs-test-error.yaml");
        error!("error");
        warn!("warn");
        info!("info");
        debug!("debug");
        trace!("trace");
    }

    #[test]
    fn yaml_file_config() {
        logging::unittestlogger();
        // see http://docs.maidsafe.net/crust/master/log4rs/index.html for syntax
        logging::init_logging("config/log4rs-test.yaml");
        error!("running in {:?}", current_dir().unwrap().display());
        error!("error");
        warn!("warn");
        info!("info");
        debug!("debug");
        trace!("trace");
    }


    #[test]
    fn fallback_config() {
        if logging::UNITTEST_LOGGING_SUBSYSTEM_INITIALIZED.load(Ordering::SeqCst) == true {
            warn!("cannot run this thes with an initialized logging subsystem, \
                run separately using 'cargo test fallback_config'");
        }
        else {
            logging::init_logging(".");
            error!("running in {:?}", current_dir().unwrap().display());
            error!("error");
            warn!("warn");
            info!("info");
            debug!("debug");
            trace!("trace");
        }
    }

}

#[macro_use]
extern crate log;
extern crate log4rs;

pub mod tcp;

use std::path::Path;

fn main() {
	// read logging parameters
	let logger_config_file = Path::new("config/log4rs.yaml");
	if logger_config_file.exists() {
		log4rs::init_file(logger_config_file, Default::default()).unwrap();
	}
	else {
		// dummy logger to console
	}

	info!("main()");

	tcp::init();

}

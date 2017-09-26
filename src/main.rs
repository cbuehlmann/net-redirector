#[macro_use]
extern crate log;
extern crate log4rs;

#[macro_use]
extern crate lazy_static;

mod logging;
mod tcp;
mod core;

fn main() {
	// read logging parameters
	logging::init_logging("config/log4rs.yaml");

	info!("starting process ");

	tcp::init();

	if false {
		logging::unittestlogger();
	}
}

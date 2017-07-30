#[macro_use]
extern crate log;
extern crate log4rs;

pub mod tcp;
mod logging;

fn main() {
	// read logging parameters
	logging::init_logging("config/log4rs.yaml");

	info!("main()");

	tcp::init();

}

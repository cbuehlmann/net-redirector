#![feature(global_allocator)]
#![feature(allocator_api)]

#[macro_use]
extern crate log;
extern crate log4rs;

#[macro_use]
extern crate lazy_static;

//#![feature(alloc_system)]
//extern crate alloc_system;


use std::heap::System;

#[global_allocator]
static ALLOCATOR: System = System;

mod logging;
mod tcp;
mod core;

use core::StoppableCore;

fn main() {
	// read logging parameters
	logging::init_logging("config/log4rs.yaml");

	info!("starting process ");

	tcp::init();

	if false {
		logging::unittestlogger();
	}

	let core = core::StoppableCore::new().unwrap();
	let timeout = std::time::Duration::from_secs(10);
	core.run_timeout(timeout).unwrap();

	info!("ending gracefully");
}

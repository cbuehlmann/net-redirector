// openwrt needs a special allocator since uClibc does not support jemalloc.
// define openwrt as simple condition
#![cfg_attr(uclibc, feature(global_allocator))]
#![cfg_attr(uclibc, feature(allocator_api))]


#[cfg(uclibc)]
use std::heap::System;

#[cfg(uclibc)]
#[global_allocator]
static ALLOCATOR: System = System;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;
extern crate log4rs;

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

#[macro_use]
extern crate log;

// the device under test
extern crate redirect;

use redirect::logging::unittestlogger;

#[test]
fn test_timeout() {
    unittestlogger();

    // this is the main event loop, powered by tokio core
    let core = redirect::core::StoppableCore::new().unwrap();
    let timeout = std::time::Duration::from_millis(5);

    match core.run_timeout(timeout) {
        Err(_) => debug!("everything ok"),
        Ok(_) => panic!("this should not happend"),
    }
}

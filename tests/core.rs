#[macro_use]
extern crate log;

// the device under test
extern crate redirect;

use redirect::logging::unittestlogger;

#[test]
fn test_timeout() {
    unittestlogger();

    warn!("creating the core");
    // this is the main event loop, powered by tokio core
    let core = redirect::core::StoppableCore::new().unwrap();
    let timeout = std::time::Duration::from_millis(5);

    warn!("running the core with timeout {}", 5);
    match core.run_timeout(timeout) {
        Err(_) => assert!(false, "should not fail"),
        Ok(x) => assert_eq!(x, 1, "expecting timeout result"),
    }
}

#[test]
fn test_signal() {
    unittestlogger();

    // this is the main event loop, powered by tokio core
    let core = redirect::core::StoppableCore::new().unwrap();
    debug!("running the core");
    core.stop();
    debug!("stopped the core");
    core.run().unwrap();
}

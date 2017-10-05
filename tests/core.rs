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
    warn!("running the core with timeout {}", 5);
    match core.run_timeout(timeout) {
        Err(_) => assert!(false, "should not fail"),
        Ok(x) => assert_eq!(x, 1, "expecting timeout result"),
    }
    debug!("core stopped");
}

#[test]
fn test_signal() {
    unittestlogger();

    // this is the main event loop, powered by tokio core
    let core = redirect::core::StoppableCore::new().unwrap();
    debug!("stopping the core");
    core.stop();
    debug!("stopped the core");
    core.run().unwrap();
}

#[test]
fn test_signal_and_timeout_stop_with_signal() {
    unittestlogger();


    // this is the main event loop, powered by tokio core
    let core = redirect::core::StoppableCore::new().unwrap();
    let timeout = std::time::Duration::from_secs(10);

    debug!("stopping the core");
    core.stop();
    debug!("stopped the core");
    let result = core.run_timeout(timeout).unwrap();
    assert_eq!(1, result, "stopped with result 1");
}

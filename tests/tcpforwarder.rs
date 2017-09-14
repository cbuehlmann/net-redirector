#[macro_use]
extern crate log;

extern crate abstract_ns;
extern crate futures;
extern crate getopts;
extern crate ns_dns_tokio;
extern crate rand;
extern crate tokio_core;
extern crate tokio_io;

// the device under test
extern crate net_interceptor;

use futures::{Future, Stream, IntoFuture};
use futures::sync::oneshot;
use futures::sync::oneshot::{Sender};
use tokio_core::net::{TcpListener};
use tokio_core::reactor::{Core, Timeout};

use std::time::Duration;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::{Arc, Barrier};
use std::thread;

use net_interceptor::logging::unittestlogger;

fn stopper(oneshot_sender_stream: Sender<i64>) -> Arc<Barrier> {
    // barrier awaits the releaser thread and the signalling one
    let barrier = Arc::new(Barrier::new(2));
    let copy_for_thread = barrier.clone();
    // the releaser thread
    thread::spawn(move || {
        debug!("waiting for end");
        copy_for_thread.wait();
        debug!("signalling end");
        // release the future blockig core.run()
        oneshot_sender_stream.send(1).unwrap();
    });
    barrier
}

#[test]
fn test_infra_connect() {
    unittestlogger();

    // this is the main event loop, powered by tokio core
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let bind_socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 12000);
    let listener = TcpListener::bind(&bind_socket_addr, &handle)
        .expect(&format!("Unable to bind to {}", &bind_socket_addr));
    warn!("Listening on {}", listener.local_addr().unwrap());

    let addr = listener.local_addr().unwrap();
    assert!(addr.port() == 12000, "should accept {}");

    let connections = listener.incoming();
    let (tx, signal) = oneshot::channel::<i64>();
    let barrier = stopper(tx);

    let acceptor = connections.for_each(move |(socket, _peer_addr)| {
        warn!("connection from {}", _peer_addr);
        barrier.wait();
        Ok(())
    });

    // register the acceptor in the reactor core
    // the map_err is a hack to satisfy connections.for_each above
    handle.spawn(acceptor.map_err(|_|()));

    let timeout = Timeout::new(std::time::Duration::from_secs(5), &handle).into_future().flatten();

    core.run(timeout.select2(signal));//.unwrap();
}

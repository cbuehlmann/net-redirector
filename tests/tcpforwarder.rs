#[macro_use]
extern crate log;

extern crate abstract_ns;
extern crate futures;
extern crate getopts;
extern crate ns_dns_tokio;
extern crate rand;
extern crate tokio_core;
extern crate tokio_io;
extern crate net2;

// the device under test
extern crate net_interceptor;

use futures::{Future, Stream, IntoFuture};
use futures::sync::oneshot;
use futures::sync::oneshot::{Sender};
use tokio_core::net::{TcpListener};
use tokio_core::reactor::{Core, Timeout};

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::{Arc, Barrier};
use std::thread;

use net_interceptor::logging::unittestlogger;

// Timeout for reactor based tests in seconds
const TIMEOUT: u64 = 5;

fn stopper(oneshot_sender_stream: Sender<i64>) -> Arc<Barrier> {
    // barrier awaits the releaser thread and the signalling one
    let barrier = Arc::new(Barrier::new(2));
    let copy_for_thread = barrier.clone();
    // the releaser thread
    let builder = thread::Builder::new()
        .name("core shutdown barrier thread".into());
    builder.spawn(move || {
        debug!("waiting for end");
        copy_for_thread.wait();
        debug!("signalling end");
        // release the future blockig core.run()
        oneshot_sender_stream.send(1).unwrap();
    }).unwrap();
    barrier
}

#[test]
fn test_infra_connect() {
    unittestlogger();

    // this is the main event loop, powered by tokio core
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let bind_socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 0);

    let listener = net2::TcpBuilder::new_v4().unwrap()
        // .reuse_address(true).unwrap()
        .bind(bind_socket_addr).unwrap()
        // 2 = allow two connection requests in the backlog
        .listen(2).unwrap();

    let listen_port = listener.local_addr().unwrap();
    let contextid = listen_port.port();
    warn!("Listening on {} in context {}", listen_port, contextid);

    let sock = TcpListener::from_listener(listener, &bind_socket_addr, &handle).unwrap();

    let (tx, signal) = oneshot::channel::<i64>();
    let barrier = stopper(tx);

    let result = sock.incoming().for_each(move |(socket, addr)| {
        //protocol.bind_connection(&handle, socket, addr, AlphaBravo::new(&path));

        warn!("connection from {} in context {}", addr, contextid);
        barrier.wait();
        Ok(())
    });

    // register the acceptor in the reactor core
    // the map_err is a hack to satisfy connections.for_each above
    handle.spawn(result.map_err(|_|()));

    let fut = Timeout::new(std::time::Duration::from_secs(TIMEOUT), &handle).into_future();
    let timeout = fut.flatten();

    let fut2 = timeout.and_then(|_| {
        assert!(false, "timeout expired waiting for connection in context {}", contextid);
        Ok(())
    });

    match core.run(fut2.select2(signal)) {
        Err(e) => { panic!("oh no!") },
        Result => { },
        _ => { },
    };

    //drop(result);
}

#[test]
fn test_two_times() {
    test_infra_connect();
    test_infra_connect();
}

#[macro_use]
extern crate log;

extern crate abstract_ns;
extern crate futures;
extern crate getopts;
extern crate ns_dns_tokio;
extern crate tokio_core;
extern crate tokio_io;
extern crate net2;

// the device under test
extern crate redirect;

use futures::{Future, Stream, IntoFuture};
use futures::sync::oneshot;
use futures::sync::oneshot::{Sender};
use tokio_core::net::{TcpListener};
use tokio_core::reactor::{Core, Timeout};

use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream};
use std::sync::{Arc, Barrier};
use std::thread;

use redirect::logging::unittestlogger;

// Timeout for reactor based tests in seconds
const TIMEOUT: u64 = 5;

fn stopper(oneshot_sender_stream: Sender<i64>) -> Arc<Barrier> {
    // barrier awaits the releaser thread and the signalling one
    let barrier = Arc::new(Barrier::new(2));
    let copy_for_thread = barrier.clone();
    // the releaser thread
    let builder = thread::Builder::new()
        .name("barrier to signal thread".into());
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
        warn!("connection from {} in context {} to {}", addr, contextid, socket.local_addr().unwrap());
        barrier.wait();
        Ok(())
    });

    // register the acceptor in the reactor core
    // the map_err is a hack to satisfy connections.for_each above
    handle.spawn(result.map_err(|_|()));

    let fut = Timeout::new(std::time::Duration::from_secs(TIMEOUT), &handle).into_future();
    let timeout = fut.flatten();

    let timeout_expired = timeout.and_then(|_| -> Result<i64, std::io::Error> {
        panic!("test timeout expired: test failed!")
    });

    // issue a connect
    let stream = TcpStream::connect(listen_port).unwrap();
    drop(stream);
//    let _ = stream.write(&[1]).unwrap(); // ignore the Result
//    let _ = stream.read(&mut [0; 128]); // ignore this too

    match core.run(timeout_expired.select2(signal)) {
        Err(_) => { panic!("oh no!"); },
        _ => { },
    };

    //drop(result);
}

#[test]
fn test_two_times() {
    test_infra_connect();
    test_infra_connect();
}

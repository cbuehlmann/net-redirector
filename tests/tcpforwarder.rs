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

use abstract_ns::Resolver;
use futures::{Future, Map, Stream, Async, Poll};
use futures::future;
use futures::sync::oneshot;
use futures::sync::oneshot::{Sender};
use ns_dns_tokio::DnsResolver;
use tokio_core::net::{TcpStream, TcpListener};
use tokio_core::reactor::{Core, Handle};
use tokio_io::{AsyncRead, io};

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::fmt::{Debug, Display};
use std::time::{Duration};
use std::sync::{Arc, Mutex, Barrier};
use std::thread;

use std::io::Error;
use std::ops::Deref;
use std::ops::DerefMut;
use std::borrow::BorrowMut;

use net_interceptor::logging::unittestlogger;

fn add_ten<F>(future: F) -> Map<F, fn(i32) -> i32>
    where F: Future<Item=i32>,
{
    error!("shutdown");
    fn add(a: i32) -> i32 { a + 10 }
    future.map(add)
}

pub struct AbortCondition {
    condition: bool,
    reactor_handle: futures::task::Task,
}

impl AbortCondition {

    fn set(&mut self) {
        debug!("abort condition met");
        self.condition = true;
        self.reactor_handle.unpark();
    }

}

impl Future for AbortCondition {

    type Item = ();
    type Error = Error;

    fn poll(&mut self) -> Poll<(), Error> {
        if  self.condition {
            Ok(Async::Ready(()))
        } else {
            self.reactor_handle = futures::task::park();
            Ok(Async::NotReady)
        }
    }

}

fn stopper(sender: Sender<i64>) -> Arc<Barrier> {
    let counting = Arc::new(Barrier::new(2));
    let val_for_thread = counting.clone();
    thread::spawn(move || {
        warn!("waiting for end");
        val_for_thread.wait();
        warn!("signalling end");
        sender.send(1).unwrap();
    });
    counting
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

//    let mut abort = AbortCondition { condition: false, reactor_handle: futures::task::park() };

    let connections = listener.incoming();
    let (tx, signal) = oneshot::channel::<i64>();

    let mut barrier = stopper(tx);
    warn!("stopper activated");

    let server = connections.for_each(move |(socket, _peer_addr)| {
        warn!("println connection from {}", _peer_addr);
        warn!("connection received");
        barrier.wait();
        Ok(())
    });

    let wait = core.handle().spawn(server.map_err(|_|()));

    fn abortCondition() -> futures::Poll<(), std::io::Error> {
        warn!("poll");
        if true {
            Ok(Async::Ready(()))
        }
        else {
            futures::task::park().unpark();
            Ok(Async::NotReady)
        }
    }

    //let abort = futures::future::poll_fn(abortCondition);

    //let timeout = tokio_core::reactor::Timeout::new_at(std::time::Duration::seconds(1));
    // run server, stop on panic
//    core.run(server.select2(timeout)).unwrap();
//    core.run(abort).unwrap();
    // core.spawn(server);
//    core.run(server.select(abort)).unwrap();
    core.run(signal).unwrap();
}

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
use std::sync::{Arc, Mutex};
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

#[derive(Copy, Clone)]
pub struct Releaser {
    block: *const AbortCondition
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

pub struct Stopper {
    sender: Sender<i64>,
}

impl Stopper {

    pub fn stop(&mut self) {
        //self.sender.send(1).unwrap();
    }

}


/*impl Copy for Abort {
    fn clone(&self) -> Abort {
        *self
    }
}
*/
// Here we'll express the handling of `client` and return it as a future
// to be spawned onto the event loop.
/*fn process(client: TcpStream) -> Box<Future<Item = (i64), Error = error>> {
    warn!("received");
    Box::new(futures::future::ok(0));
}
*/
fn blank() -> Result<(), std::io::Error> {
    warn!("blank");
    Ok(())
}

fn direct(stopper: & mut Option<Sender<i64>>) -> Result<(), std::io::Error> {
    warn!("connection received");
//    let mut s = Box::borrow_mut(stopper);
    let mut s : Sender<i64> = stopper.unwrap();
    s.send(1).unwrap();
    Ok(())
}


fn stopper(sender: Sender<i64>) -> Arc<Mutex> {
    let counting = Arc::new(Mutex::new(0));
    thread::spawn(move |sender| {
        debug!("waiting for end")
        let mut num = sender.lock().unwrap();
        debug!("signalling end");
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

//    let stopper = Arc::new(tx);
    let mut boxed = Some(tx);
    {

    //        let copy = Arc::clone(&stopper);
            //let server = connections.for_each(move |(socket, _peer_addr)| fx(&stopper));
        let server = connections.for_each(move |(socket, _peer_addr)| direct(& mut boxed));
        let wait = core.handle().spawn(server.map_err(|_|()));

    }

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

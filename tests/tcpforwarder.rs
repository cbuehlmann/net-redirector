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
use futures::{Future, Map, Stream, Async};
use futures::future;
use ns_dns_tokio::DnsResolver;
use tokio_core::net::{TcpStream, TcpListener};
use tokio_core::reactor::Core;
use tokio_io::{AsyncRead, io};

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::fmt::{Debug, Display};

use net_interceptor::logging::unittestlogger;

fn add_ten<F>(future: F) -> Map<F, fn(i32) -> i32>
    where F: Future<Item=i32>,
{
    error!("shutdown");
    fn add(a: i32) -> i32 { a + 10 }
    future.map(add)
}


// Here we'll express the handling of `client` and return it as a future
// to be spawned onto the event loop.
/*fn process(client: TcpStream) -> Box<Future<Item = (i64), Error = error>> {
    warn!("received");
    Box::new(futures::future::ok(0));
}
*/
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
    let server = connections.for_each(move |(socket, _peer_addr)| {
        warn!("connection received");
        /*let (writer, reader) = socket.framed(LineCodec).split();
        let service = s.new_service()?;

        let responses = reader.and_then(move |req| service.call(req));
        let server = writer.send_all(responses)
            .then(|_| Ok(()));
        handle.spawn(server);
        */
        Ok(())
    });

    fn abortCondition() -> futures::Poll<(), std::io::Error> {
        warn!("poll");
        if true {
            Ok(Async::Ready(()))
        }
        else {
            Ok(Async::NotReady)
        }
    }

    let abort = futures::future::poll_fn(abortCondition);

    // run server, stop on panic
    core.run(server).unwrap();
//    core.run(server.select2(abort)).unwrap();
}

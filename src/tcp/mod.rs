extern crate abstract_ns;
extern crate futures;
extern crate getopts;
extern crate ns_dns_tokio;
extern crate rand;
extern crate tokio_core;
extern crate tokio_io;
extern crate log;

use self::abstract_ns::Resolver;
use self::futures::{Future, Stream};
use self::futures::future;
use self::ns_dns_tokio::DnsResolver;
use self::tokio_core::net::{TcpStream, TcpListener};
use self::tokio_core::reactor::Core;
use self::tokio_io::{AsyncRead, io};

use std::net::{SocketAddr};
use std::fmt::{Debug, Display};

pub fn init() {
    debug!("tcp module loaded");
}

pub fn forward(bind_ip: &str, local_port: i32, remote_host: &str, remote_port: i32) {
    //this is the main event loop, powered by tokio core
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    //listen on the specified IP and port
    let bind_addr = format!("{}:{}", bind_ip, local_port);
    let bind_sock = bind_addr.parse().unwrap();
    let listener = TcpListener::bind(&bind_sock, &handle)
        .expect(&format!("Unable to bind to {}", &bind_addr));
    println!("Listening on {}", listener.local_addr().unwrap());

    //we have either been provided an IP address or a host name
    //instead of trying to check its format, just trying creating a SocketAddr from it
    let parse_result = format!("{}:{}", remote_host, remote_port).parse::<SocketAddr>();
    let server = future::result(parse_result)
        .or_else(|_| {
            //it's a hostname; we're going to need to resolve it
            //create an async dns resolver
            let resolver = DnsResolver::system_config(&handle).unwrap();

            resolver.resolve(&format!("{}:{}", remote_host, remote_port))
                .map(move |resolved| {
                    resolved.pick_one()
                        .expect(&format!("No valid IP addresses for target {}", remote_host))
                })
                .map_err(|err| println!("{:?}", err))
        })
        .and_then(|remote_addr| {
            println!("Resolved {}:{} to {}",
                     remote_host,
                     remote_port,
                     remote_addr);

            let remote_addr = remote_addr.clone();
            let handle = handle.clone();
            listener.incoming()
                .for_each(move |(client, client_addr)| {
                    println!("New connection from {}", client_addr);

                    //establish connection to upstream for each incoming client connection
                    let handle = handle.clone();
                    TcpStream::connect(&remote_addr, &handle).and_then(move |remote| {
                        let (client_recv, client_send) = client.split();
                        let (remote_recv, remote_send) = remote.split();

                        let remote_bytes_copied = io::copy(remote_recv, client_send);
                        let client_bytes_copied = io::copy(client_recv, remote_send);

                        fn error_handler<T, V>(err: T, client_addr: V)
                            where T: Debug,
                                  V: Display
                        {
                            warn!("Error writing from upstream server to remote client {}!",
                                     client_addr);
                            warn!("{:?}", err);
                            ()
                        };

                        let client_addr_clone = client_addr.clone();
                        let async1 = remote_bytes_copied.map(move |(count, _, _)| {
                                debug!("Transferred {} bytes from upstream server to \
                                               remote client {}",
                                              count,
                                              client_addr_clone )
                            })
                            .map_err(move |err| error_handler(err, client_addr_clone));

                        let client_addr_clone = client_addr;
                        let async2 = client_bytes_copied.map(move |(count, _, _)| {
                                debug!("Transferred {} bytes from remote client {} to \
                                               upstream server",
                                              count,
                                              client_addr_clone)
                            })
                            .map_err(move |err| error_handler(err, client_addr_clone));

                        handle.spawn(async1);
                        handle.spawn(async2);

                        Ok(())
                    })
                })
                .map_err(|err| warn!("{:?}", err))
        });

    core.run(server).unwrap();
}

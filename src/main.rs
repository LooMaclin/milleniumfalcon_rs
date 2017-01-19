//#![deny(warnings)]
#![feature(conservative_impl_trait)]
extern crate futures;
extern crate hyper;
extern crate pretty_env_logger;
#[macro_use]
extern crate log;
extern crate net2;
extern crate tokio_core;
extern crate scheduler;

#[macro_use]
extern crate serde_derive;

extern crate serde_json;
extern crate fringe;

use fringe::{OsStack, Generator};

use hyper::{Get, Post, StatusCode};
use hyper::header::ContentLength;
use hyper::server::{Server, Service, Request, Response};
use std::borrow::Cow;
use futures::Future;
use futures::future::{ok};

use futures::Stream;
use std::io::{self, Write};
use net2::TcpBuilder;
use net2::unix::UnixTcpBuilderExt;
use tokio_core::net::TcpListener;

#[derive(Serialize, Deserialize, Debug)]
struct Wrapper<'a> {
    count: u32,
    next: Cow<'a, str>,
    previous: Option<u32>,
    results: Vec<Planet<'a>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Planet<'a> {
    name: Cow<'a, str>,
    rotation_period: Cow<'a, str>,
    orbital_period: Cow<'a, str>,
    diameter: Cow<'a, str>,
    climate: Cow<'a, str>,
    gravity: Cow<'a, str>,
    terrain: Cow<'a, str>,
    surface_water: Cow<'a, str>,
    population: Cow<'a, str>,
    residents: Vec<Cow<'a, str>>,
    films: Vec<Cow<'a, str>>,
    created: Cow<'a, str>,
    edited: Cow<'a, str>,
    url: Cow<'a, str>,
}


static INDEX: &'static [u8] = b"Try POST /echo";

#[derive(Clone, Copy)]
struct Echo;

impl<'a> Default for Wrapper<'a> {
    fn default() -> Wrapper<'a> {
        Wrapper {
            count: 1,
            next: Cow::Borrowed("next"),
            previous: Some(1),
            results: vec![],
        }
    }
}

impl Service for Echo {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item = self::Response, Error = hyper::error::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        match (req.method(), req.path()) {
            (&Post, "/planets.json") => {
                let body = Vec::new();
                Box::new(req.body()
                    .fold(body, |mut body, chunk| {
                        body.extend_from_slice(&chunk);
                        Ok::<Vec<u8>, hyper::Error>(body)
                    })
                    .map(|full_body| {
                        let deserialized_body : Wrapper =
                            match serde_json::from_slice(&full_body) {
                                Ok(result) => {
                                    result
                                },
                                Err(ref error) => {
                                    println!("Error: {}", error);
                                    Wrapper::default()
                                }
                            };
                        let serialized_body = serde_json::to_string(&deserialized_body).unwrap();
                        Response::new()
                            .with_header(ContentLength(serialized_body.len() as u64))
                            .with_body(serialized_body)
                    }))
            },
            _ => {
                Box::new(ok(Response::new()))
            }
        }
    }

}


fn main() {
    use std::net::SocketAddr;
    pretty_env_logger::init().unwrap();
    let addr: SocketAddr = "127.0.0.1:1337".parse().unwrap();
    let mut threads = vec![];
    for i in 0..11 {
        use std::thread;
        let i = i;
        let handle = thread::spawn(move|| {
            let (listening, server) = Server::standalone(|tokio| {
                let listener = TcpBuilder::new_v4()?.reuse_port(true)?.bind(addr)?.listen(10000)?;
                let addr = try!(listener.local_addr());
                let listener = try!(TcpListener::from_listener(listener, &addr, tokio));
                Server::new(listener.incoming(), addr).handle(|| Ok(Echo), tokio)
            }).unwrap();
            println!("Listening {} on http://{}", i, listening);
            server.run();
        });
        threads.push(handle);
    }
    for t in threads {
        t.join().unwrap();
    }
}
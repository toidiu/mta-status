extern crate futures;
extern crate hyper;
extern crate net2;
extern crate tokio_core;
extern crate serde;
extern crate serde_json;


use futures::{Future, Poll, task, Async, Stream};

use hyper::server::{Http};

use net2::unix::UnixTcpBuilderExt;

use std::{thread};
use std::sync::Arc;
use std::net::SocketAddr;
use std::sync::Mutex;

use tokio_core::reactor::Core;
use tokio_core::net::{TcpListener, TcpStream};

pub mod my_service;

/// Source:
/// https://blog.guillaume-gomez.fr/articles/2017-02-22+Rust+asynchronous+HTTP+server+with+tokio+and+hyper

struct ThreadData {
    entries: Vec<(TcpStream, SocketAddr)>,
    task: Option<task::Task>,
}

impl ThreadData {
    pub fn new() -> Arc<Mutex<ThreadData>> {
        Arc::new(Mutex::new(ThreadData {
            entries: Vec::new(),
            task: None,
        }))
    }
}

pub fn start_better_server(addr: &str, num_thread: usize) {

    //// some init
    let addr: SocketAddr = addr.parse().unwrap();
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    //// have a vec of thread data that can handle the incoming request
    let threads = make_service_threads(num_thread);


    //// build the listener
    let listener = net2::TcpBuilder::new_v4()
        .unwrap()
        .reuse_port(true)
        .unwrap()
        .bind(&addr)
        .unwrap()
        .listen(128)
        .unwrap();
    let listener = TcpListener::from_listener(listener, &addr, &handle).unwrap();


    let mut counter = 0;
    //// on main thread have a core run
    core.run(listener.incoming().for_each(|(socket, addr)| {

        //// for each incoming request add the data to thread round-robin style
        let ref entry = threads[counter];
        {
            let mut entry = entry.lock().unwrap();
            entry.entries.push((socket, addr));

            if let Some(task) = entry.task.take() {
                task.notify();
            }
        }
        counter += 1;
        if counter >= num_thread {
            counter = 0;
        }
        Ok(())
    })).unwrap();
}

struct Foo<F: Fn()> {
    c: F,
}



// This is where the magic occurs, our object needs to be polled anytime it receives a new client!
impl<F: Fn()> Future for Foo<F> {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        (self.c)();
        // If we returned `Async::Ready`, this function will never be called again so let's
        // avoid it.
        Ok(Async::NotReady)
    }
}

fn make_service_threads(num_thread: usize) -> Vec<Arc<Mutex<ThreadData>>> {
    let mut threads = Vec::new();
    for id in 0..num_thread {
        let data = ThreadData::new();
        threads.push(data.clone());

        thread::spawn(move || {
            let mut core = Core::new().unwrap();
            let handle = core.handle();
            let protocol = Http::new();

            core.run(Foo {
                c: || {
                    let mut data = data.lock().unwrap();
                    for (socket, addr) in data.entries.drain(..) {
                        info!("here==== {}", id);
                        let my_svr = my_service::GetStatus::new(handle.clone());
                        protocol.bind_connection(
                            &handle,
                            socket,
                            addr,
                            my_svr,
                        );
                    }
                    // We reset the task in our `ThreadData` in case we switched context.
                    data.task = Some(task::current());

                },
            }).unwrap();

        });
    }
    threads
}









// #[derive(Clone, Copy)]
// struct Echo {
//     id: usize,
// }

// impl Service for Echo {
//     type Request = Request;
//     type Response = Response;
//     type Error = hyper::Error;
//     type Future = FutureResult<Response, hyper::Error>;

//     fn call(&self, req: Request) -> Self::Future {
//         futures::future::ok(match (req.method(), req.path()) {
//             (&Get, "/data") => {
//                 // let b = cpu_intensive_work().into_bytes();

//                 // for x in 0..10000000 {
//                 //     let y = format!("Value: {}", x);
//                 // }
//                 let sleep_time = time::Duration::from_secs(1);
//                 thread::sleep(sleep_time);
//                 println!("here==== {}", self.id);
//                 let b = "hi";

//                 Response::new()
//                     .with_header(ContentLength(b.len() as u64))
//                     .with_body(b)
//             }
//             _ => Response::new().with_status(StatusCode::NotFound),
//         })
//     }
// }

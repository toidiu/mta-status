extern crate futures;
extern crate hyper;
extern crate net2;
extern crate tokio_core;
extern crate serde;
extern crate serde_json;


use futures::{Future, Poll, task, Async, Stream};

use hyper::server::Http;

use net2::unix::UnixTcpBuilderExt;

use std::thread;
use std::sync::Arc;
use std::net::SocketAddr;
use std::sync::Mutex;

use tokio_core::reactor::Core;
use tokio_core::net::{TcpListener, TcpStream};

pub mod my_service;

/// Source:
/// https://blog.guillaume-gomez.fr/
///     articles/2017-02-22+Rust+asynchronous+HTTP+server+with+tokio+and+hyper

pub fn start_server(addr: &str, num_thread: usize) {

    //// some init
    let addr: SocketAddr = addr.parse().unwrap();
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    //// have a vec of thread data that can handle the incoming request
    let threads = make_worker_threads(num_thread);


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


    //// on main thread have a core run
    let mut counter = 0;
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

fn make_worker_threads(num_thread: usize) -> Vec<Arc<Mutex<WorkerThread>>> {
    let mut threads = Vec::new();
    for id in 0..num_thread {
        let data = WorkerThread::new();
        threads.push(data.clone());

        thread::spawn(move || {
            let mut core = Core::new().unwrap();
            let handle = core.handle();
            let protocol = Http::new();

            core.run(Qlosure {
                c: || {
                    let mut data = data.lock().unwrap();
                    for (socket, addr) in data.entries.drain(..) {
                        warn!("=====running on thread===== {}", id);
                        let my_svr = my_service::GetStatus::new(handle.clone());
                        protocol.bind_connection(&handle, socket, addr, my_svr);
                    }
                    // We reset the task in our `WorkerThread` in case we switched context.
                    data.task = Some(task::current());

                },
            }).unwrap();

        });
    }
    threads
}


struct WorkerThread {
    entries: Vec<(TcpStream, SocketAddr)>,
    task: Option<task::Task>,
}

impl WorkerThread {
    pub fn new() -> Arc<Mutex<WorkerThread>> {
        Arc::new(Mutex::new(WorkerThread {
            entries: Vec::new(),
            task: None,
        }))
    }
}

struct Qlosure<F: Fn()> {
    c: F,
}

// This is where the magic occurs, our object needs to be polled anytime it receives a new client!
impl<F: Fn()> Future for Qlosure<F> {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        (self.c)();
        // If we returned `Async::Ready`, this function will never be called again so let's
        // avoid it.
        Ok(Async::NotReady)
    }
}

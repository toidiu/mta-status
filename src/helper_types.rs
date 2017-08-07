extern crate hyper;
extern crate futures;



type DataFut = futures::Future<Item = String, Error = hyper::Error>;


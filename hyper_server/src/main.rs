extern crate hyper;
use hyper::net::HttpListener;
use hyper::net::NetworkListener;
use hyper::server::{Request,Response,Handler,Fresh};
use std::io::Read;
fn callback(mut req:Request, res:Response<Fresh>){
    let mut data:Vec<u8> = vec![];
    println!("---{:?}----",req.read_to_end(&mut data));
    let data = String::from_utf8(data).unwrap();
    println!("----recive data = {:?}",data);
}


fn main() {

    let server = hyper::Server::http("127.0.0.1:1337").unwrap().handle(callback);
//    server.handle_threads()
    println!("Hello, world!");

}

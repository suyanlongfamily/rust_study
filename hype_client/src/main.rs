extern crate hyper;
fn main() {
    println!("Hello, world!");
    let client = hyper::client::Client::new();
    let res = client
        .post("http://127.0.0.1:1337")
        .body("{\"jsonrpc\":2.0,\"method\":\"cita_blockHeight\",\"params\":[],\"id\":2}")
        .send();
    println!("----{:?}---", res.ok().unwrap());
}

mod server;
mod config;
use std::str::FromStr;

fn main() {
    let root = String::from_str("C:\\Users\\afanty\\Desktop\\webTest").unwrap();
    let host = String::from_str("127.0.0.1").unwrap();
    let c = config::RockConfig::new(root, host, 9999);
    let server = server::Rock::new(c);
    server.start();
}
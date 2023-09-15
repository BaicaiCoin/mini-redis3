#![feature(impl_trait_in_assoc_type)]

use std::net::SocketAddr;

use volo_mini_redis::P;

#[volo::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let addr: SocketAddr = "127.0.0.1:9090".parse().unwrap();
    let addr = volo::net::Address::from(addr);

    let p = P {};
    volo_gen::mini::redis::RedisProxyServer::new(p)
        .run(addr)
        .await
        .unwrap();
}
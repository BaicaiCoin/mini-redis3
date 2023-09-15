use lazy_static::lazy_static;
use std::{net::SocketAddr, io, process};

lazy_static! {
    static ref CLIENT: volo_gen::mini::redis::RedisProxyClient = {
        let addr: SocketAddr = "127.0.0.1:9090".parse().unwrap();
        volo_gen::mini::redis::RedisProxyClientBuilder::new("volo-mini-redis")
            .address(addr)
            .build()
    };
}

#[volo::main]
async fn main() {
    tracing_subscriber::fmt::init();
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.strip_suffix("\n").unwrap().to_string();
        let string_vec: Vec<String> = input.split(' ').map(|str| str.to_string()).collect();
        let mut req = volo_gen::mini::redis::RedisRequest {
            key: None,
            value: None,
            request_type: volo_gen::mini::redis::RequestType::Illegal,
        };
        if string_vec[0] == "PING" {
            req = volo_gen::mini::redis::RedisRequest {
                key: None,
                value: None,
                request_type: volo_gen::mini::redis::RequestType::Ping,
            }
        }
        else if string_vec[0] == "SET" && string_vec.len() == 3 {
            req = volo_gen::mini::redis::RedisRequest {
                key: Some(vec![string_vec.get(1).unwrap().clone().into()]),
                value: Some(string_vec.get(2).unwrap().clone().into()),
                request_type: volo_gen::mini::redis::RequestType::Set,
            }
        }
        else if string_vec[0] == "GET" && string_vec.len() == 2 {
            req = volo_gen::mini::redis::RedisRequest {
                key: Some(vec![string_vec.get(1).unwrap().clone().into()]),
                value: None,
                request_type: volo_gen::mini::redis::RequestType::Get,
            }
        }
        else if string_vec[0] == "DEL" {
            let mut tmp = vec![];
            for i in 1..string_vec.len() {
                tmp.push(string_vec.get(i).unwrap().clone().into());
            }
            req = volo_gen::mini::redis::RedisRequest {
                key: Some(tmp),
                value: None,
                request_type: volo_gen::mini::redis::RequestType::Del,
            }
        }
        else if string_vec[0] == "exit" && string_vec.len() == 1 {
            process::exit(0);
        }
        let resp = CLIENT.redis_receive(req).await;
        match resp {
            Ok(info) => {
                if let Some(value) = info.value {
                    tracing::info!("Value is {:?}", value);
                } else {
                    // 处理 value 为 None 的情况
                    tracing::info!("Value is None");
                }
            }
            Err(e) => tracing::error!("{:?}", e),
        }
    }
}

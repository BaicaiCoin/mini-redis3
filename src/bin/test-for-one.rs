use std::process::{Command, Child};
use std::thread;
use std::time::Duration;
use lazy_static::lazy_static;
use std::net::SocketAddr;
use volo_mini_redis::LogLayer;
use std::fs::OpenOptions;
use std::io::Write;
use std::env;
use config::{Config, FileFormat};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct RedisConfig {
    main_node: NodeConfig,
    slave_nodes: Vec<NodeConfig>,
}

#[derive(Debug, Deserialize)]
struct NodeConfig {
    address: SocketAddr,
    is_main: bool,
}

impl NodeConfig {
    fn is_main(&self) -> bool {
        self.is_main
    }
}

fn start_custom_server() -> Child {
    // 启动你自己的服务器的命令
    let mut cmd = Command::new("server");
    
    // cmd.arg("--port").arg("8080");
    // 添加服务器参数和配置，例如端口号、配置文件等
    // cmd.arg("set").arg("key1").arg("value1");
    // let child = cmd.env("RUST_LOG", "TRACE").spawn().expect("Failed to start custom server");
    let child = cmd.spawn().expect("Failed to start custom server");
    // println!("{}",child.id());
    println!("server begins");
    // 等待一些时间以确保服务器启动完成
    thread::sleep(Duration::from_secs(1)); // 根据需要调整等待时间
    child
}

fn stop_custom_server(child: &mut Child) {
    // 终止自定义服务器
    // thread::sleep(Duration::from_secs(1));
    let _ = child.kill();
    println!("server stops");
    // 等待一些时间以确保服务器关闭
    thread::sleep(Duration::from_secs(3)); // 根据需要调整等待时间
}

#[volo::main]
async fn main() {
    // nothing();
    let file_path = "redis.aof";
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true) // 截断文件
        .create(true)
        .open(file_path)
        .expect("Failed to open file");
    // 清空文件内容，这里写入一个空字符串
    file.write_all(b"").expect("Failed to clear file");

    let mut settings = Config::new();
    settings.merge(config::File::new("config", FileFormat::Toml).required(true)).unwrap();
    let redis_config: RedisConfig = settings.try_into().unwrap();
    let main_node = redis_config.main_node;
    let slave_nodes = redis_config.slave_nodes;
    
// 启动自定义服务器
    let current_exe = env::current_exe().expect("Failed to get current executable path");

    // 计算 starter.exe 相对于当前可执行文件的路径
    let starter_path = current_exe
        .parent()
        .expect("Failed to get parent directory")
        .join("starter");
    
    let mut master = Command::new(starter_path)
    .arg(main_node.address.to_string())
    .arg(main_node.is_main.to_string())
    .spawn()
    .expect("Failed to open master starter");

    let current_exe = env::current_exe().expect("Failed to get current executable path");
    lazy_static! {
        static ref CLIENT1: volo_gen::mini::redis::RedisProxyClient = {
            let addr: SocketAddr = "127.0.0.1:9090".parse().unwrap();
            volo_gen::mini::redis::RedisProxyClientBuilder::new("volo-mini-redis")
                .address(addr)
                .build()
        };
    }
    tracing_subscriber::fmt::init();
    let mut inputs = Vec::new();
    inputs.push("SET key3 value3");
    inputs.push("SET key2 value2");
    inputs.push("SET key1 value1");
    loop {
        // let mut input = String::new();
        // io::stdin().read_line(&mut input).unwrap();
        let input = inputs.pop();
        if let Some(value) = input{
            let string_vec: Vec<String> = value.split(' ').map(|str| str.to_string()).collect();
            println!("{}",value);
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
                // process::exit(0);
                break;
            }
            let resp = CLIENT1.redis_receive(req).await;
            match resp {
                Ok(info) => tracing::info!("{:?}", info.value.unwrap()),
                Err(e) => tracing::error!("{:?}", e),
            }
        }else{
            break;
        }
        thread::sleep(Duration::from_secs(2));
    }
    let _ = master.kill();

    // 启动自定义服务器
    let current_exe = env::current_exe().expect("Failed to get current executable path");

    // 计算 starter.exe 相对于当前可执行文件的路径
    let starter_path = current_exe
        .parent()
        .expect("Failed to get parent directory")
        .join("starter");
    
    let mut master = Command::new(starter_path)
    .arg(main_node.address.to_string())
    .arg(main_node.is_main.to_string())
    .spawn()
    .expect("Failed to open master starter");

    let current_exe = env::current_exe().expect("Failed to get current executable path");
    lazy_static! {
        static ref CLIENT2: volo_gen::mini::redis::RedisProxyClient = {
            let addr: SocketAddr = "127.0.0.1:9090".parse().unwrap();
            volo_gen::mini::redis::RedisProxyClientBuilder::new("volo-mini-redis")
                .address(addr)
                .build()
        };
    }
    tracing_subscriber::fmt::init();
    
    let mut inputs:Vec<&str> = Vec::new();
    inputs.push("GET key3");
    inputs.push("GET key2");
    inputs.push("GET key1");
    loop {
        // let mut input = String::new();
        // io::stdin().read_line(&mut input).unwrap();
        let input = inputs.pop();
        if let Some(value) = input{
            let string_vec: Vec<String> = value.split(' ').map(|str| str.to_string()).collect();
            println!("{}",value);
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
                // process::exit(0);
                break;
            }
            let resp = CLIENT2.redis_receive(req).await;
            match resp {
                Ok(info) => tracing::info!("{:?}", info.value.unwrap()),
                Err(e) => tracing::error!("{:?}", e),
            }
        }else{
            break;
        }
        thread::sleep(Duration::from_secs(2));
    }
    let _ = master.kill();
}


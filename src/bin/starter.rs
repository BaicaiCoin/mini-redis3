use std::process::{Command, Child};
use std::{net::SocketAddr, vec::Vec};
use config::{Config, FileFormat};
use serde::Deserialize;
use std::env;

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

fn main(){

    let mut settings = Config::new();
    settings.merge(config::File::new("config", FileFormat::Toml).required(true)).unwrap();
    let redis_config: RedisConfig = settings.try_into().unwrap();
    let main_node = redis_config.main_node;
    let slave_nodes = redis_config.slave_nodes;
    
    let current_exe = env::current_exe().expect("Failed to get current executable path");

    // 计算 server.exe 相对于当前可执行文件的路径
    let server_path = current_exe
        .parent()
        .expect("Failed to get parent directory")
        .join("server.exe");
    
    let mut master = Command::new(server_path)
    .arg(main_node.address.to_string())
    .arg(main_node.is_main.to_string())
    .spawn()
    .expect("Failed to open master server");

    
    let mut slaves:Vec<Child> = Vec::new();
    for node in slave_nodes{
        let server_path = current_exe
        .parent()
        .expect("Failed to get parent directory")
        .join("server.exe");
        let mut slave = Command::new(server_path)
        .arg(node.address.to_string())
        .arg(node.is_main.to_string())
        .spawn()
        .expect("Failed to open master server");
        slaves.push(slave);
    }

    let proxy_path = current_exe
        .parent()
        .expect("Failed to get parent directory")
        .join("proxy.exe");
    let mut proxy = Command::new(proxy_path)
    .spawn()
    .expect("Failed to open proxy server");
}
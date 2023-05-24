use std::env;
use std::net::{IpAddr, TcpStream};
use tokio::runtime::Runtime;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("You must follow these structure: portscanner <IPADDRESS> <PORTS>");
        return;
    }

    let ip: IpAddr = match args[1].parse() {
        Ok(ip) => ip,
        Err(_) => {
            println!("Invalid IP Address");
            return;
        }
    };

    let ports: Vec<u16> = match args[2].parse() {
        Ok(ports) => ports,
        Err(_) => {
            println!("Invalid ports");
            return;
        }
    };

    let rt = Runtime::new().unwrap();

    rt.block_on(async {
        for port in ports {
            let ip_clone = ip.clone();

            tokio::spawn(async move {
                match TcpStream::connect((ip_clone, port)).await {
                    Ok(_) => println!("Port {} open", port),
                    Err(_) => println!("Port {} closed", port),
                }
            });
        }
    })
}
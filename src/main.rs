use std::env;
use std::net::{IpAddr, TcpStream};
use tokio::runtime::Runtime;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("You must follow this structure: portscanner <IPADDRESS> <PORTS>");
        return;
    }

    let ip: IpAddr = match args[1].parse() {
        Ok(ip) => ip,
        Err(_) => {
            println!("Invalid IP Address");
            return;
        }
    };

    let ports: Result<Vec<u16>, _> = args[2]
        .split(',')
        .map(str::parse)
        .collect();

    let ports: Vec<u16> = match ports {
        Ok(ports) => ports,
        Err(_) => {
            println!("Invalid ports");
            return;
        }
    };

    let rt = Runtime::new().unwrap();

    for port in ports {
        let ip_clone = ip.clone();

        rt.spawn(async move {
            let result = tokio::task::spawn_blocking(move || {
                TcpStream::connect((ip_clone, port))
            })
            .await;

            match result {
                Ok(Ok(_)) => println!("Port {} open", port),
                _ => println!("Port {} closed", port),
            }
        });
    }

    rt.shutdown_background();
}
use std::{thread, env, process, str, panic, io};
use std::io::prelude::*;
use std::net::TcpStream;
use serde::{Deserialize, Serialize};
// The First attempt at Rust, less go--

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Message {
    user: String,
    msg: String,
}

fn help() {
    println!("anonchat-cli is a client for anonchat servers with V2 Protocol.\nUsage: anonchat-cli <ip> [name]\n\tip - IP of the server\n\tname - Name of your client. 'Anon' by default.\n----- Hello there!");
    process::exit(1);
}

fn read_msgs(stream_orig: TcpStream) {
    let mut _data: String = String::from("No data");
    loop {
        let stream = stream_orig.try_clone()
            .expect("Clone failed!"); // Clone to avoid errors

        let mut buffer = [0; 2048];

        let mut handle = stream.take(2048);
        let handler = handle.read(&mut buffer);
        if handler.is_ok() {
            let _data = match str::from_utf8(&buffer) {
                Ok(v) => v,
                Err(e) => "Invalid UTF-8 sequence",
            };
            let mut _data = _data.trim().to_string();
            let _data = _data.trim_matches(char::from(0)).to_string();

            let result = panic::catch_unwind(|| {
                let json: serde_json::Value = serde_json::from_str(_data.as_str()).expect("JSON was not well-formatted");

                let user = json["user"].to_string(); // Convert to string name and msg, and remove "
                let user = user.trim_matches('"');
                let msg = json["msg"].to_string();
                let msg = msg.trim_matches('"');

                println!("<{}> {}", user, msg);
            });
            if result.is_err(){
                println!("[CLIENT] Message cannot be readed :(");
            };
        } else {
            println!("Error while reading data!")
        }

        _data = String::from("No data");

    }
}

fn main() {
    let args = args_parse();
    let ip: String = args.0;
    let name: String = args.1;

    let stream: TcpStream = TcpStream::connect(ip)
        .expect("No connection from server/Invalid adress...");
    
    stream.set_nodelay(true).expect("set_nodelay call failed");

    let read_stream = stream.try_clone()
        .expect("Clone failed!"); // Clone to avoid errors
    let _handle_read = thread::spawn(|| read_msgs(read_stream) );
    println!("// Connected!");
    loop {
        let mut stream_clone = stream.try_clone()
            .expect("Clone failed!"); // Clone to avoid errors

        let mut message = String::new();
        io::stdin()
        .read_line(&mut message)
        .expect("Failed to read line");

        let tmsg = Message {
            user: name.clone(),
            msg: message.trim().to_string(),
        };

        let result = stream_clone.write(serde_json::to_string(&tmsg).unwrap().as_bytes());
        if result.is_err() {
            println!("[CLIENT] Cannot write message!");
        }
    }
}

fn args_parse() -> (String, String){
    let args: Vec<_> = env::args().collect();
    let mut ip: String = String::from("None");
    let mut name: String = String::from("Anon");

    if args.len() > 1 && args.len() < 3 {
        ip = env::args().nth(1).unwrap();
    } else if args.len() > 2 {
        ip = env::args().nth(1).unwrap();
        name = env::args().nth(2).unwrap();
    } else {
        println!("Not enough arguments.\nUsage: anonchat-cli <ip> [name]\n\tip - IP of the server\n\tname - Name of your client. 'Anon' by default.");
        process::exit(1);
    };

    match ip.as_str() {
        "help" => help(),
        "--help" => help(),
        "-h" => help(),
        "-help" => help(),
        &_ => {},
    }

    return (ip, name);
}

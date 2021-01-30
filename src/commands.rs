use std::{error::Error, net::UdpSocket};

pub fn fetch(arguments: &[&str]) -> Result<String, Box<dyn Error>> {
    let url = arguments[0];
    println!("request url {}", url);
    let res = reqwest::blocking::get(url)?;
    println!("Status: {}", &res.status());
    println!("Headers:\n{:#?}", res.headers());

    let body = res.text()?;
    println!("Body:\n{}", body);
    Ok(body)
}

pub fn send(arguments: &[&str]) -> Result<String, Box<dyn Error>> {
    let socket = UdpSocket::bind("0.0.0.0:3400").expect("couldn't bind to address");
    let connection = socket.connect("localhost:34254");
    if connection.is_ok() {
        socket.send(arguments[0].as_bytes()).unwrap();
        Ok(String::from("ok"))
    } else {
        Ok(String::from("Unable to connect"))
    }
}

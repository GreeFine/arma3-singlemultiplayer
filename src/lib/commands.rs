use crate::{str_to_cstr, ACALLBACKPTR};
use std::{error::Error, ffi::CString};
use std::{net::UdpSocket, thread};

pub struct Connection {
    socket: Option<UdpSocket>,
    is_connected: bool,
}

impl Connection {
    pub const fn new() -> Self {
        Self {
            socket: None,
            is_connected: false,
        }
    }
    pub fn send(&self, arguments: &[&str]) -> Result<String, Box<dyn Error>> {
        if self.is_connected {
            self.socket
                .as_ref()
                .unwrap()
                .send(arguments[0].as_bytes())
                .unwrap();
            Ok(String::from("ok"))
        } else {
            Ok(String::from("I am not connected to a server"))
        }
    }

    pub fn connnect(&'static mut self, _arguments: &[&str]) -> Result<String, Box<dyn Error>> {
        if self.is_connected {
            return Ok(String::from("Already connected"));
        }

        let socket = UdpSocket::bind("0.0.0.0:0").expect("couldn't bind to address");
        let connection = socket.connect("localhost:34254");
        let mut separator = [0; 2];
        '|'.encode_utf8(&mut separator);

        if connection.is_ok() {
            socket.send(b"Hello").unwrap();
            self.socket = Some(socket);
            self.is_connected = true;
            thread::spawn(move || {
                let socket = self.socket.as_ref().unwrap();
                let cb_name: *const i8 = str_to_cstr!("ASMPrcv");
                let mut buf = [0; 1024];
                loop {
                    let (amt, _src) = socket.recv_from(&mut buf).unwrap();
                    let mut split_iter = buf[..amt].split(|val| val == &separator[0]);
                    // DEBUG ONLY, should exist in release
                    let (mut cb_function, mut cb_payload) = (
                        str_to_cstr!("dbg_error") as *const i8,
                        str_to_cstr!(&buf[..amt]) as *const i8,
                    );
                    if let Some(function) = split_iter.next() {
                        if let Some(payload) = split_iter.next() {
                            cb_function = str_to_cstr!(function);
                            cb_payload = str_to_cstr!(payload);
                        }
                    };
                    unsafe {
                        if let Some(ptr) = ACALLBACKPTR {
                            ptr(cb_name, cb_function, cb_payload);
                        }
                    }
                }
            });
            Ok(String::from("ok"))
        } else {
            Ok(String::from("Unable to connect"))
        }
    }
}

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

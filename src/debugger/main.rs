#![allow(non_snake_case)]
use libc::{c_char, c_int};
use rand::prelude::*;
use std::ffi::{CStr, CString};
use std::{env, thread, time::Duration};
use std::{io, str};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        println!("Missing parametters");
    } else {
        let mut stresstest = false;
        let function_args = args[2..].to_vec();
        let function;
        if args[1] == "stresstest" {
            stresstest = true;
            function = String::from("connect");
        } else {
            function = String::from(&args[1]);
        }

        println!(
            "Testing lib with function: {} args: {:#?}",
            &function, &function_args
        );
        if stresstest {
            stresstesting();
        } else {
            call_lib(&function, &function_args);
            if &function == "connect" {
                controle_unit();
            }
        }
    }
}

fn stresstesting() {
    let sendfnc = "send";
    let mut rng = rand::thread_rng();
    let mut pos: [f32; 3] = [0.0, 0.0, 0.0];
    call_lib(&String::from("connect"), &[]);
    loop {
        let displacement: f32 = rng.gen();
        let direction = rng.gen_range(0..2);
        if rng.gen() {
            pos[direction] += displacement;
        } else {
            pos[direction] -= displacement;
        }
        unsafe {
            rve_get(sendfnc, &[format!("[{},{},{}]", pos[0], pos[1], pos[2])]);
        };
        thread::sleep(Duration::from_millis(30));
    }
}

fn call_lib(function: &str, function_args: &[String]) {
    unsafe {
        rve_get_version();
        rve_send_ptr();
        rve_get(function, function_args);
    }
}

fn controle_unit() {
    let _stdout = io::stdout().into_raw_mode().unwrap();
    let mut stdin = termion::async_stdin().keys();
    let mut pos: [i8; 3] = [0, 0, 0];
    loop {
        let sendfnc = "send";
        let input = stdin.next();
        if let Some(Ok(key)) = input {
            match key {
                termion::event::Key::Char('q') => break,
                termion::event::Key::Char('w') => unsafe {
                    pos[0] += 1;
                    rve_get(sendfnc, &[format!("[{},{},{}]", pos[0], pos[1], pos[2])]);
                },
                termion::event::Key::Char('s') => unsafe {
                    pos[0] -= 1;
                    rve_get(sendfnc, &[format!("[{},{},{}]", pos[0], pos[1], pos[2])]);
                },
                termion::event::Key::Char('a') => unsafe {
                    pos[1] += 1;
                    rve_get(sendfnc, &[format!("[{},{},{}]", pos[0], pos[1], pos[2])]);
                },
                termion::event::Key::Char('d') => unsafe {
                    pos[1] -= 1;
                    rve_get(sendfnc, &[format!("[{},{},{}]", pos[0], pos[1], pos[2])]);
                },
                _ => {}
            }
            // write!(stdout, "{}", termion::cursor::Goto(2, 0),).unwrap();
        }
        thread::sleep(Duration::from_millis(50));
    }
}

unsafe fn rve_get_version() {
    let buffer = CString::new("____________________").unwrap().into_raw();

    let utf8_arr: &[u8] = CStr::from_ptr(buffer).to_bytes();
    println!("buffer: [{}]", str::from_utf8(utf8_arr).unwrap());
    ASMP::RVExtensionVersion(buffer, utf8_arr.len() + 1);
    println!("buffer result: [{}]", str::from_utf8(utf8_arr).unwrap());
}

extern "stdcall" fn a3ptr(
    name: *const c_char,
    function: *const c_char,
    data: *const c_char,
) -> c_int {
    unsafe {
        println!(
            "CallBacked from lib: {} | {} | {}",
            CStr::from_ptr(name).to_str().unwrap(),
            CStr::from_ptr(function).to_str().unwrap(),
            CStr::from_ptr(data).to_str().unwrap(),
        );
    }
    0
}

unsafe fn rve_send_ptr() {
    ASMP::RVExtensionRegisterCallback(a3ptr);
}

unsafe fn rve_get(function: &str, function_args: &[String]) {
    let function_c = CString::new(function).unwrap().into_raw();
    let buffer = CString::new("________________________________________________________________")
        .unwrap()
        .into_raw();

    let utf8_arr: &[u8] = CStr::from_ptr(buffer).to_bytes();
    println!("buffer: [{}]", str::from_utf8(utf8_arr).unwrap());

    let mut args = Vec::new();
    for arg in function_args {
        let new_arg: *const i8 = CString::new(arg.clone()).unwrap().into_raw();
        args.push(new_arg);
    }

    let argv = args.as_ptr();
    ASMP::RVExtensionArgs(
        buffer,
        (utf8_arr.len() + 1) as c_int,
        function_c,
        argv,
        args.len() as c_int,
    );
    println!("buffer result: [{}]", str::from_utf8(utf8_arr).unwrap());
}

use std::{
    net::{SocketAddr, UdpSocket},
    str::from_utf8,
};

pub fn start() -> std::io::Result<()> {
    {
        let socket = UdpSocket::bind("0.0.0.0:34254")?;
        let mut users_socket: Vec<SocketAddr> = Vec::new();

        loop {
            // Receives a single datagram message on the socket. If `buf` is too small to hold
            // the message, it will be cut off.
            let mut buf = [0; 1024];
            let (amt, src) = socket.recv_from(&mut buf)?;

            check_if_new_user(src, &mut users_socket);

            // Redeclare `buf` as slice of the received data and send reverse data back to origin.
            let buf = &mut buf[..amt];
            println!(
                "rcv from: {}, msg: [{}]",
                src.to_string(),
                from_utf8(buf).unwrap()
            );
            tranfer_packet(src, buf, &users_socket, &socket)?;
        }
    } // the socket is closed here
}

fn tranfer_packet(
    sender: SocketAddr,
    buf: &[u8],
    users_socket: &[SocketAddr],
    socket: &UdpSocket,
) -> std::io::Result<()> {
    for src in users_socket {
        if &sender != src {
            socket.send_to(buf, src)?;
        }
    }
    Ok(())
}

fn check_if_new_user(src: SocketAddr, users_socket: &mut Vec<SocketAddr>) {
    if !users_socket.contains(&src) {
        users_socket.push(src);
        println!("New user: {}", src.to_string())
    }
}

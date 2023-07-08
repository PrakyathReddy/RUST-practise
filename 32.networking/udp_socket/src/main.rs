use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:34254")?;

    // recieves a single datagram message on the socket. If 'buf' is too
    // small to hold the message, it will cut off.
    let mut buf = [0; 10];
    let (amt, src) = socket.recv_from(&mut buf)?;

    // re-declare buf as slice of the recieved data and send reverse data
    // back to origin
    let buf = &mut buf[..amt];
    buf.reverse();
    socket.send_to(buf, src)?;

    Ok(())
}

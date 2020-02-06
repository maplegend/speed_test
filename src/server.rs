use std::net::UdpSocket;
use clap::ArgMatches;

#[path = "utils/utils.rs"]
mod utils;

pub fn run(args: &ArgMatches) -> std::io::Result<()> {
    start(args.value_of("ip").unwrap())
}

pub fn start(ip: &str) -> std::io::Result<()>{
    {
        let socket = UdpSocket::bind(ip)?;

        loop {
            // Receives a single datagram message on the socket. If `buf` is too small to hold
            // the message, it will be cut off.
            let mut buf = [0; 2000];
            let (amt, src) = socket.recv_from(&mut buf)?;

            // Redeclare `buf` as slice of the received data and send reverse data back to origin.

            let buf = & mut buf[..amt];
            //buf.reverse();
            //utils::print_timestamp(u64::from_be_bytes(utils::to_static8(buf)));

            let mut timestamp = utils::get_timestamp().to_be_bytes();
            println!("{:?} {}", timestamp, timestamp.len());
            println!("{:?}", buf);

            socket.send_to(&[buf, &mut timestamp].concat(), &src)?;
        }
    } // the socket is closed here
}





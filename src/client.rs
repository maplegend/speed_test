use std::net::UdpSocket;
use clap::ArgMatches;

#[path = "utils/utils.rs"]
mod utils;

pub fn run(args: &ArgMatches) -> std::io::Result<()> {
    let mut payload = vec![];
    if let Some(pl) = args.value_of("payload") {
        payload = pl.as_bytes().to_vec();
    } else if let Some(c) = args.value_of("size") {
        payload = vec![123u8; c.parse::<usize>().expect("Size should be int")]
    }
    connect(args.value_of("ip").unwrap(),
            payload,
            args.value_of("tries").unwrap().parse::<u32>().expect("Tries should be int"))
}

fn connect(addr: &str, payload: Vec<u8>, tries: u32) -> std::io::Result<()> {
    {
        let socket = UdpSocket::bind("127.0.0.1:34254")?;
        socket.connect(addr).expect("Cannot connect to the server");

        let mut res: Vec<(u64, u64)> =  vec![];
        for _ in 0..tries {
            res.push(measure_delays(&socket, &payload));
        }

        let mut rping_sum = 0;
        let mut sping_sum = 0;
        res.iter().for_each(|el| {
            rping_sum += el.0;
            sping_sum += el.1;
        });
        let tping_sum = rping_sum + sping_sum;
        let len = (res.len() as u64) * 1_000_000;
        println!("Send avg ping: {}; Receive avg ping: {}; Total avg ping: {}",
                 sping_sum/len, rping_sum/len, tping_sum/len);

    } // the socket is closed here
    Ok(())
}

fn measure_delays(socket: &UdpSocket, payload: &[u8]) -> (u64, u64) {
    let in_ns = utils::get_timestamp();
    //println!("send stamp {}", in_ns);
    socket.send(payload).expect("Cannot send payload");
    let mut buf = [0u8; 2000];
    socket.recv(&mut buf).expect("Cannot receive response");
    let server_recv_time = u64::from_be_bytes(utils::to_static8(&buf[payload.len()..payload.len()+8]));
    let send_ping = server_recv_time - in_ns;
    let recv_time = utils::get_timestamp();
    let recv_ping = recv_time - server_recv_time;
    println!("rping {} sping {}", recv_ping, send_ping);

    (recv_ping, send_ping)
}


use heapless::consts::U512;
use librobot::transmission::navigation::{NavigationCommand, NavigationFrame};
use librobot::transmission::Jsonizable;
use std::net::UdpSocket; // type level integer used to specify capacity

fn get_input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer
}

fn normalize(cmd: &NavigationCommand, data: u16) -> u16 {
    match cmd {
        NavigationCommand::GoForward => data * 100,
        NavigationCommand::GoBackward => data * 100,
        NavigationCommand::TurnRelative => (data as f32 * (314.15 / 180.)) as u16,
        NavigationCommand::TurnAbsolute => (data as f32 * (314.15 / 180.)) as u16,
        _ => unreachable!(),
    }
}

fn main() {
    let mut cpt = 1;
    let socket = UdpSocket::bind("0.0.0.0:5001").expect("couldn't bind to address");
    socket.connect("192.168.2.1:51").unwrap();
    loop {
        println!("What do you want to do ?");
        println!("  1) Go Forward");
        println!("  2) Go Backward");
        println!("  3) Turn Relative");
        println!("  4) Turn Absolute");
        let mut frame = NavigationFrame::default();
        frame.counter = cpt;
        frame.asserv_lin = true;
        frame.asserv_ang = true;
        let cmd = match get_input().trim().parse::<i64>().unwrap() {
            1 => NavigationCommand::GoForward,
            2 => NavigationCommand::GoBackward,
            3 => NavigationCommand::TurnRelative,
            4 => NavigationCommand::TurnAbsolute,
            _ => continue,
        };
        frame.command = cmd;
        println!("Enter desired command");

        let data = get_input().trim().parse::<u16>().unwrap();
        frame.args_cmd1 = normalize(&cmd, data);
        match cmd {
            NavigationCommand::TurnAbsolute | NavigationCommand::TurnRelative => {
                frame.args_cmd2 = 0;
            }
            _ => {}
        }
        println!("Data : {}", frame.args_cmd1);
        println!("Sending...");
        socket
            .send(
                frame
                    .to_string::<U512>()
                    .expect("Failed JSON ser")
                    .as_bytes(),
            )
            .unwrap();
        println!("Done !");
        cpt += 1;
    }
}

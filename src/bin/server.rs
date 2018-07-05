extern crate argparse;
extern crate dsc_band_ack;

use argparse::{ArgumentParser, Store};
use dsc_band_ack::server;

fn main() {
    // Default address and port values
    let mut ip = "127.0.0.1".to_string();
    let mut port = "4040".to_string();
    let mut device = "".to_string();

    {
        let mut ap = ArgumentParser::new();
        ap.set_description("DSC Band Ack Server.");
        ap.refer(&mut ip).add_option(&["--ip"], Store, "IP address to listen on");
        ap.refer(&mut port).add_option(&["-p", "--port"], Store, "Port to listen on");
        ap.refer(&mut device).add_option(&["-d", "--device"], Store, "Serial device to use");
        ap.parse_args_or_exit();
    }

    let address = format!("{}:{}", ip, port);
    server::run(&address, device);
}

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate argparse;
use argparse::{ArgumentParser, StoreOption, Store};

use std::env;



mod band_ack;
mod server;



fn main() {
    // let mut address = "127.0.0.1:9123".to_string();
    let mut ip = "127.0.0.1".to_string();
    let mut port = "4040".to_string();

    let mut call_address: Option<u8> = None;

    {
        let mut ap = ArgumentParser::new();
        ap.set_description("DSC Band Ack Server.");

        ap.refer(&mut call_address).add_option(
            &["-c", "--call"], StoreOption,
            "If set, we will not start a TCP server, and just get the ticks from the device with the provides address"
        );

        ap.refer(&mut ip).add_option(&["--ip"], Store, "IP address to listen on");
        ap.refer(&mut port).add_option(&["-p", "--port"], Store, "Port to listen on");

        ap.parse_args_or_exit();
    }

    match call_address {
        None => {
            let address = format!("{}:{}", ip, port);
            server::run(&address);
        },
        Some(address) => match band_ack::get_ticks(address) {
            Ok(ticks) => println!("{}", ticks),
            Err(err) => println!("{}", err),
        },
    }


}

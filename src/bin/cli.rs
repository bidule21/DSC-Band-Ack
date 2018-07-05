extern crate argparse;
extern crate dsc_band_ack;

use argparse::{ArgumentParser, Store};
use dsc_band_ack::band_ack;

fn main() {
    // Band Ack device address, will be called
    let mut address: u8 = 0_u8;
    let mut device = "".to_string();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("DSC Band Ack CLI.");
        ap.refer(&mut address).add_option(
            &["-a", "--address"], Store,
            "Band Ack device address to get ticks from"
        );
        ap.refer(&mut device).add_option(&["-d", "--device"], Store, "Serial device to use");
        ap.parse_args_or_exit();
    };

    match band_ack::get_ticks(address, device) {
        Ok(ticks) => println!("{}", ticks),
        Err(err) => println!("{}", err),
    };
}

extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/band_ack/band_ack.c")
        .compile("band_ack_c");
}

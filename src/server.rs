use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::Read;
use std::time::Duration;
use std::str;
use serde_json;
use serde_json::Error as JSONError;

use band_ack;



#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
enum Action {
    Ping,
    GetTicks { address: u8 },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
enum Answer {
    Pong,
    Ticks { address: u8, ticks: u16 },
    Error { error: String },
}



/// Start TCP server which will listen to new connection and spawn a thread for each client.
/// This method will block.
///
/// server_address      IP and port to listen on
pub fn run(server_address: &str) {
    let listener = TcpListener::bind(server_address).unwrap();
    println!("DSC Band Ack TCP Server ready on {}", server_address);
    for stream in listener.incoming() {
        thread::spawn(|| {
            if let Ok(stream) = stream {
                client_loop(stream);
            }
        });
    }
}



/// Perform client loop for TCP client. We wait until we recive an action, perform it, and send
/// results back to the sender
fn client_loop(mut stream: TcpStream) {
    let _ = stream.set_nonblocking(true).expect("set_nonblocking call failed");
    let _ = stream.set_read_timeout(Some(Duration::from_millis(200))).expect("set_read_timeout call failed");
    loop {
        // read max 512 byte
        let mut buf = [0; 512];
        match stream.read(&mut buf) {
            // if we have some data recived, parse it as json
            Ok(m) if m > 0 => {
                let json_string = str::from_utf8(&buf[0..m]).unwrap();
                let action: Result<Action, JSONError> = serde_json::from_str(&json_string);
                let answer = match action {
                    // Send pong to sender
                    Ok(Action::Ping) => Answer::Pong,

                    // perform get_ticks and send result to sender
                    Ok(Action::GetTicks{address}) => {
                        match band_ack::get_ticks(address) {
                            Ok(ticks) => Answer::Ticks { address, ticks },
                            Err(err) => Answer::Error { error: format!("{}", err) },
                        }
                    },

                    // Send json error back to sender
                    Err(err) => Answer::Error { error: format!("{}", err) },
                };
                let answer_json = serde_json::to_string(&answer).unwrap();
                let _ = stream.write(answer_json.as_bytes());
                return
            },
            Err(_) => return,
            _ => {},
        };

    }
}

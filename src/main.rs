//! Forwards received packets to the Arrow Network
use std::net::TcpStream;
use std::io::prelude::*;

// use svc_telemetry_client_rest::types::MavlinkMessage;
// use adsb_deku::Frame;
// use adsb_deku::deku::DekuContainerRead;

const MSG_BUF_SIZE: usize = 100;
const DUMP_1090_PORT_VAR: &str = "DUMP_1090_PORT";
const DUMP_1090_HOST_VAR: &str = "DUMP_1090_HOST";
const ADSB_URL_VAR: &str = "ADSB_URL";

#[tokio::main]
async fn main() {
    //
    // Get Env Variables
    //
    let port = std::env::var(DUMP_1090_PORT_VAR).expect(
        &format!("ERROR: environment variable missing: {}",
            DUMP_1090_PORT_VAR)
    );

    let host = std::env::var(DUMP_1090_HOST_VAR).expect(
        &format!("ERROR: environment variable missing: {}",
            DUMP_1090_HOST_VAR)
    );

    let url = std::env::var(ADSB_URL_VAR)
        .expect(&format!("ERROR: environment variable missing: {}",
            ADSB_URL_VAR));

    //
    // Bind to dump1090
    //
    println!("Connecting to dump1090 at {}:{}", host, port);
    let mut stream = TcpStream::connect(format!("{host}:{port}"))
        .expect("Couldn't bind to the dump1090 port.");
    stream.set_read_timeout(None).expect("set_read_timeout call failed");

    //
    // Create Ring Buffer for messages to svc-telemetry
    //
    let rb = ringbuf::HeapRb::<[u8; 14]>::new(MSG_BUF_SIZE);
    let (mut prod, mut cons) = rb.split();

    //
    // Create async thread for sending messages to svc-telemetry
    //
    tokio::spawn(async move {
        let client = reqwest::Client::new();

        loop {
            if let Some(data) = cons.pop() {
                let req = client
                    .post(url.clone())
                    .header("Content-Type", "application/octet-stream")
                    .body(data.to_vec())
                    .send()
                    .await;

                println!("{:?}", req);
            }
        }
    });

    //
    // Read output from dump1090 and forward complete frames
    //
    let mut buf: [u8; 1] = [0];
    let mut message: bool = false;
    let mut msg: Vec<u8> = vec![];
    loop {
        stream.read(&mut buf).unwrap();
        if buf[0] == 42 {
            message = true;
            msg.clear();
        } else if buf[0] == 59 && message {
            let text = std::str::from_utf8(&msg).unwrap();
            let data = hex::decode(text).unwrap();

            let Ok(frame) = <[u8; 14]>::try_from(data) else {
                return;
            };

            if ((frame[0] >> 3) & 0x1F) != 17 {
                return;
            }

            prod.push(frame).unwrap();
            message = false;
            msg.clear();
        } else {
            msg.push(buf[0]);
        }
    }
}

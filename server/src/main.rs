use std::env;
use std::error::Error;
use std::mem;
use std::net::UdpSocket;
use std::process::Command;
use structview::{u32_le, View};

const MESSAGE_MAGIC: u32 = 0x768DD122;
const MESSAGE_COMMAND_LEFT: u32 = 1;
const MESSAGE_COMMAND_RIGHT: u32 = 2;
const MESSAGE_COMMAND_UP: u32 = 3;
const MESSAGE_COMMAND_DOWN: u32 = 4;

#[derive(Clone, Copy, View)]
#[repr(C)]
struct Message {
    magic: u32_le,
    command: u32_le,
}

fn main() -> Result<(), Box<Error>> {
    let socket = UdpSocket::bind("0.0.0.0:15170")?;

    env::set_var("DISPLAY", ":0.0");

    loop {
        let mut buf = [0; mem::size_of::<Message>()];

        let amt = socket.recv(&mut buf)?;

        if amt == mem::size_of::<Message>() {
            if let Ok(message) = Message::view(&buf) {

                if message.magic.to_int() == MESSAGE_MAGIC {
                    match message.command.to_int() {
                        MESSAGE_COMMAND_LEFT => {
                            Command::new("xdotool")
                                .arg("key")
                                .arg("Left")
                                .output()
                                .expect("failed to execute process");
                        }
                        MESSAGE_COMMAND_RIGHT => {
                            Command::new("xdotool")
                                .arg("key")
                                .arg("Right")
                                .output()
                                .expect("failed to execute process");
                        }
                        MESSAGE_COMMAND_UP => {
                            Command::new("xdotool")
                                .arg("key")
                                .arg("Up")
                                .output()
                                .expect("failed to execute process");
                        }
                        MESSAGE_COMMAND_DOWN => {
                            Command::new("xdotool")
                                .arg("key")
                                .arg("Down")
                                .output()
                                .expect("failed to execute process");
                        }
                        _ => ()
                    }
                }
            }
        }
    }
}

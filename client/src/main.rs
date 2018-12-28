use gilrs::ev::Axis::{LeftStickX, LeftStickY};
use gilrs::ev::EventType::{AxisChanged, ButtonPressed};
use gilrs::{Button, Event, Gilrs};
use std::error::Error;
use std::mem;
use std::net::UdpSocket;
use std::slice::from_raw_parts;
use std::thread;
use std::time::Duration;
use winapi::um::winuser;

const MESSAGE_MAGIC: u32 = 0x768DD122;
const MESSAGE_COMMAND_LEFT: u32 = 1;
const MESSAGE_COMMAND_RIGHT: u32 = 2;
const MESSAGE_COMMAND_UP: u32 = 3;
const MESSAGE_COMMAND_DOWN: u32 = 4;
const MESSAGE_COMMAND_ENTER: u32 = 5;
const MESSAGE_COMMAND_ESCAPE: u32 = 6;
const MESSAGE_COMMAND_MENU: u32 = 7;

#[derive(Clone, Copy)]
#[repr(C)]
struct Message {
    magic: u32,
    command: u32,
}

fn send_command(socket: &UdpSocket, command: u32) -> Result<(), Box<Error>> {
    let message = Message {
        magic: MESSAGE_MAGIC,
        command: command,
    };

    socket.send_to(
        unsafe {
            from_raw_parts::<u8>(&message as *const _ as *const _, mem::size_of::<Message>())
        },
        "192.168.0.133:15170",
    )?;

    Ok(())
}

fn main() -> Result<(), Box<Error>> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;

    let mut gilrs = Gilrs::new()?;

    let mut left_axis_held = false;
    let mut right_axis_held = false;
    let mut up_axis_held = false;
    let mut down_axis_held = false;

    let mut left_held = false;
    let mut right_held = false;
    let mut up_held = false;
    let mut down_held = false;

    loop {
        while let Some(event) = gilrs.next_event() {
            match event {
                Event {
                    id: 0,
                    event: AxisChanged(LeftStickX, value, _),
                    time: _,
                } => {
                    if value <= -1.0 && left_axis_held == false {
                        println!("Left Pressed");
                        send_command(&socket, MESSAGE_COMMAND_LEFT)?;

                        left_axis_held = true;
                    } else if value > -1.0 && value != 0.0 {
                        left_axis_held = false;
                    }

                    if value >= 1.0 && right_axis_held == false {
                        println!("Right Pressed");
                        send_command(&socket, MESSAGE_COMMAND_RIGHT)?;

                        right_axis_held = true;
                    } else if value < 1.0 && value != 0.0 {
                        right_axis_held = false;
                    }
                }
                Event {
                    id: 0,
                    event: AxisChanged(LeftStickY, value, _),
                    time: _,
                } => {
                    if value >= 1.0 && up_axis_held == false {
                        println!("Up Pressed");
                        send_command(&socket, MESSAGE_COMMAND_UP)?;

                        up_axis_held = true;
                    } else if value < 1.0 && value != 0.0 {
                        up_axis_held = false;
                    }

                    if value <= -1.0 && down_axis_held == false {
                        println!("Down Pressed");
                        send_command(&socket, MESSAGE_COMMAND_DOWN)?;

                        down_axis_held = true;
                    } else if value > -1.0 && value != 0.0 {
                        down_axis_held = false;
                    }
                }
                Event {
                    id: 0,
                    event: ButtonPressed(Button::South, _),
                    time: _,
                } => {
                    println!("Enter Pressed");
                    send_command(&socket, MESSAGE_COMMAND_ENTER)?;
                }
                Event {
                    id: 0,
                    event: ButtonPressed(Button::East, _),
                    time: _,
                } => {
                    println!("Escape Pressed");
                    send_command(&socket, MESSAGE_COMMAND_ESCAPE)?;
                }
                Event {
                    id: 0,
                    event: ButtonPressed(Button::North, _),
                    time: _,
                } => {
                    println!("Menu Pressed");
                    send_command(&socket, MESSAGE_COMMAND_MENU)?;
                }
                _ => (),
            }

            println!("New event: {:?}", event);
        }

        if unsafe { winuser::GetAsyncKeyState(winuser::VK_LEFT) as u16 & 0x8000 > 0 } {
            if left_held == false {
                println!("Left Pressed");
                send_command(&socket, MESSAGE_COMMAND_LEFT)?;
            }

            left_held = true;
        } else {
            left_held = false;
        }

        if unsafe { winuser::GetAsyncKeyState(winuser::VK_RIGHT) as u16 & 0x8000 > 0 } {
            if right_held == false {
                println!("Right Pressed");
                send_command(&socket, MESSAGE_COMMAND_RIGHT)?;
            }

            right_held = true;
        } else {
            right_held = false;
        }

        if unsafe { winuser::GetAsyncKeyState(winuser::VK_UP) as u16 & 0x8000 > 0 } {
            if up_held == false {
                println!("Up Pressed");
                send_command(&socket, MESSAGE_COMMAND_UP)?;
            }

            up_held = true;
        } else {
            up_held = false;
        }

        if unsafe { winuser::GetAsyncKeyState(winuser::VK_DOWN) as u16 & 0x8000 > 0 } {
            if down_held == false {
                println!("Down Pressed");
                send_command(&socket, MESSAGE_COMMAND_DOWN)?;
            }

            down_held = true;
        } else {
            down_held = false;
        }

        thread::sleep(Duration::from_millis(1));
    }
}

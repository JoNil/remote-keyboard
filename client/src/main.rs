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

#[derive(Clone, Copy)]
#[repr(C)]
struct Message {
    magic: u32,
    command: u32,
}

fn main() -> Result<(), Box<Error>> {
    
    let socket = UdpSocket::bind("0.0.0.0:0")?;

    let mut left_held = false;
    let mut right_held = false;
    let mut up_held = false;
    let mut down_held = false;

    loop {

         if unsafe { winuser::GetAsyncKeyState(winuser::VK_LEFT) as u16 & 0x8000 > 0 } {

            if left_held == false {
                println!("Left Pressed");

                let message = Message {
                    magic: MESSAGE_MAGIC,
                    command: MESSAGE_COMMAND_LEFT,
                };

                socket.send_to(
                    unsafe { from_raw_parts::<u8>(&message as *const _ as *const _, mem::size_of::<Message>()) },
                    "192.168.0.133:15170")?;
            }
            
            left_held = true;
            
        } else {
            left_held = false;
        }

        if unsafe { winuser::GetAsyncKeyState(winuser::VK_RIGHT) as u16 & 0x8000 > 0 } {

            if right_held == false {
                println!("Right Pressed");

                let message = Message {
                    magic: MESSAGE_MAGIC,
                    command: MESSAGE_COMMAND_RIGHT,
                };

                socket.send_to(
                    unsafe { from_raw_parts::<u8>(&message as *const _ as *const _, mem::size_of::<Message>()) },
                    "192.168.0.133:15170")?;
            }
            
            right_held = true;
            
        } else {
            right_held = false;
        }

        if unsafe { winuser::GetAsyncKeyState(winuser::VK_UP) as u16 & 0x8000 > 0 } {

            if up_held == false {
                println!("Up Pressed");

                let message = Message {
                    magic: MESSAGE_MAGIC,
                    command: MESSAGE_COMMAND_UP,
                };

                socket.send_to(
                    unsafe { from_raw_parts::<u8>(&message as *const _ as *const _, mem::size_of::<Message>()) },
                    "192.168.0.133:15170")?;
            }
            
            up_held = true;
            
        } else {
            up_held = false;
        }

        if unsafe { winuser::GetAsyncKeyState(winuser::VK_DOWN) as u16 & 0x8000 > 0 } {

            if down_held == false {
                println!("Down Pressed");

                let message = Message {
                    magic: MESSAGE_MAGIC,
                    command: MESSAGE_COMMAND_DOWN,
                };

                socket.send_to(
                    unsafe { from_raw_parts::<u8>(&message as *const _ as *const _, mem::size_of::<Message>()) },
                    "192.168.0.133:15170")?;
            }
            
            down_held = true;
            
        } else {
            down_held = false;
        }

        thread::sleep(Duration::from_millis(10));
    }
}

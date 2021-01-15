extern crate syscall;

use syscall::data::Packet;
use syscall::flag::CloneFlags;
use syscall::scheme::Scheme;
use std::fs::File;
use std::io::{Read, Write};
use scheme::NullScheme;

mod scheme;

fn main() {
    if unsafe { syscall::clone(CloneFlags::empty()).unwrap() } == 0 {
        let mut socket = File::create(":null").expect("nulld: failed to create null scheme");
        let scheme = NullScheme;

        syscall::setrens(0, 0).expect("nulld: failed to enter null namespace");

        loop {
            let mut packet = Packet::default();
            if socket.read(&mut packet).expect("nulld: failed to read events from null scheme") == 0 {
                break;
            }

            scheme.handle(&mut packet);

            socket.write(&packet).expect("nulld: failed to write responses to null scheme");
        }
    }
}

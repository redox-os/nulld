use std::fs::File;
use std::io::{Read, Write};

use syscall::data::Packet;
use syscall::scheme::Scheme;

use scheme::NullScheme;

mod scheme;

fn main() {
    redox_daemon::Daemon::new(move |daemon| {
        let mut socket = File::create(":null").expect("nulld: failed to create null scheme");
        let scheme = NullScheme;

        syscall::setrens(0, 0).expect("nulld: failed to enter null namespace");

        daemon.ready().expect("nulld: failed to notify parent");

        loop {
            let mut packet = Packet::default();
            if socket.read(&mut packet).expect("nulld: failed to read events from null scheme") == 0 {
                std::process::exit(0);
            }

            scheme.handle(&mut packet);

            socket.write(&packet).expect("nulld: failed to write responses to null scheme");
        }
    }).expect("nulld: failed to daemonize");
}

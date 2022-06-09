use std::mem::size_of_val;
use std::{thread::sleep, time::Duration};

use libbpf_rs::MapFlags;
use libc;
use plain;

use prog::*;

#[path = "bpf/.output/program.skel.rs"]
mod prog;

fn main() -> ! {
    let builder = ProgramSkelBuilder::default();

    let open_skel = builder.open().expect("Failed to open BPF program");
    let mut skel = open_skel.load().expect("Failed to load BPF program");

    // skel.attach()?

    unsafe {
        let protocol = (libc::ETH_P_ALL as libc::c_short).to_be() as libc::c_int;

        let sock = libc::socket(libc::AF_PACKET, libc::SOCK_RAW, protocol);
        assert_ne!(sock, -1, "Failed to create socket");

        let prog_fd = skel.progs().bpf_program().fd();
        let value = &prog_fd as *const i32;
        let option_len = size_of_val(&prog_fd) as libc::socklen_t;

        let sockopt = libc::setsockopt(
            sock,
            libc::SOL_SOCKET,
            libc::SO_ATTACH_BPF,
            value as *const libc::c_void,
            option_len,
        );
        assert_eq!(sockopt, 0, "Failed to set socket option");
    };

    let mut maps = skel.maps_mut();
    let map = maps.map();

    let key = unsafe { plain::as_bytes(&(libc::IPPROTO_ICMP as u32)) };
    let mut value: u64 = 0;

    loop {
        match map.lookup(key, MapFlags::ANY) {
            Ok(Some(buf)) => {
                plain::copy_from_bytes(&mut value, &buf).expect("Invalid buffer");
                println!("Map Value: {:?}", value);
            }
            _ => {}
        }
        sleep(Duration::from_secs(5));
    }
}

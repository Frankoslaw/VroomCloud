#![no_std]
#![no_main]
#![allow(nonstandard_style, dead_code)]

use aya_bpf::{
    bindings::xdp_action,
    macros::{map, xdp},
    maps::HashMap,
    programs::XdpContext
};
use aya_log_ebpf::info;

use core::mem;
use network_types::{
    eth::{EthHdr, EtherType},
    ip::{IpProto, Ipv4Hdr},
    udp::UdpHdr,
};

use demo_kong_common::BackendPorts;

#[map(name = "BACKEND_PORTS")]
static mut BACKEND_PORTS: HashMap<u16, BackendPorts> =
    HashMap::<u16, BackendPorts>::with_max_entries(10, 0);



#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}

#[inline(always)]
fn ptr_at<T>(ctx: &XdpContext, offset: usize) -> Result<*const T, ()> {
    let start = ctx.data();
    let end = ctx.data_end();
    let len = mem::size_of::<T>();

    if start + offset + len > end {
        return Err(());
    }

    Ok((start + offset) as *const T)
}

#[inline(always)]
fn ptr_at_mut<T>(ctx: &XdpContext, offset: usize) -> Result<*mut T, ()> {
    let ptr = ptr_at::<T>(ctx, offset)?;
    Ok(ptr as *mut T)
}


#[xdp]
pub fn demo_kong(ctx: XdpContext) -> u32 {
    match try_demo_kong(ctx) {
        Ok(ret) => ret,
        Err(_) => xdp_action::XDP_ABORTED,
    }
}

fn try_demo_kong(ctx: XdpContext) -> Result<u32, ()> {
    let ethhdr: *const EthHdr = ptr_at(&ctx, 0)?; //
    match unsafe { (*ethhdr).ether_type } {
        EtherType::Ipv4 => {}
        _ => return Ok(xdp_action::XDP_PASS),
    }

    let ipv4hdr: *const Ipv4Hdr = ptr_at(&ctx, EthHdr::LEN)?;

    let udphdr: *mut UdpHdr = match unsafe { (*ipv4hdr).proto } {
        IpProto::Udp => {
            ptr_at_mut(&ctx, EthHdr::LEN + Ipv4Hdr::LEN)?
        }
        _ => return Ok(xdp_action::XDP_PASS),
    };

    let source_port = u16::from_be(unsafe { (*udphdr).source });
    let ports_to_ignore: [u16; 7] = [22, 111, 2049, 4000, 4001, 4002, 20048 ];
    if ports_to_ignore.contains(&source_port) {
        return Ok(xdp_action::XDP_PASS);
    }

    let destination_port = u16::from_be(unsafe { (*udphdr).dest });

    info!(&ctx, "received a UDP packet");

    let backend =s match unsafe { BACKEND_PORTS.get(&destination_port) } {
        Some(backends) => {
            info!(&ctx, "FOUND backends for port");
            backends
        }
        None => {
            info!(&ctx, "NO backends found for this port");
            return Ok(xdp_action::XDP_PASS);
        }
    };

    if backends.index > backends.ports.len() - 1 {
        return Ok(xdp_action::XDP_ABORTED);
    }

    let new_destination_port = backends.ports[backends.index];
    unsafe { (*udphdr).dest = u16::from_be(new_destination_port) };

    info!(
        &ctx,
        "redirected port {} to {}", destination_port, new_destination_port
    );

    let mut new_backends = BackendPorts {
        ports: backends.ports,
        index: backends.index + 1,
    };

    if new_backends.index > new_backends.ports.len() - 1
        || new_backends.ports[new_backends.index] == 0
    {
        new_backends.index = 0;
    }

    match unsafe { BACKEND_PORTS.insert(&destination_port, &new_backends, 0) } {
        Ok(_) => {
            info!(&ctx, "index updated for port {}", destination_port);
            Ok(xdp_action::XDP_PASS)
        }
        Err(err) => {
            info!(&ctx, "error inserting index update: {}", err);
            Ok(xdp_action::XDP_ABORTED)
        }
    }
}

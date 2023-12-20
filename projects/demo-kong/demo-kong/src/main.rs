use anyhow::Context;
use aya::maps::HashMap;
use aya::programs::{Xdp, XdpFlags};
use aya::{include_bytes_aligned, Bpf};
use aya_log::BpfLogger;
use clap::Parser;

use demo_kong_common::BackendPorts;
use log::{info, warn, debug};

use actix_web::{get, post, put, web, App, HttpServer, Responder, HttpResponse};
use std::sync::Mutex;

use serde::{Serialize, Deserialize};

struct AppState {
    bpf: Mutex<Bpf>
}

#[get("/backends")]
async fn get_backends(data: web::Data<AppState>) -> impl Responder {
    let bpf = data.bpf.lock().unwrap();

    let backends: HashMap<_, u16, BackendPorts> =
        HashMap::try_from(bpf.map("BACKEND_PORTS").unwrap()).unwrap();

    let mut source_ports = std::collections::HashMap::new();

    for key in backends.keys() {
        let key = key.unwrap();
        source_ports.insert(
            key,
            backends.get(&key, 0).unwrap()
        );
    }

    HttpResponse::Ok().body(format!("{:#?}", source_ports))
}

#[get("/backend/{source_port}")]
async fn get_backend(data: web::Data<AppState>, path: web::Path<u16>) -> impl Responder {
    let source_port = path.into_inner();
    let bpf = data.bpf.lock().unwrap();

    let backends: HashMap<_, u16, BackendPorts> =
        HashMap::try_from(bpf.map_mut("BACKEND_PORTS").unwrap()).unwrap();


}

#[derive(Deserialize)]
struct UpdateBackendPorts {
    ports: [u16, 16],
}

#[put("/backends/{source_port}/ports")]
async fn put_ports(data: web::Data<AppState>, update_backend_ports: web::Json<UpdateBackendPorts>) -> impl Responder {
    let bpf = data.bpf.lock().unwrap();

    let backends: HashMap<_, u16, BackendPorts> =
        HashMap::try_from(bpf.map_mut("BACKEND_PORTS").unwrap()).unwrap();

    HttpResponse::Ok().body
}

#[derive(Debug, Parser)]
struct Opt {
    // #[clap(short, long, default_value = "eth0")]
    #[clap(short, long, default_value = "lo")]
    iface: String,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let opt = Opt::parse();

    env_logger::init();

    // Bump the memlock rlimit. This is needed for older kernels that don't use the
    // new memcg based accounting, see https://lwn.net/Articles/837122/
    let rlim = libc::rlimit {
        rlim_cur: libc::RLIM_INFINITY,
        rlim_max: libc::RLIM_INFINITY,
    };
    let ret = unsafe { libc::setrlimit(libc::RLIMIT_MEMLOCK, &rlim) };
    if ret != 0 {
        debug!("remove limit on locked memory failed, ret is: {}", ret);
    }

    // This will include your eBPF object file as raw bytes at compile-time and load it at
    // runtime. This approach is recommended for most real-world use cases. If you would
    // like to specify the eBPF program at runtime rather than at compile-time, you can
    // reach for `Bpf::load_file` instead.
    #[cfg(debug_assertions)]
    let mut bpf = Bpf::load(include_bytes_aligned!(
        "../../target/bpfel-unknown-none/debug/demo-kong"
    ))?;
    #[cfg(not(debug_assertions))]
    let mut bpf = Bpf::load(include_bytes_aligned!(
        "../../target/bpfel-unknown-none/release/demo-kong"
    ))?;
    if let Err(e) = BpfLogger::init(&mut bpf) {
        // This can happen if you remove all log statements from your eBPF program.
        warn!("failed to initialize eBPF logger: {}", e);
    }
    let program: &mut Xdp = bpf.program_mut("demo_kong").unwrap().try_into()?;
    program.load()?;
    program.attach(&opt.iface, XdpFlags::default())
        .context("failed to attach the XDP program with default flags - try changing XdpFlags::default() to XdpFlags::SKB_MODE")?;

    let mut backends: HashMap<_, u16, BackendPorts> =
        HashMap::try_from(bpf.map_mut("BACKEND_PORTS").unwrap())?;

    let mut ports: [u16; 16] = [0; 16];

    ports[0] = 9876;
    ports[1] = 9877;
    ports[2] = 9878;

    let backend_ports = BackendPorts { ports, index: 0 };
    backends.insert(9875, backend_ports, 0)?;

    let app_state = web::Data::new(AppState {
        bpf: Mutex::new(bpf)
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(get_backends)
            .service(get_backend)
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await?;

    info!("Exiting...");

    Ok(())

}

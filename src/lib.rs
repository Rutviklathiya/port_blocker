use anyhow::{bail, Result};

#[path = "./bpf/.out/block.skel.rs"]
mod block_skel;
use block_skel::{BlockMapsMut, BlockSkelBuilder};

const FIREWALL_FILE: &str = "/sys/fs/bpf/my_firewall";

fn bump_memlock_rlimit() -> Result<()> {
    let rlimit = libc::rlimit {
        rlim_cur: 128 << 20,
        rlim_max: 128 << 20,
    };

    if unsafe { libc::setrlimit(libc::RLIMIT_MEMLOCK, &rlimit) } != 0 {
        bail!("Failed to increase rlimit");
    }

    Ok(())
}

// No code changes are expected here
pub fn unload_firewall() {
    // This is a VERY bad way to unload the firewall
    let _ = std::fs::remove_file(FIREWALL_FILE);

    // It would be nice if we could do the following
    // Question: Why are we unable to do this?
    // See: https://github.com/libbpf/libbpf-rs/blob/master/libbpf-rs/src/link.rs#L19
    //if let Ok(mut link) = libbpf_rs::Link::open(FIREWALL_FILE) {
    //    link.unpin().unwrap();
    //}

    // give the kernel a moment to unload the bpffs file
    std::thread::sleep(std::time::Duration::from_millis(250));
}

fn load_ports(ports: Vec<u16>, mut maps: BlockMapsMut) -> Result<()> {
    let allowed_ports = maps.allowed_ports();

    for (i, p) in ports.into_iter().enumerate() {
        let key = (i as u32).to_ne_bytes();
        let port = p.to_ne_bytes();
        allowed_ports.update(&key, &port, libbpf_rs::MapFlags::ANY)?;
    }

    Ok(())
}

pub fn load_firewall(ports: Vec<u16>) -> Result<()> {
    bump_memlock_rlimit()?;

    let builder = BlockSkelBuilder::default();
    let mut skel = builder.open()?.load()?;
    let mut progs = skel.progs_mut();

    let mut link = progs
        .allow_xdp()
        .attach_xdp(2)?;
    link.pin(FIREWALL_FILE)?;

    let ro = skel.rodata();
    let max_ports: usize = ro.MAX_PORTS as usize;

    if ports.len() > max_ports {
        link.unpin()?;
        link.detach()?;
        bail!("too many ports");
    }

    let maps = skel.maps_mut();
    load_ports(ports, maps)?;

    Ok(())
}


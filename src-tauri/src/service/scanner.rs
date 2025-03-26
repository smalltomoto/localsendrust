use crate::models::device::Device;
use local_ip_address::local_ip;
use std::net::{IpAddr, Ipv4Addr};
use std::process::Stdio;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::vec;
use tokio::process::Command;
use tokio::time::{sleep, Duration};

async fn ping(ip: &str) -> bool {
    let output = Command::new("ping")
        .args(["-n", "1", "-w", "4", ip])
        .creation_flags(0x08000000)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .output()
        .await;

    match output {
        Ok(out) => out.status.success(),
        Err(_e) => false,
    }
}

async fn scan_local_network() -> Vec<Device> {
    let local_ip = match local_ip().ok() {
        Some(IpAddr::V4(ip)) => ip,
        _ => panic!("无法获取本机ID"),
    };

    let subnet = Ipv4Addr::new(
        local_ip.octets()[0],
        local_ip.octets()[1],
        local_ip.octets()[2],
        0,
    );

    let mut devices = Vec::new();
    let mut handles: Vec<tokio::task::JoinHandle<Option<Device>>> = vec![];
    let sorting = Arc::new(AtomicUsize::new(1));

    for i in 1..254 {
        let sorting = sorting.clone();
        let target_ip = Ipv4Addr::new(
            subnet.octets()[0],
            subnet.octets()[1],
            subnet.octets()[2],
            i,
        );

        let ip_str = target_ip.to_string();

        handles.push(tokio::spawn(async move {
            if ping(&ip_str).await {
                let id = sorting.fetch_add(1, Ordering::SeqCst);
                Some(Device {
                    id,
                    name: format!("设备 {}", id),
                    ip: ip_str,
                    online: true,
                })
            } else {
                None
            }
        }));

        sleep(Duration::from_millis(5)).await;
    }
    for handle in handles {
        if let Ok(Some(device)) = handle.await {
            devices.push(device);
        }
    }
    devices
}

pub async fn scan_device() -> Vec<Device> {
    scan_local_network().await
}

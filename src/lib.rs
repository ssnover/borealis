use futures_util::{future::FutureExt, pin_mut, select, stream::StreamExt};
use mdns::{Record, RecordKind};
use std::error::Error;
use std::net::IpAddr;
use std::time::Duration;

const NANOLEAF_SERVICE: &'static str = "_nanoleafapi._tcp.local";

pub async fn discover_aurora(query_timeout: Duration) -> Result<IpAddr, Box<dyn Error>> {
    let t1 = timeout(query_timeout).fuse();
    let t2 = get_aurora_ip_address().fuse();
    pin_mut!(t1, t2);

    select! {
        (addr) = t2 => { return addr; },
        () = t1 => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::TimedOut,
                "No devices found",
            )));
        },
    }
}

async fn timeout(duration: Duration) {
    std::thread::sleep(duration);
}

async fn get_aurora_ip_address() -> Result<IpAddr, Box<dyn Error>> {
    let stream = mdns::discover::all(NANOLEAF_SERVICE, Duration::from_secs(5))?.listen();
    pin_mut!(stream);

    while let Some(Ok(response)) = stream.next().await {
        println!("Got response to MDNS query.");
        let addr = response.records().filter_map(self::to_ip_addr).next();
        if let Some(addr) = addr {
            return Ok(addr);
        }
    }
    Err(Box::new(std::io::Error::new(
        std::io::ErrorKind::Other,
        "Failed to query",
    )))
}

fn to_ip_addr(record: &Record) -> Option<IpAddr> {
    match record.kind {
        RecordKind::A(addr) => Some(addr.into()),
        RecordKind::AAAA(addr) => Some(addr.into()),
        _ => None,
    }
}

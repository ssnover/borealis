use futures_util::{pin_mut, stream::StreamExt};
use mdns::{Record, RecordKind};
use std::error::Error;
use std::net::Ipv4Addr;
use tokio::time::timeout;
use tokio::time::Duration;

const NANOLEAF_SERVICE: &'static str = "_nanoleafapi._tcp.local";

pub async fn discover_aurora(query_timeout: Duration) -> Result<Ipv4Addr, Box<dyn Error>> {
    timeout(query_timeout, get_aurora_ip_address()).await?
}

async fn get_aurora_ip_address() -> Result<Ipv4Addr, Box<dyn Error>> {
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

fn to_ip_addr(record: &Record) -> Option<Ipv4Addr> {
    match record.kind {
        RecordKind::A(addr) => Some(addr.into()),
        _ => None,
    }
}

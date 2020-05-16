use borealis::client::Aurora;
use std::net::Ipv4Addr;

#[tokio::main]
async fn main() {
    let aurora = Aurora::new(
        Ipv4Addr::new(192, 168, 1, 19),
        None,
        "RTpfvilFvOgYYKBLWqfJClz3fZc9Ws9N".to_string(),
    )
    .unwrap();
    println!("Firmware Version: {}", aurora.get_firmware_version().await.unwrap());

    //aurora.set_brightness(0, Some(std::time::Duration::from_secs(0)));
    //aurora.set_brightness(100, Some(std::time::Duration::from_secs(5)));

    //println!("Effects Available: {:?}", aurora.get_effects().unwrap());

    for effect in aurora.get_effects().await.unwrap() {
        println!("Changing to effect: {}", &effect);
        aurora.set_effect(effect).await;
        std::thread::sleep(std::time::Duration::from_secs(10));
    }

    aurora.set_effect("Forest".to_string()).await;
}

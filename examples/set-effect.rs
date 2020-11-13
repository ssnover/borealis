use borealis::Aurora;
use std::env;
use std::net::Ipv4Addr;

#[tokio::main]
pub async fn main() {
    let effect_name: String = env::args()
        .skip(1)
        .next()
        .expect("Must specify effect name to display.");

    let aurora = Aurora::new(
        Ipv4Addr::new(192, 168, 1, 12),
        None,
        &"I8NTBbt5IsFhZ5yAuSaa38m9j70m4odx".to_string(),
    );

    tokio::spawn(async move {
        println!("Setting Aurora to display {}", effect_name);
        aurora.turn_on().await.unwrap();
        aurora.set_effect(&effect_name).await.unwrap();
    })
    .await
    .unwrap();
}

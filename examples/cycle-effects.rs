use borealis::Aurora;
use std::env;
use std::time::Duration;

#[tokio::main]
pub async fn main() {
    let args: Vec<String> = env::args().collect();
    let cycle_period_secs = args[1].parse::<u64>().expect("Invalid arguments.");

    let aurora = Aurora::new("192.168.1.12:16021", "I8NTBbt5IsFhZ5yAuSaa38m9j70m4odx").unwrap();

    tokio::spawn(async move {
        loop {
            cycle_effects(&aurora, Duration::from_secs(cycle_period_secs)).await;
        }
    })
    .await
    .unwrap();
}

async fn cycle_effects(aurora: &Aurora<'_>, effect_period: Duration) {
    let effects = aurora.get_effects().await.unwrap();
    for effect in &effects {
        println!("Changing to effect: {}", &effect);
        aurora.set_effect(effect).await.unwrap();
        std::thread::sleep(effect_period);
    }
}

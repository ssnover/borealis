use borealis::Aurora;

#[tokio::main]
pub async fn main() {
    let effect_name: String = std::env::args()
        .skip(1)
        .next()
        .expect("Must specify effect name to display.");

    let aurora = Aurora::new("192.168.1.12:16021", "I8NTBbt5IsFhZ5yAuSaa38m9j70m4odx").unwrap();

    println!("Setting Aurora to display {}", effect_name);
    aurora.turn_on().await.unwrap();
    aurora.set_effect(&effect_name).await.unwrap();
}

use turbo_rl::mtg_env::play_mtg;

#[tokio::main]
async fn main() {
    play_mtg().await.expect("TODO: panic message");
}
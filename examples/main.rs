use turbo_rl::rl_agents::Agent;

#[tokio::main]
async fn main() {
    let mut mtg_agent = Agent::new(0.9, 0.1, 0.1, 2);
    mtg_agent.train().await.expect("TODO: panic message");
}
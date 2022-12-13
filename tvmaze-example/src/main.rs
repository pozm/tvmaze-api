use tvmaze_api::{TvMazeClient, ShowLookup, cacher::InMemoryCacher};

#[tokio::main]
async fn main() {
    let imc = InMemoryCacher::new(None);
    let client = TvMazeClient::new(Box::new(imc));
    let show = client.get_show(61351).await.unwrap();
    let episodes = show.episodes().await.unwrap();
    for ep in episodes {
        println!("{}x{}: {}", ep.season, ep.number, ep.name);
    }
}

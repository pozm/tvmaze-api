use tvmaze_api::{TvMazeClient, ShowLookup, cacher::InMemoryCacher};

#[tokio::main]
async fn main() {
    let imc = InMemoryCacher::new();
    let client = TvMazeClient::new(Box::new(imc));
    let show = client.lookup_show(ShowLookup::TVDB(397934)).await.unwrap();
    let show = client.lookup_show(ShowLookup::TVDB(397934)).await.unwrap();
    let episodes = show.episodes().await.unwrap();
    for ep in episodes {
        println!("{:?}", ep.name);
    }
}

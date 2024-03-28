use gloo::timers::future::TimeoutFuture;
use share::GameCover as GameCoverModel;

pub async fn get_my_games() -> Vec<GameCoverModel> {
    TimeoutFuture::new(1_000).await;

    vec![
        Default::default(),
        Default::default(),
        Default::default(),
        Default::default(),
        Default::default(),
        Default::default(),
        Default::default(),
        Default::default(),
        Default::default(),
        Default::default(),
        Default::default(),
    ]
}

pub async fn get_installed_games() -> Vec<GameCoverModel> {
    TimeoutFuture::new(1_000).await;

    vec![
        Default::default(),
        Default::default(),
    ]
}

pub async fn check_installed(name: String) -> bool {
    _ = name;
    TimeoutFuture::new(1_00).await;
    false
}


pub async fn launch_game(name: String) {
    _ = name;
    TimeoutFuture::new(2_000).await;
}
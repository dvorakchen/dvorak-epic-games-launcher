use super::url_fn;
use gloo::timers::future::TimeoutFuture;
use share::{GameCover as GameCoverModel, Review, ServerResponse};

url_fn!("games");

pub async fn get_in_library_games(username: impl AsRef<str>) -> Vec<GameCoverModel> {
    let username = username.as_ref();
    TimeoutFuture::new(1_000).await;

    let games = reqwest::get(scope_url(username))
        .await
        .unwrap()
        .json::<ServerResponse<Vec<GameCoverModel>>>()
        .await
        .unwrap();

    games.content.unwrap_or(Vec::new())
}

pub async fn get_installed_games() -> Vec<GameCoverModel> {
    TimeoutFuture::new(1_000).await;

    vec![Default::default(), Default::default()]
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

pub async fn get_recently_reviews(game_id: String) -> Vec<Review> {
    _ = game_id;

    TimeoutFuture::new(2_000).await;

    vec![
        Review {
            id: "000".to_string(),
            username: "Dvorak Chen".to_string(),
            review_type: share::ReviewType::ThumbsUp,
            content: "The great Chinese game ever seen".to_string(),
            datetime: "2024-01-01".to_string(),
        },
        Review {
            id: "001".to_string(),
            username: "BoyNextDoor".to_string(),
            review_type: share::ReviewType::ThumbsUp,
            content: r"I will leave the cat here, so that everybody who passes by can pet it and give it a thumbs up and awards.
　　　 　　／＞　　フ
　　　 　　| 　_　 _ l
　 　　 　／` ミ＿xノ
　　 　 /　　　 　 |
　　　 /　 ヽ　　 ﾉ
　 　 │　　|　|　|
　／￣|　　 |　|　|
　| (￣ヽ＿_ヽ_)__)
　＼二つ".to_string(),
            datetime: "2024-01-02".to_string(),
        },
        Review {
            id: "002".to_string(),
            username: "DarkKing".to_string(),
            review_type: share::ReviewType::ThumbsDown,
            content: "No sex with Monkey".to_string(),
            datetime: "2024-01-03".to_string(),
        },
    ]
}

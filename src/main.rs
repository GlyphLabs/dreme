use actix_web::{get, http, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use cached::proc_macro::cached;
use env_logger;
use reqwest::Client;
use rand::seq::SliceRandom;

mod types;

const DEFAULT_SUBREDDITS: [&str; 6] = [
    "memes",
    "dankmemes",
    "funny",
    "antimeme",
    "wholesomememes",
    "me_irl"
];

struct AppState {
    client: Client,
}

#[cached(
    size = 10,
    time = 3600,
    time_refresh = true,
    key = "String",
    convert = r#"{ format!("{}", subreddit) }"#,
    result = true
)]
async fn get_memes_from_subreddt(
    http: &Client,
    subreddit: String,
) -> Result<Vec<types::Post>, Box<dyn std::error::Error>> {
    let hot_posts_res = http
        .get(format!("https://reddit.com/r/{}/hot.json", subreddit))
        .query(&[
            ("limit", 100),
        ])
        .send()
        .await?;
    if hot_posts_res.status() != 200 {
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("subreddit r/{} was not found", subreddit))))
    }
    let reddit_data: types::RedditResponse = hot_posts_res.json::<types::RedditResponse>().await.unwrap();
    let memes = reddit_data.data.children.iter().map(|child| child.data.clone()).collect::<Vec<types::Post>>();
    Ok(memes)
}

async fn get_random_meme(http: &Client, subreddit: String) -> Result<Vec<types::Post>, Box<dyn std::error::Error>> {
    let meme = get_memes_from_subreddt(http, subreddit).await;
    if meme.is_ok() {
        return Ok(vec![meme.unwrap().choose(&mut rand::thread_rng()).unwrap().clone()])
    } else {
        return Err(meme.err().unwrap())
    }
}

#[get("/")]
async fn index() -> Result<HttpResponse, http::Error> {
    Ok(HttpResponse::Ok().body("Hello World!"))
}

#[get("/{user}")]
async fn user(data: web::Data<AppState>, subreddit: web::Path<String>) -> impl Responder {
    let res = get_random_meme(&data.client, subreddit.to_string()).await;
    if res.is_ok() {
        return HttpResponse::Ok().json(res.unwrap());
    } else {
        return HttpResponse::InternalServerError().body(res.err().expect("Error").to_string());
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    println!("[i] starting on port {}", 8080);
    let temp_client = Client::new();
    for subreddit in DEFAULT_SUBREDDITS.iter() {
        println!("[i] fetching memes from r/{}", subreddit.to_string());
        let _ = get_memes_from_subreddt(&temp_client, subreddit.to_string()).await;
    };
    drop(temp_client);
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                client: Client::new(),
            }))
            .wrap(Logger::default())
            .service(user)
            .service(index)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
use std::borrow::BorrowMut;
use std::env;

use std::sync::Arc;
use std::time::{Duration, Instant};

use axum::extract::{RawQuery, State};
use axum::Json;
use axum::{routing::get, Router};

use reqwest::header::CONTENT_TYPE;

use serenity::async_trait;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{CommandResult, StandardFramework};
use serenity::model::channel::Message;
use serenity::model::prelude::{
    ChannelId, ForumTagId, GuildChannel, GuildId, Ready, ReactionType,
};
use serenity::prelude::*;

use tokio::sync::Mutex;
use tower_http::cors::{Any, CorsLayer};

use lazy_static::lazy_static;

use serde::Serialize;

use reqwest;

lazy_static! {
    static ref BOT_TOKEN: String = env::var("BOT_TOKEN").unwrap();
    static ref FORUM_ID: u64 = env::var("FORUM_ID").unwrap().parse::<u64>().unwrap();
    static ref GUILD_ID: u64 = env::var("GUILD_ID").unwrap().parse::<u64>().unwrap();
    static ref CACHE_TIME: u64 = env::var("CACHE_TIME").unwrap().parse::<u64>().unwrap();
}

#[group]
#[commands(ping)]
struct General;

struct Handler;

#[allow(non_snake_case)]
#[derive(Clone, Serialize, Debug)]
pub struct Cache {
    pub tags: Vec<Tag>,
    pub items: Vec<Media>,
}

#[derive(Clone, Serialize, Debug)]
pub struct Tag {
    pub id: ForumTagId,
    pub name: String,
}

#[derive(Clone, Serialize, Debug)]
pub struct Media {
    pub src: String,
    pub tags: Vec<Tag>,
    pub media_type: String,
}

#[allow(non_snake_case)]
pub struct DB {
    pub context: Context,
    pub cache: Cache,
    pub cache_t: Instant,
}

impl DB {
    pub fn default(ctx: Context) -> DB {
        return DB {
            context: ctx,
            cache: Cache {
                tags: Vec::new(),
                items: Vec::new(),
            },
            cache_t: Instant::now() - Duration::from_secs(*CACHE_TIME),
        };
    }
}

// pub type AppState = Arc<Mutex<Vec<DB>>>;
//
// pub fn app_state() -> AppState {
// Arc::new(Mutex::new(Vec::new()))
// }

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _data_about_bot: Ready) {}
    async fn cache_ready(&self, ctx: Context, _guilds: Vec<GuildId>) {
        let state = Arc::new(Mutex::new(DB::default(
            ctx.clone()
        )));

        tokio::spawn(async move {
            loop {
                println!("initialising api");
                let cors = CorsLayer::new().allow_origin(Any);
                let app = Router::new()
                    .route("/v1/links", get(get_links))
                    .with_state(Arc::clone(&state))
                    .layer(cors);
                let server = axum::Server::bind(&"0.0.0.0:3002".parse().unwrap())
                    .serve(app.into_make_service());
                server.await;
            }
        });
        println!("started");
    }
}

#[tokio::main]
async fn main() {
    // start listening for events by starting a single shard
    // get_links(ctx, channel);

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment

    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(BOT_TOKEN.as_str(), intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    };
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}

#[axum::debug_handler]
async fn get_links(opts: Option<RawQuery>, State(state): State<Arc<Mutex<DB>>>) -> Json<Cache> {
    let mut s = state.lock().await;
    let _ = opts;

    if CACHE_TIME.lt(&s.cache_t.elapsed().as_secs()) {
        println!("refreshing cache with {:?}", s.cache_t.elapsed().as_secs());
        let (items, tags) = gen_links(&s.context).await;
        s.cache = Cache {
            items: items,
            tags: tags,
        };
        s.cache_t = Instant::now();
    }
    println!("{:?}", Json(s.cache.items.clone()));
    return Json(s.cache.clone());
}

async fn gen_links(ctx: &Context) -> (Vec<Media>, Vec<Tag>) {
    println!("Link requested");
    let tags = gen_tags(&ctx).await;
    let archived_channels = ChannelId(*FORUM_ID)
        .get_archived_public_threads(&ctx.http, None, None)
        .await
        .unwrap();
    let active_channels = GuildId(*GUILD_ID)
        .get_active_threads(&ctx.http)
        .await
        .unwrap();
    // let threads : Vec<(&ChannelId, &GuildChannel)>= channels.iter().filter(|x|
    // x.1.kind == ChannelType::
    //
    // ).collect();
    let mut medias: Vec<Media> = vec![];
    for t in archived_channels.threads.iter().chain(
        active_channels
            .threads
            .iter()
            .filter(|r| r.parent_id.unwrap() == *FORUM_ID),
    ) {
        medias.extend_from_slice(get_attachments(ctx, t, &tags).await.borrow_mut());
    }

    (medias, tags)
}

async fn gen_tags(ctx: &Context) -> Vec<Tag> {
    let results = ctx.http.get_channel(*FORUM_ID).await;
    results
        .unwrap()
        .guild()
        .unwrap()
        .available_tags
        .iter()
        .map(|y| Tag {
            id: y.id,
            name: y.name.clone(),
        })
        .collect()
}

async fn get_attachments(ctx: &Context, c: &GuildChannel, tags: &Vec<Tag>) -> Vec<Media> {
    let mut medias = Vec::new();
    let message = c.messages(&ctx.http, |r| r).await.expect("No permission");

    for attach in message.into_iter().fold(vec![], |a, b| {
        if b.reactions.into_iter().map(|r| r.reaction_type).any(|x| "▪\u{fe0f}".parse::<ReactionType>().unwrap() == x.clone()){
            return a
        };
        return [a, b.attachments].concat();
    }) {
        
        medias.push(Media {
            src: attach.url.clone(),
            tags: c
                .applied_tags
                .clone()
                .into_iter()
                .map(|tag| {
                    let name;
                    if let Some(t) = tags.into_iter().find(|f| f.id == tag) {
                        name = t.name.clone();
                    } else {
                        name = "errored".to_string();
                    }
                    Tag {
                        id: tag,
                        name: name, // name: "thing".to_string()
                    }
                })
                .collect(),
            media_type: is_video(attach.url).await,
        })
    }
    medias
}

async fn is_video(url: String) -> String {
    reqwest::get(url)
        .await
        .expect("Something wrong with requests")
        .headers()
        .get(CONTENT_TYPE)
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}

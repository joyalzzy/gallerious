use std::borrow::BorrowMut;
use std::env;
use std::ops::ControlFlow;
use std::time::Instant;

use axum::extract::{RawQuery, State};
use axum::Json;
use axum::{routing::get, Router};

use serenity::async_trait;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{CommandResult, StandardFramework};
use serenity::model::channel::Message;
use serenity::model::prelude::{ChannelId, GuildChannel, GuildId, Ready, MessageReaction, Reaction, ReactionType};
use serenity::prelude::*;

use tokio::time::Interval;
use tower_http::cors::{Any, CorsLayer};

use lazy_static::lazy_static;

lazy_static! {
    static ref BOT_TOKEN: String = env::var("BOT_TOKEN").unwrap();
    static ref FORUM_ID: u64 = env::var("FORUM_ID").unwrap().parse::<u64>().unwrap();
    static ref GUILD_ID: u64 = env::var("GUILD_ID").unwrap().parse::<u64>().unwrap();
}

#[group]
#[commands(ping)]
struct General;

struct Handler;

#[allow(non_snake_case)]
#[derive(Clone)]
pub struct DB {
    pub context: Context,
    pub cache: Json<Vec<String>>,
    pub cache_t: Instant
}
impl DB {
    pub fn default (ctx: Context, cache: Json<Vec<String>>)-> DB {
        return DB {
            context: ctx,
            cache: cache,
            cache_t: Instant::now()
        }
    }
}

// pub type AppState = Arc<Mutex<Vec<DB>>>;
//
// pub fn app_state() -> AppState {
// Arc::new(Mutex::new(Vec::new()))
// }

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _data_about_bot: Ready) {

    }
    async fn cache_ready(&self, ctx: Context, _guilds: Vec<GuildId>) {
        tokio::spawn(async move {
            loop {
                println!("initialising api");
                let state = DB::default(ctx.clone(), Json(gen_links(&ctx).await));
                let cors = CorsLayer::new().allow_origin(Any);
                let app = Router::new()
                    .route("/v1/links", get(get_links))
                    .with_state(state)
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

async fn get_links(opts: Option<RawQuery>, State(mut state): State<DB>) -> Json<Vec<String>> {
    if state.cache_t.elapsed().as_secs() > 300 {
        println!("refreshing cache");
        let _ = opts;
        state.cache = Json(gen_links(&state.context).await) ;
        state.cache_t = Instant::now();
    }
    return state.cache;
    
}

async fn gen_links(ctx: &Context) -> Vec<String> {
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
    let mut urls = vec![];
    for t in archived_channels.threads.iter().chain(
        active_channels
            .threads
            .iter()
            .filter(|r| r.parent_id.unwrap() == *FORUM_ID),
    ) {
        urls.extend_from_slice(get_attachments(ctx, t).await.borrow_mut());
    }
    urls
    // for msg in messages {
    // if let img  = Some(msg.attachments).unwrap() {
    // img.iter().for_each(|f| println!("{:?}", f.proxy_url));
    // }
    // }
}

async fn get_attachments(ctx: &Context, c: &GuildChannel) -> Vec<String> {
    let mut urls = Vec::new();
    c.messages(&ctx.http, |r| r)
        .await
        .into_iter()
        .for_each(|f| {
            f.into_iter()
                .for_each(|g| {
                    let mut blacklisted = false;
                    g.reactions.into_iter().for_each(|x: MessageReaction|  {
                        // println!("{:?}", x.reaction_type);
                        if (x.reaction_type == "▪\u{fe0f}".parse().unwrap()) {
                            // println!("blacklisted");
                            blacklisted = true;
                            return;
                        }
                    }); 
                    if blacklisted {
                        return;
                    }
                    g.attachments.into_iter().for_each(|h| urls.push(h.url))
                })
        });
    urls
}

#[macro_use]
extern crate dotenvy_macro;

use std::borrow::BorrowMut;

use axum::extract::{RawQuery, State};
use axum::Json;
use axum::{routing::get, Router};

use serenity::async_trait;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{CommandResult, StandardFramework};
use serenity::model::channel::Message;
use serenity::model::prelude::{ChannelId, GuildChannel, GuildId, Ready};
use serenity::prelude::*;

use tower_http::cors::{Any, CorsLayer};

#[group]
#[commands(ping)]
struct General;

struct Handler;

#[allow(non_snake_case)]
#[derive(Clone)]
pub struct DB {
    pub context: Context,
}

// pub type AppState = Arc<Mutex<Vec<DB>>>;
//
// pub fn app_state() -> AppState {
// Arc::new(Mutex::new(Vec::new()))
// }

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _data_about_bot: Ready) {
        println!("started");
    }
    async fn cache_ready(&self, ctx: Context, _guilds: Vec<GuildId>) {
        tokio::spawn(async move {
            loop {
                let state = DB {
                    context: ctx.clone(),
                };
                let cors = CorsLayer::new().allow_origin(Any);
                let app = Router::new().route("/v1/links", get(get_links)).with_state(state).layer(cors);
                let server = axum::Server::bind(&"0.0.0.0:3002".parse().unwrap())
                    .serve(app.into_make_service());
                server.await;
            }
        });
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

    let token = dotenv!("DISCORD_TOKEN");

    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(token, intents)
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

async fn get_links(opts: Option<RawQuery>, State(state): State<DB>) -> Json<Vec<String>> {
    let _ = opts;
    Json(gen_links(&state.context).await)
}

async fn gen_links(ctx: &Context) -> Vec<String> {
    let id = dotenv!("FORUM_ID").parse::<u64>().unwrap();
    let gid = dotenv!("GID").parse::<u64>().unwrap();
    let archived_channels = ChannelId(id)
        .get_archived_public_threads(&ctx.http, None, None)
        .await
        .unwrap();
    let active_channels = GuildId(gid).get_active_threads(&ctx.http).await.unwrap();
    // let threads : Vec<(&ChannelId, &GuildChannel)>= channels.iter().filter(|x|
    // x.1.kind == ChannelType::
    //
    // ).collect();
    let mut urls = vec![];
    for t in archived_channels.threads.iter().chain(
        active_channels
            .threads
            .iter()
            .filter(|r| r.parent_id.unwrap() == id),
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
    return c
        .messages(&ctx.http, |r| r)
        .await
        .iter()
        .map(|r| {
            r.iter()
                .map(|a| a.attachments.iter().map(|f| f.url.clone()).collect())
        })
        .flatten()
        .filter(|r: &String| !r.is_empty())
        .collect();
}

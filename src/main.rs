mod commands;
mod utils;

use poise::{
    samples::{register_globally, register_in_guild},
    serenity_prelude::{self as serenity, FullEvent},
};
use sea_orm::{prelude::*, IntoActiveModel, IntoSimpleExpr, Set, UpdateResult};
use tracing::{debug, info};
use utils::{
    db::get_member,
    xp::{can_earn_xp, level_up, xp_from_message},
};

async fn event_handler(
    ctx: &poise::serenity_prelude::Context,
    event: &poise::serenity_prelude::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    #[allow(clippy::single_match)]
    match event {
        FullEvent::Message { new_message: msg } => {
            // check if the channel or thread is directly allowed to earn xp
            if !can_earn_xp(&data.db, msg.channel_id).await? {
                if let Some(channel) = msg.channel(&ctx).await?.guild() {
                    let parent = channel.parent_id;
                    if let Some(parent) = parent {
                        // check if the forum channel or category is allowed to earn xp
                        if !can_earn_xp(&data.db, parent).await? {
                            if let Some(channel) = parent.to_channel(&ctx).await?.guild() {
                                let parent = channel.parent_id;
                                if let Some(parent) = parent {
                                    // check if the category is allowed to earn xp
                                    // this will only happen if msg.channel is a thread
                                    if !can_earn_xp(&data.db, parent).await? {
                                        return Ok(());
                                    }
                                }
                            }
                        }
                    }
                }
            }
            let xp = xp_from_message(msg.content.as_str());
            let mut member = get_member(&data.db, msg.author.id)
                .await?
                .into_active_model();
            member.xp = Set(member.xp.unwrap() + xp);
            let leveled_up = level_up(&mut member);
            let level = member.level.clone().unwrap();
            member.save(&data.db).await?;
            if leveled_up {
                let _ = msg
                    .reply_ping(
                        &ctx.http,
                        format!(
                            "Congratulations {}! You have reached level {}!",
                            msg.author.id, level
                        ),
                    )
                    .await;
            }
        },
        _ => {},
    }
    Ok(())
}

#[tracing::instrument]
#[allow(clippy::inconsistent_digit_grouping)]
async fn income(db: &DatabaseConnection) -> Result<UpdateResult, sea_orm::DbErr> {
    entity::member::Entity::update_many()
        .col_expr(
            entity::member::Column::Balance,
            entity::member::Column::Balance
                .into_simple_expr()
                .add(100_000_000_00_i64),
        )
        .exec(db)
        .await
}

#[derive(Debug)]
pub struct Data {
    pub db: sea_orm::DatabaseConnection,
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Ctx<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::builder()
                .with_default_directive(tracing::Level::INFO.into())
                .from_env_lossy(),
        )
        .init();
    let token = std::env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let db_url = std::env::var("DATABASE_URL").expect("Expected a database URL in the environment");

    let mut opt = sea_orm::ConnectOptions::new(db_url);
    opt.sqlx_logging_level(tracing::log::LevelFilter::Trace);
    let db = sea_orm::Database::connect(opt).await.unwrap();
    let db2 = db.clone();

    tokio::spawn(async move {
        loop {
            let now = chrono::Utc::now();
            let next_midnight = now.date_naive().succ_opt().unwrap();
            let next_midnight = next_midnight.and_hms_opt(0, 0, 0).unwrap();
            let duration = next_midnight - now.naive_utc();
            info!("Next midnight: {}", next_midnight);
            debug!("Sleeping for {} seconds", duration.num_seconds());
            tokio::time::sleep(tokio::time::Duration::from_secs(
                duration.num_seconds() as u64
            ))
            .await;
            let _ = income(&db2).await;
        }
    });

    let framework = poise::Framework::builder()
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                if let Some(guild_id) = std::env::var("GUILD_ID").ok().map(|s| s.parse().unwrap()) {
                    register_in_guild(ctx, &framework.options().commands, guild_id).await?;
                } else {
                    register_globally(ctx, &framework.options().commands).await?;
                }
                Ok(Data { db })
            })
        })
        .options(poise::FrameworkOptions {
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },
            on_error: |error| {
                Box::pin(async move {
                    tracing::error!("Error: {:?}", error);
                })
            },
            pre_command: |_ctx| {
                Box::pin(async move {
                    // let txn = ctx.data().db.begin().await.unwrap();
                    // ctx.set_invocation_data(txn).await;
                })
            },
            post_command: |_ctx| {
                Box::pin(async move {
                    // let txn = ctx.invocation_data::<DatabaseTransaction>().
                    // await.unwrap(); txn.deref().commit().
                    // await.unwrap();
                })
            },
            commands: commands::commands(),
            ..Default::default()
        })
        .build();

    let mut client = poise::serenity_prelude::ClientBuilder::new(
        token,
        serenity::prelude::GatewayIntents::all(),
    )
    .framework(framework)
    .await
    .expect("Failed to create client");

    client.start().await.unwrap();
}

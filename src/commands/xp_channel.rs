use poise::serenity_prelude::{Channel, ChannelId, Mention};
use sea_orm::{EntityTrait, IntoActiveModel, Set};

use crate::{
    utils::{
        embed::{not_admin, Embed},
        perms::admin,
    },
    Ctx,
    Data,
};

#[poise::command(slash_command, rename = "add-xp-channel")]
pub async fn add_xp_channel(ctx: Ctx<'_>, channel: Channel) -> Result<(), crate::Error> {
    if !admin(&ctx).await? {
        return not_admin(&ctx).await;
    }
    let xp_channel = entity::xp_channel::Entity::find_by_id(channel.id().to_string())
        .one(&ctx.data().db)
        .await?;
    if xp_channel.is_some() {
        Embed::error(&ctx)
            .description(format!("{} is already an XP channel.", channel))
            .send(&ctx)
            .await
    } else {
        let xp_channel = entity::xp_channel::ActiveModel {
            id: Set(channel.id().to_string()),
        };
        entity::xp_channel::Entity::insert(xp_channel)
            .exec(&ctx.data().db)
            .await?;
        Embed::success(&ctx)
            .description(format!("Added {} as an XP channel.", channel))
            .send(&ctx)
            .await
    }
}

#[poise::command(slash_command, rename = "remove-xp-channel")]
pub async fn remove_xp_channel(ctx: Ctx<'_>, channel: Channel) -> Result<(), crate::Error> {
    if !admin(&ctx).await? {
        return not_admin(&ctx).await;
    }
    let xp_channel = entity::xp_channel::Entity::find_by_id(channel.id().to_string())
        .one(&ctx.data().db)
        .await?;
    if let Some(xp_channel) = xp_channel {
        entity::xp_channel::Entity::delete(xp_channel.into_active_model())
            .exec(&ctx.data().db)
            .await?;
        Embed::success(&ctx)
            .description(format!("Removed {} as an XP channel.", channel))
            .send(&ctx)
            .await
    } else {
        Embed::error(&ctx)
            .description(format!("{} is not an XP channel.", channel))
            .send(&ctx)
            .await
    }
}

#[poise::command(slash_command, rename = "check-xp-channel")]
pub async fn check_xp_channel(ctx: Ctx<'_>, channel: Channel) -> Result<(), crate::Error> {
    if !admin(&ctx).await? {
        return not_admin(&ctx).await;
    }
    let xp_channel = entity::xp_channel::Entity::find_by_id(channel.id().to_string())
        .one(&ctx.data().db)
        .await?;
    if xp_channel.is_some() {
        Embed::success(&ctx)
            .description(format!("{} is an XP channel.", channel))
            .send(&ctx)
            .await
    } else {
        Embed::error(&ctx)
            .description(format!("{} is not an XP channel.", channel))
            .send(&ctx)
            .await
    }
}

#[poise::command(slash_command, rename = "list-xp-channels")]
pub async fn list_xp_channels(ctx: Ctx<'_>) -> Result<(), crate::Error> {
    if !admin(&ctx).await? {
        return not_admin(&ctx).await;
    }
    let xp_channels = entity::xp_channel::Entity::find()
        .all(&ctx.data().db)
        .await?
        .into_iter()
        .map(|xp_channel| {
            std::convert::Into::<Mention>::into(xp_channel.id.parse::<ChannelId>().unwrap())
                .to_string()
        })
        .collect::<Vec<_>>();
    if xp_channels.is_empty() {
        return Embed::info(&ctx)
            .description("No XP channels.")
            .send(&ctx)
            .await;
    }
    Embed::info(&ctx)
        .description(xp_channels.join("\n"))
        .send(&ctx)
        .await
}

pub fn commands() -> Vec<poise::Command<Data, crate::Error>> {
    vec![
        add_xp_channel(),
        remove_xp_channel(),
        check_xp_channel(),
        list_xp_channels(),
    ]
}

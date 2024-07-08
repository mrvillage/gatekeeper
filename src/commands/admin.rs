use poise::serenity_prelude::{Mention, User, UserId};
use sea_orm::{prelude::*, ActiveValue, IntoActiveModel, QueryOrder};

use crate::{
    utils::{
        db::get_member,
        embed::{not_admin, not_owner, Embed},
        perms::{admin, is_owner},
    },
    Ctx,
    Data,
};

#[poise::command(
    slash_command,
    rename = "add-admin",
    default_member_permissions = "ADMINISTRATOR"
)]
pub async fn add_admin(ctx: Ctx<'_>, user: User) -> Result<(), crate::Error> {
    if !is_owner(&ctx.author().id.to_string()) {
        return not_owner(&ctx).await;
    }
    let user_id = user.id.to_string();
    let mut mem = get_member(&ctx.data().db, &user_id)
        .await?
        .into_active_model();
    if mem.permissions.unwrap() == 1 {
        return Embed::error(&ctx)
            .description(format!("{} is already an admin.", user))
            .send(&ctx)
            .await;
    }
    mem.permissions = ActiveValue::Set(1);
    mem.save(&ctx.data().db).await?;
    Embed::success(&ctx)
        .description(format!("Added {} as an admin.", user))
        .send(&ctx)
        .await
}

#[poise::command(
    slash_command,
    rename = "remove-admin",
    default_member_permissions = "ADMINISTRATOR"
)]
pub async fn remove_admin(ctx: Ctx<'_>, user: User) -> Result<(), crate::Error> {
    if !is_owner(&ctx.author().id.to_string()) {
        return not_owner(&ctx).await;
    }
    let user_id = user.id.to_string();
    let mut mem = get_member(&ctx.data().db, &user_id)
        .await?
        .into_active_model();
    if mem.permissions.unwrap() == 0 {
        return Embed::error(&ctx)
            .description(format!("{} is not an admin.", user))
            .send(&ctx)
            .await;
    }
    mem.permissions = ActiveValue::Set(0);
    mem.save(&ctx.data().db).await?;
    Embed::success(&ctx)
        .description(format!("Removed {} as an admin.", user))
        .send(&ctx)
        .await
}

#[poise::command(
    slash_command,
    rename = "check-admin",
    default_member_permissions = "ADMINISTRATOR"
)]
pub async fn check_admin(ctx: Ctx<'_>, user: User) -> Result<(), crate::Error> {
    if !admin(&ctx).await? {
        return not_admin(&ctx).await;
    }
    let user_id = user.id.to_string();
    let mem = get_member(&ctx.data().db, &user_id).await?;
    Embed::info(&ctx)
        .description(if mem.permissions == 1 {
            format!("{} is an admin.", user)
        } else {
            format!("{} is not an admin.", user)
        })
        .send(&ctx)
        .await
}

#[poise::command(
    slash_command,
    rename = "list-admins",
    default_member_permissions = "ADMINISTRATOR"
)]
pub async fn list_admins(ctx: Ctx<'_>) -> Result<(), crate::Error> {
    if !admin(&ctx).await? {
        return not_admin(&ctx).await;
    }
    let admins = entity::member::Entity::find()
        .filter(entity::member::Column::Permissions.gte(1))
        .order_by_asc(entity::member::Column::Id)
        .all(&ctx.data().db)
        .await?
        .into_iter()
        .map(|m| std::convert::Into::<Mention>::into(m.id.parse::<UserId>().unwrap()).to_string())
        .collect::<Vec<String>>();
    if admins.is_empty() {
        return Embed::info(&ctx).description("No admins.").send(&ctx).await;
    }
    Embed::info(&ctx)
        .description(admins.join("\n"))
        .send(&ctx)
        .await
}

pub fn commands() -> Vec<poise::Command<Data, crate::Error>> {
    vec![add_admin(), remove_admin(), check_admin(), list_admins()]
}

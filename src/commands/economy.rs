use poise::serenity_prelude::User;
use sea_orm::{prelude::*, ActiveValue, IntoActiveModel, Set};

use crate::{
    utils::{
        db::get_member,
        embed::{not_admin, Embed},
        num::{money, to_money},
        perms::{admin, is_admin},
    },
    Ctx,
    Data,
};

#[poise::command(slash_command)]
pub async fn balance(ctx: Ctx<'_>, user: Option<User>) -> Result<(), crate::Error> {
    let user_id = ctx.author().id.to_string();
    let mut mem = get_member(&ctx.data().db, &user_id).await?;
    let mut author = ctx.author();
    if let Some(ref user) = user {
        if is_admin(&mem) {
            mem = get_member(&ctx.data().db, &user.id.to_string()).await?;
            author = user;
        } else {
            Embed::error(&ctx)
                .description("You do not have permission to view other users' balances.")
                .send(&ctx)
                .await?;
        }
    }
    Embed::info(&ctx)
        .author(author)
        .description(format!("Balance is: {}", money(mem.balance)))
        .send(&ctx)
        .await
}

#[poise::command(
    slash_command,
    rename = "add-money",
    default_member_permissions = "ADMINISTRATOR"
)]
pub async fn add_money(
    ctx: Ctx<'_>,
    user: User,
    #[min = 0] amount: f64,
) -> Result<(), crate::Error> {
    if !admin(&ctx).await? {
        return not_admin(&ctx).await;
    }
    let user_id = user.id.to_string();
    let mut member = get_member(&ctx.data().db, &user_id)
        .await?
        .into_active_model();
    let amount = to_money(amount);
    member.balance = ActiveValue::Set(member.balance.unwrap() + amount);
    member.save(&ctx.data().db).await?;
    Embed::success(&ctx)
        .description(format!("Added {} to {}'s balance.", money(amount), user))
        .send_pub(&ctx)
        .await
}

#[poise::command(
    slash_command,
    rename = "remove-money",
    default_member_permissions = "ADMINISTRATOR"
)]
pub async fn remove_money(
    ctx: Ctx<'_>,
    user: User,
    #[min = 0] amount: f64,
) -> Result<(), crate::Error> {
    if !admin(&ctx).await? {
        return not_admin(&ctx).await;
    }
    let user_id = user.id.to_string();
    let mut mem = get_member(&ctx.data().db, &user_id)
        .await?
        .into_active_model();
    let amount = to_money(amount);
    let balance = mem.balance.unwrap();
    if amount > balance {
        return Embed::error(&ctx)
            .description("Cannot remove more money than the user has.")
            .send(&ctx)
            .await;
    }
    mem.balance = ActiveValue::Set(balance - amount);
    mem.save(&ctx.data().db).await?;
    Embed::success(&ctx)
        .description(format!(
            "Removed {} from {}'s balance.",
            money(amount),
            user,
        ))
        .send_pub(&ctx)
        .await
}

#[poise::command(slash_command)]
pub async fn transfer(
    ctx: Ctx<'_>,
    user: User,
    #[min = 0] amount: f64,
) -> Result<(), crate::Error> {
    let mut me = get_member(&ctx.data().db, &ctx.author().id.to_string())
        .await?
        .into_active_model();
    let mut them = get_member(&ctx.data().db, &user.id.to_string())
        .await?
        .into_active_model();
    let amount = to_money(amount);
    if amount > me.balance.clone().unwrap() {
        return Embed::error(&ctx)
            .description("Cannot transfer more money than you have.")
            .send(&ctx)
            .await;
    }
    me.balance = Set(me.balance.unwrap() - amount);
    them.balance = Set(them.balance.unwrap() + amount);
    me.save(&ctx.data().db).await?;
    them.save(&ctx.data().db).await?;
    Embed::success(&ctx)
        .description(format!("Transferred {} to {}.", money(amount), user))
        .send_pub(&ctx)
        .await
}

#[poise::command(slash_command)]
pub async fn pay(ctx: Ctx<'_>, #[min = 0] amount: f64) -> Result<(), crate::Error> {
    let mut me = get_member(&ctx.data().db, &ctx.author().id.to_string())
        .await?
        .into_active_model();
    let amount = to_money(amount);
    if amount > me.balance.clone().unwrap() {
        return Embed::error(&ctx)
            .description("Cannot pay more money than you have.")
            .send(&ctx)
            .await;
    }
    me.balance = Set(me.balance.unwrap() - amount);
    me.save(&ctx.data().db).await?;
    Embed::success(&ctx)
        .description(format!("Paid {}.", money(amount)))
        .send_pub(&ctx)
        .await
}

pub fn commands() -> Vec<poise::Command<Data, crate::Error>> {
    vec![balance(), add_money(), remove_money(), transfer(), pay()]
}

use poise::serenity_prelude::{Mention, Role, RoleId};
use sea_orm::{prelude::*, EntityTrait, IntoActiveModel, QueryFilter, QueryOrder, Set};

use crate::{
    utils::{
        embed::{not_admin, Embed},
        perms::admin,
    },
    Ctx,
    Data,
};

#[poise::command(slash_command, rename = "add-xp-role")]
pub async fn add_xp_role(
    ctx: Ctx<'_>,
    role: Role,
    #[min = 1] level: i32,
) -> Result<(), crate::Error> {
    if !admin(&ctx).await? {
        return not_admin(&ctx).await;
    }
    let xp_role = entity::xp_role::Entity::find_by_id(role.id.to_string())
        .one(&ctx.data().db)
        .await?;
    if xp_role.is_some() {
        Embed::error(&ctx)
            .description(format!("{} is already an XP role.", role))
            .send(&ctx)
            .await
    } else {
        let xp_role = entity::xp_role::ActiveModel {
            id:    Set(role.id.to_string()),
            level: Set(level),
        };
        entity::xp_role::Entity::insert(xp_role)
            .exec(&ctx.data().db)
            .await?;
        ctx.defer_ephemeral().await?;
        let members = entity::member::Entity::find()
            .filter(entity::member::Column::Level.gte(level))
            .all(&ctx.data().db)
            .await?;
        for mem in members {
            ctx.http()
                .add_member_role(
                    ctx.data().primary_guild_id,
                    mem.id.parse().unwrap(),
                    role.id,
                    Some("Role added due to XP level."),
                )
                .await?;
        }
        Embed::success(&ctx)
            .description(format!("Added {} as an XP role at level {}.", role, level))
            .send(&ctx)
            .await
    }
}

#[poise::command(slash_command, rename = "remove-xp-role")]
pub async fn remove_xp_role(ctx: Ctx<'_>, role: Role) -> Result<(), crate::Error> {
    if !admin(&ctx).await? {
        return not_admin(&ctx).await;
    }
    let xp_role = entity::xp_role::Entity::find_by_id(role.id.to_string())
        .one(&ctx.data().db)
        .await?;

    if let Some(xp_role) = xp_role {
        entity::xp_role::Entity::delete(xp_role.clone().into_active_model())
            .exec(&ctx.data().db)
            .await?;
        ctx.defer_ephemeral().await?;
        let members = entity::member::Entity::find()
            .filter(entity::member::Column::Level.gte(xp_role.level))
            .all(&ctx.data().db)
            .await?;
        for mem in members {
            ctx.http()
                .remove_member_role(
                    ctx.data().primary_guild_id,
                    mem.id.parse().unwrap(),
                    role.id,
                    Some("Role removed due to XP role deletion."),
                )
                .await?;
        }
        Embed::success(&ctx)
            .description(format!("Removed {} as an XP role.", role))
            .send(&ctx)
            .await
    } else {
        Embed::error(&ctx)
            .description(format!("{} is not an XP role.", role))
            .send(&ctx)
            .await
    }
}

#[poise::command(slash_command, rename = "check-xp-role")]
pub async fn check_xp_role(ctx: Ctx<'_>, role: Role) -> Result<(), crate::Error> {
    if !admin(&ctx).await? {
        return not_admin(&ctx).await;
    }
    let xp_role = entity::xp_role::Entity::find_by_id(role.id.to_string())
        .one(&ctx.data().db)
        .await?;
    if let Some(xp_role) = xp_role {
        Embed::info(&ctx)
            .description(format!(
                "{} is an XP role at level {}.",
                role, xp_role.level
            ))
            .send(&ctx)
            .await
    } else {
        Embed::info(&ctx)
            .description(format!("{} is not an XP role.", role))
            .send(&ctx)
            .await
    }
}

#[poise::command(slash_command, rename = "list-xp-roles")]
pub async fn list_xp_roles(ctx: Ctx<'_>) -> Result<(), crate::Error> {
    let xp_roles = entity::xp_role::Entity::find()
        .order_by_asc(entity::xp_role::Column::Level)
        .all(&ctx.data().db)
        .await?
        .into_iter()
        .map(|xp_role| {
            let role = Mention::from(xp_role.id.parse::<RoleId>().unwrap());
            (role, xp_role.level)
        })
        .collect::<Vec<_>>();
    if xp_roles.is_empty() {
        return Embed::info(&ctx)
            .description("No XP roles have been set.")
            .send(&ctx)
            .await;
    }
    Embed::info(&ctx)
        .description(
            xp_roles
                .iter()
                .map(|(role, level)| format!("{} - Level {}", role, level))
                .collect::<Vec<_>>()
                .join("\n"),
        )
        .send(&ctx)
        .await
}

pub fn commands() -> Vec<poise::Command<Data, crate::Error>> {
    vec![
        add_xp_role(),
        remove_xp_role(),
        check_xp_role(),
        list_xp_roles(),
    ]
}

use poise::serenity_prelude::{Mention, Role, RoleId};
use sea_orm::{prelude::*, Set};

use crate::{
    utils::{
        embed::{not_admin, Embed},
        perms::admin,
    },
    Ctx,
    Data,
};

#[poise::command(slash_command, rename = "add-auto-role")]
pub async fn add_auto_role(
    ctx: Ctx<'_>,
    role: Role,
    group: Option<String>,
) -> Result<(), crate::Error> {
    if !admin(&ctx).await? {
        return not_admin(&ctx).await;
    }
    let auto_role = entity::auto_role::Entity::find_by_id(role.id.to_string())
        .one(&ctx.data().db)
        .await?;
    if auto_role.is_some() {
        Embed::error(&ctx)
            .description(format!("{} is already an auto role.", role))
            .send(&ctx)
            .await
    } else {
        if let Some(ref group) = group {
            let auto_role_group = entity::auto_role_group::Entity::find_by_id(group.clone())
                .one(&ctx.data().db)
                .await?;
            if auto_role_group.is_none() {
                return Embed::error(&ctx)
                    .description(format!("Auto role group {} does not exist.", group))
                    .send(&ctx)
                    .await;
            }
        }
        let auto_role = entity::auto_role::ActiveModel {
            id:    Set(role.id.to_string()),
            group: Set(group.clone()),
        };
        entity::auto_role::Entity::insert(auto_role)
            .exec(&ctx.data().db)
            .await?;
        if let Some(group) = group {
            Embed::success(&ctx)
                .description(format!(
                    "Added {} as an auto role in group {}.",
                    role, group
                ))
                .send(&ctx)
                .await
        } else {
            Embed::success(&ctx)
                .description(format!("Added {} as an auto role.", role))
                .send(&ctx)
                .await
        }
    }
}

#[poise::command(slash_command, rename = "remove-auto-role")]
pub async fn remove_auto_role(ctx: Ctx<'_>, role: Role) -> Result<(), crate::Error> {
    if !admin(&ctx).await? {
        return not_admin(&ctx).await;
    }
    let auto_role = entity::auto_role::Entity::find_by_id(role.id.to_string())
        .one(&ctx.data().db)
        .await?;
    if let Some(auto_role) = auto_role {
        auto_role.delete(&ctx.data().db).await?;
        Embed::success(&ctx)
            .description(format!("Removed {} as an auto role.", role))
            .send(&ctx)
            .await
    } else {
        Embed::error(&ctx)
            .description(format!("{} is not an auto role.", role))
            .send(&ctx)
            .await
    }
}

#[poise::command(slash_command, rename = "roles")]
pub async fn auto_roles(ctx: Ctx<'_>) -> Result<(), crate::Error> {
    let auto_roles = entity::auto_role::Entity::find()
        .all(&ctx.data().db)
        .await?;
    if auto_roles.is_empty() {
        return Embed::error(&ctx)
            .description("No auto roles.")
            .send(&ctx)
            .await;
    }
    let mut groups = std::collections::HashMap::new();
    for auto_role in auto_roles {
        let group = auto_role
            .group
            .clone()
            .unwrap_or_else(|| "None".to_string());
        groups
            .entry(group)
            .or_insert_with(Vec::new)
            .push(Mention::from(auto_role.id.parse::<RoleId>().unwrap()));
    }
    Embed::info(&ctx)
        .description(groups.iter().fold(String::new(), |mut s, (group, roles)| {
            s.push_str(&format!("**{}**\n", group));
            s.push_str(&roles.iter().fold(String::new(), |mut s, role| {
                s.push_str(&format!("{}\n", role));
                s
            }));
            s
        }))
        .send(&ctx)
        .await
}

#[poise::command(slash_command, rename = "role")]
pub async fn auto_role(ctx: Ctx<'_>, role: Role) -> Result<(), crate::Error> {
    let auto_role = entity::auto_role::Entity::find_by_id(role.id.to_string())
        .one(&ctx.data().db)
        .await?;
    if let Some(auto_role) = auto_role {
        ctx.defer_ephemeral().await?;
        if let Some(group) = auto_role.group {
            let group_roles = entity::auto_role::Entity::find()
                .filter(entity::auto_role::Column::Group.eq(group))
                .all(&ctx.data().db)
                .await?;
            for role in group_roles {
                ctx.http()
                    .remove_member_role(
                        ctx.data().primary_guild_id,
                        ctx.author().id,
                        role.id.parse().unwrap(),
                        Some("Role removed due to auto role group change."),
                    )
                    .await?;
            }
        }
        ctx.http()
            .add_member_role(
                ctx.data().primary_guild_id,
                ctx.author().id,
                role.id,
                Some("Auto role."),
            )
            .await?;
        Embed::success(&ctx)
            .description(format!("Added {}!", role))
            .send(&ctx)
            .await
    } else {
        Embed::error(&ctx)
            .description(format!("{} is not an auto role.", role))
            .send(&ctx)
            .await
    }
}

#[poise::command(slash_command, rename = "remove-role")]
pub async fn remove_role(ctx: Ctx<'_>, role: Role) -> Result<(), crate::Error> {
    let auto_role = entity::auto_role::Entity::find_by_id(role.id.to_string())
        .one(&ctx.data().db)
        .await?;
    if auto_role.is_some() {
        ctx.defer_ephemeral().await?;
        ctx.http()
            .remove_member_role(
                ctx.data().primary_guild_id,
                ctx.author().id,
                role.id,
                Some("Role removed due to auto role."),
            )
            .await?;
        Embed::success(&ctx)
            .description(format!("Removed {}!", role))
            .send(&ctx)
            .await
    } else {
        Embed::error(&ctx)
            .description(format!("{} is not an auto role.", role))
            .send(&ctx)
            .await
    }
}

pub fn commands() -> Vec<poise::Command<Data, crate::Error>> {
    vec![
        add_auto_role(),
        remove_auto_role(),
        auto_roles(),
        auto_role(),
        remove_role(),
    ]
}

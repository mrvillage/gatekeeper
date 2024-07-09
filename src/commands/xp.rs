use poise::serenity_prelude::{model::mention, User, UserId};
use sea_orm::{prelude::*, IntoActiveModel, QueryOrder, QuerySelect, Set};

use crate::{
    utils::{
        db::get_member,
        embed::{not_admin, Embed},
        num::Ths,
        perms::admin,
        xp::{level_down, level_up, xp_at_level},
    },
    Ctx,
    Data,
};

#[poise::command(slash_command)]
pub async fn xp(ctx: Ctx<'_>, user: Option<User>) -> Result<(), crate::Error> {
    let user = user.as_ref().unwrap_or_else(|| ctx.author());
    let member = get_member(&ctx.data().db, user.id).await?;
    let rank = entity::member::Entity::find()
        .filter(entity::member::Column::Xp.gt(member.xp))
        .count(&ctx.data().db)
        .await?
        + 1;
    let xp_in_level = member.xp - xp_at_level(member.level);
    let xp_to_next_level = xp_at_level(member.level + 1);
    Embed::info(&ctx)
        .author(user)
        .description(format!("Leaderboard rank: #{}", rank.ths()))
        .field("XP", member.xp.ths(), true)
        .field("Level", member.level.ths(), true)
        .field(
            "Progress",
            format!(
                "{:.2}% ({}/{})",
                (xp_in_level as f64) / (xp_to_next_level as f64),
                xp_in_level.ths(),
                xp_to_next_level.ths(),
            ),
            true,
        )
        .send(&ctx)
        .await
}

#[poise::command(
    slash_command,
    rename = "add-xp",
    default_member_permissions = "ADMINISTRATOR"
)]
pub async fn add_xp(ctx: Ctx<'_>, user: User, #[min = 0] xp: i32) -> Result<(), crate::Error> {
    if !admin(&ctx).await? {
        return not_admin(&ctx).await;
    }
    let mut member = get_member(&ctx.data().db, user.id)
        .await?
        .into_active_model();
    member.xp = Set(member.xp.unwrap() + xp);
    let leveled_up = level_up(ctx.http(), ctx.data(), &mut member).await?;
    member.save(&ctx.data().db).await?;
    Embed::success(&ctx)
        .description(format!(
            "Added {} XP to {}.{}",
            xp.ths(),
            user,
            if leveled_up { " They leveled up!" } else { "" }
        ))
        .send(&ctx)
        .await
}

#[poise::command(
    slash_command,
    rename = "remove-xp",
    default_member_permissions = "ADMINISTRATOR"
)]
pub async fn remove_xp(ctx: Ctx<'_>, user: User, #[min = 0] xp: i32) -> Result<(), crate::Error> {
    if !admin(&ctx).await? {
        return not_admin(&ctx).await;
    }
    let mut member = get_member(&ctx.data().db, user.id)
        .await?
        .into_active_model();
    if member.xp.clone().unwrap() < xp {
        return Embed::error(&ctx)
            .description("User doesn't have enough XP.")
            .send(&ctx)
            .await;
    }
    member.xp = Set(member.xp.unwrap() - xp);
    let leveled_down = level_down(ctx.http(), ctx.data(), &mut member).await?;
    member.save(&ctx.data().db).await?;
    Embed::success(&ctx)
        .description(format!(
            "Removed {} XP from {}.{}",
            xp.ths(),
            user,
            if leveled_down {
                " They leveled down!"
            } else {
                ""
            }
        ))
        .send(&ctx)
        .await
}

#[poise::command(
    slash_command,
    rename = "add-level",
    default_member_permissions = "ADMINISTRATOR"
)]
pub async fn add_level(
    ctx: Ctx<'_>,
    user: User,
    #[min = 0] level: i32,
) -> Result<(), crate::Error> {
    if !admin(&ctx).await? {
        return not_admin(&ctx).await;
    }
    let mut member = get_member(&ctx.data().db, user.id)
        .await?
        .into_active_model();
    member.level = Set(member.level.unwrap() + level);
    if level > 0 {
        member.xp = Set(xp_at_level(member.level.clone().unwrap()));
    }
    member.save(&ctx.data().db).await?;
    Embed::success(&ctx)
        .description(format!(
            "Added {} level{} to {}.",
            level.ths(),
            if level != 1 { "s" } else { "" },
            user
        ))
        .send(&ctx)
        .await
}

#[poise::command(
    slash_command,
    rename = "remove-level",
    default_member_permissions = "ADMINISTRATOR"
)]
pub async fn remove_level(
    ctx: Ctx<'_>,
    user: User,
    #[min = 0] level: i32,
) -> Result<(), crate::Error> {
    if !admin(&ctx).await? {
        return not_admin(&ctx).await;
    }
    let mut member = get_member(&ctx.data().db, user.id)
        .await?
        .into_active_model();
    if member.level.clone().unwrap() < level {
        return Embed::error(&ctx)
            .description("User doesn't have enough levels.")
            .send(&ctx)
            .await;
    }
    member.level = Set(member.level.unwrap() - level);
    if level > 0 {
        member.xp = Set(xp_at_level(member.level.clone().unwrap()));
    }
    member.save(&ctx.data().db).await?;
    Embed::success(&ctx)
        .description(format!(
            "Removed {} level{} from {}.",
            level.ths(),
            if level != 1 { "s" } else { "" },
            user
        ))
        .send(&ctx)
        .await
}

#[poise::command(slash_command)]
pub async fn leaderboard(ctx: Ctx<'_>, #[min = 1] page: Option<u64>) -> Result<(), crate::Error> {
    let member = get_member(&ctx.data().db, ctx.author().id).await?;
    let rank = entity::member::Entity::find()
        .filter(entity::member::Column::Xp.gt(member.xp))
        .count(&ctx.data().db)
        .await?
        + 1;
    let members = entity::member::Entity::find()
        .order_by_desc(entity::member::Column::Xp)
        .limit(10)
        .offset((page.unwrap_or(1) - 1) * 10)
        .all(&ctx.data().db)
        .await?;
    if members.is_empty() {
        return Embed::error(&ctx)
            .description("No members found on this page.")
            .send(&ctx)
            .await;
    }
    let lb = members
        .iter()
        .enumerate()
        .map(|(i, member)| {
            format!(
                "**{}.** {} - {} (Level {})",
                (i + 1).ths(),
                mention::Mention::from(member.id.parse::<UserId>().unwrap()),
                member.xp.ths(),
                member.level.ths()
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    Embed::info(&ctx)
        .description(format!("Your leaderboard rank: #{}\n\n{}", rank, lb))
        .send(&ctx)
        .await
}

pub fn commands() -> Vec<poise::Command<Data, crate::Error>> {
    vec![
        xp(),
        add_xp(),
        remove_xp(),
        add_level(),
        remove_level(),
        leaderboard(),
    ]
}

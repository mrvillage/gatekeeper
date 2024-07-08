use sea_orm::{prelude::*, EntityTrait, Set};

use crate::{
    utils::{
        embed::{not_admin, Embed},
        perms::admin,
    },
    Ctx,
    Data,
};

#[poise::command(slash_command, rename = "add-auto-role-group")]
pub async fn add_auto_role_group(ctx: Ctx<'_>, name: String) -> Result<(), crate::Error> {
    if !admin(&ctx).await? {
        return not_admin(&ctx).await;
    }
    if entity::auto_role_group::Entity::find_by_id(name.clone())
        .one(&ctx.data().db)
        .await?
        .is_some()
    {
        Embed::error(&ctx)
            .description(format!("Auto role group {} already exists.", name))
            .send(&ctx)
            .await
    } else {
        let auto_role_group = entity::auto_role_group::ActiveModel {
            name: Set(name.clone()),
        };
        entity::auto_role_group::Entity::insert(auto_role_group)
            .exec(&ctx.data().db)
            .await?;
        Embed::success(&ctx)
            .description(format!("Added auto role group {}.", name))
            .send(&ctx)
            .await
    }
}

#[poise::command(slash_command, rename = "remove-auto-role-group")]
pub async fn remove_auto_role_group(ctx: Ctx<'_>, name: String) -> Result<(), crate::Error> {
    if !admin(&ctx).await? {
        return not_admin(&ctx).await;
    }
    let auto_role_group = entity::auto_role_group::Entity::find_by_id(name.clone())
        .one(&ctx.data().db)
        .await?;
    if let Some(auto_role_group) = auto_role_group {
        if entity::auto_role::Entity::find()
            .filter(entity::auto_role::Column::Group.eq(auto_role_group.name.clone()))
            .one(&ctx.data().db)
            .await?
            .is_some()
        {
            return Embed::error(&ctx)
                .description(format!("Auto role group {} has auto roles.", name))
                .send(&ctx)
                .await;
        }
        auto_role_group.delete(&ctx.data().db).await?;
        Embed::success(&ctx)
            .description(format!("Removed auto role group {}.", name))
            .send(&ctx)
            .await
    } else {
        Embed::error(&ctx)
            .description(format!("Auto role group {} doesn't exist.", name))
            .send(&ctx)
            .await
    }
}

pub fn commands() -> Vec<poise::Command<Data, crate::Error>> {
    vec![add_auto_role_group(), remove_auto_role_group()]
}

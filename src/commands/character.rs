use poise::serenity_prelude::{
    ChannelId,
    ChannelType,
    CreateMessage,
    EditThread,
    ForumTagId,
    Mention,
    RoleId,
};

use crate::{
    utils::{
        embed::{not_admin, Embed},
        perms::admin,
    },
    Ctx,
    Data,
};

const CHARACTERS_CHANNEL: ChannelId = ChannelId::new(1259348869943394304);
const APPROVED_TAG: ForumTagId = ForumTagId::new(1259350583417110549);
const APPROVED_CHARACTER_ROLE: RoleId = RoleId::new(661973550894415903);

#[poise::command(slash_command)]
pub async fn approve(ctx: Ctx<'_>) -> Result<(), crate::Error> {
    if !admin(&ctx).await? {
        return not_admin(&ctx).await;
    }
    ctx.defer_ephemeral().await?;
    let channel = ctx.guild_channel().await;
    match channel {
        None => {
            Embed::error(&ctx)
                .description("This command must be used in a thread.")
                .send(&ctx)
                .await
        },
        Some(mut thread) => {
            match thread.kind {
                ChannelType::PublicThread | ChannelType::PrivateThread => {
                    let parent = thread.parent_id.unwrap();
                    if parent != CHARACTERS_CHANNEL {
                        return Embed::error(&ctx)
                            .description(format!(
                                "This command must be used in {}.",
                                Mention::from(CHARACTERS_CHANNEL)
                            ))
                            .send(&ctx)
                            .await;
                    }
                    let mut tags = thread.applied_tags.clone();
                    if tags.contains(&APPROVED_TAG) {
                        return Embed::error(&ctx)
                            .description("This character is already approved.")
                            .send(&ctx)
                            .await;
                    }
                    tags.push(APPROVED_TAG);
                    thread
                        .edit_thread(
                            ctx.http(),
                            EditThread::new().applied_tags(tags).locked(true),
                        )
                        .await?;
                    ctx.http()
                        .add_member_role(
                            ctx.data().primary_guild_id,
                            thread.owner_id.unwrap(),
                            APPROVED_CHARACTER_ROLE,
                            Some("Character approved."),
                        )
                        .await?;
                    thread
                        .send_message(
                            ctx.http(),
                            CreateMessage::new().content(format!(
                                "Congratulations, {}! Your character has been approved. You can \
                                 select your faction by using the `/role` command.",
                                Mention::from(thread.owner_id.unwrap())
                            )),
                        )
                        .await?;
                    Embed::success(&ctx)
                        .description(format!("Character approved by {}.", ctx.author()))
                        .send(&ctx)
                        .await
                },
                _ => {
                    Embed::error(&ctx)
                        .description("This command must be used in a thread.")
                        .send(&ctx)
                        .await
                },
            }
        },
    }
}

pub fn commands() -> Vec<poise::Command<Data, crate::Error>> {
    vec![approve()]
}

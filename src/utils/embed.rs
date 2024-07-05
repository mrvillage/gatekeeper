#![allow(unused)]

use poise::{
    serenity_prelude::{
        Colour,
        CreateEmbed,
        CreateEmbedAuthor,
        CreateEmbedFooter,
        Timestamp,
        User,
    },
    CreateReply,
};

use crate::Ctx;

pub enum EmbedStyle {
    Info,
    Success,
    Warning,
    Error,
}

impl EmbedStyle {
    pub fn colour(&self) -> Colour {
        match self {
            Self::Info => Colour::BLUE,
            Self::Success => Colour::DARK_GREEN,
            Self::Warning => Colour::ORANGE,
            Self::Error => Colour::RED,
        }
    }
}

pub struct Embed {
    embed: CreateEmbed,
}

impl Embed {
    #[inline]
    pub fn new(ctx: &Ctx<'_>, style: EmbedStyle) -> Self {
        Self {
            embed: CreateEmbed::new(),
        }.colour(style.colour()).footer("Star Wars Roleplay", Some("https://cdn.discordapp.com/icons/504665700024057886/a_eac97a46a66b93b25e36723221f297c7.webp?size=160")).timestamp(Timestamp::now()).author(ctx.author())
    }

    #[inline]
    pub fn info(ctx: &Ctx<'_>) -> Self {
        Self::new(ctx, EmbedStyle::Info)
    }

    #[inline]
    pub fn success(ctx: &Ctx<'_>) -> Self {
        Self::new(ctx, EmbedStyle::Success)
    }

    #[inline]
    pub fn warning(ctx: &Ctx<'_>) -> Self {
        Self::new(ctx, EmbedStyle::Warning)
    }

    #[inline]
    pub fn error(ctx: &Ctx<'_>) -> Self {
        Self::new(ctx, EmbedStyle::Error)
    }

    #[inline]
    pub fn author(mut self, u: &User) -> Self {
        self.embed = self.embed.author(
            CreateEmbedAuthor::new(u.name.clone()).icon_url(u.avatar_url().unwrap_or_default()),
        );
        self
    }

    #[inline]
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.embed = self.embed.title(title);
        self
    }

    #[inline]
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.embed = self.embed.description(description);
        self
    }

    #[inline]
    pub fn field(
        mut self,
        name: impl Into<String>,
        value: impl Into<String>,
        inline: bool,
    ) -> Self {
        self.embed = self.embed.field(name, value, inline);
        self
    }

    #[inline]
    pub fn image(mut self, url: impl Into<String>) -> Self {
        self.embed = self.embed.image(url);
        self
    }

    #[inline]
    pub fn thumbnail(mut self, url: impl Into<String>) -> Self {
        self.embed = self.embed.thumbnail(url);
        self
    }

    #[inline]
    pub fn footer(mut self, text: impl Into<String>, icon_url: Option<impl Into<String>>) -> Self {
        let mut footer = CreateEmbedFooter::new(text);
        if let Some(icon_url) = icon_url {
            footer = footer.icon_url(icon_url);
        }
        self.embed = self.embed.footer(footer);
        self
    }

    #[inline]
    pub fn timestamp(mut self, timestamp: impl Into<Timestamp>) -> Self {
        self.embed = self.embed.timestamp(timestamp);
        self
    }

    #[inline]
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.embed = self.embed.url(url);
        self
    }

    #[inline]
    pub fn colour(mut self, colour: Colour) -> Self {
        self.embed = self.embed.colour(colour);
        self
    }

    #[inline]
    pub fn build(self) -> CreateEmbed {
        self.embed
    }

    #[inline]
    pub async fn send(self, ctx: &Ctx<'_>) -> Result<(), crate::Error> {
        ctx.send(CreateReply::default().embed(self.build()).ephemeral(true))
            .await?;
        Ok(())
    }

    #[inline]
    pub async fn send_pub(self, ctx: &Ctx<'_>) -> Result<(), crate::Error> {
        ctx.send(CreateReply::default().embed(self.build()).ephemeral(false))
            .await?;
        Ok(())
    }
}

#[inline]
pub async fn not_admin(ctx: &Ctx<'_>) -> Result<(), crate::Error> {
    Embed::error(ctx)
        .description("You are not an admin.")
        .send(ctx)
        .await
}

#[inline]
pub async fn not_owner(ctx: &Ctx<'_>) -> Result<(), crate::Error> {
    Embed::error(ctx)
        .description("You are not the owner of this bot.")
        .send(ctx)
        .await
}

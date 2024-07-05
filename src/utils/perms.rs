use super::db::get_member;
use crate::Ctx;

#[inline]
pub fn is_owner(id: &str) -> bool {
    id == "1136467589879582780" || id == "258298021266063360"
}

#[inline]
pub fn is_admin(member: &entity::member::Model) -> bool {
    member.permissions > 0 || is_owner(&member.id)
}

#[inline]
pub async fn admin(ctx: &Ctx<'_>) -> Result<bool, crate::Error> {
    Ok(is_admin(
        &get_member(&ctx.data().db, &ctx.author().id.to_string()).await?,
    ))
}

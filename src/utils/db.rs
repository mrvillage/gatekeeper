use sea_orm::{DatabaseConnection, EntityTrait};

#[inline]
pub async fn get_member(
    db: &DatabaseConnection,
    id: impl ToString,
) -> Result<entity::member::Model, sea_orm::DbErr> {
    let id = id.to_string();
    let member = entity::member::Entity::find_by_id(&id).one(db).await?;
    match member {
        Some(member) => Ok(member),
        None => {
            let model = entity::member::ActiveModel {
                id: sea_orm::ActiveValue::Set(id.clone()),
                ..Default::default()
            };
            entity::member::Entity::insert(model).exec(db).await?;
            entity::member::Entity::find_by_id(id)
                .one(db)
                .await
                .map(|m| m.unwrap())
        },
    }
}

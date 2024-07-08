use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
#[allow(clippy::inconsistent_digit_grouping)]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Member::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Member::Id).text().not_null().primary_key())
                    .col(
                        ColumnDef::new(Member::Balance)
                            .big_integer()
                            .not_null()
                            .default(1_000_000_000_00_i64),
                    )
                    .col(ColumnDef::new(Member::Xp).integer().not_null().default(0))
                    .col(
                        ColumnDef::new(Member::Level)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(Member::Permissions)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(XpChannel::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(XpChannel::Id)
                            .text()
                            .not_null()
                            .primary_key(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(XpRole::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(XpRole::Id).text().not_null().primary_key())
                    .col(ColumnDef::new(XpRole::Level).integer().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(AutoRole::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(AutoRole::Id).text().not_null().primary_key())
                    .col(
                        ColumnDef::new(AutoRole::Group)
                            .text()
                            .default(None::<String>),
                    )
                    .foreign_key(
                        &mut ForeignKey::create()
                            .name("fk_auto_role_group")
                            .from(AutoRole::Table, AutoRole::Group)
                            .to(AutoRoleGroup::Table, AutoRoleGroup::Name)
                            .on_delete(ForeignKeyAction::Cascade)
                            .to_owned(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(AutoRoleGroup::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(AutoRoleGroup::Name)
                            .text()
                            .not_null()
                            .primary_key(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Member::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(XpChannel::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(XpRole::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(AutoRole::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(AutoRoleGroup::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Member {
    Table,
    Id,
    Balance,
    Xp,
    Level,
    Permissions,
}

#[derive(DeriveIden)]
enum XpChannel {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum XpRole {
    Table,
    Id,
    Level,
}

#[derive(DeriveIden)]
enum AutoRole {
    Table,
    Id,
    Group,
}

#[derive(DeriveIden)]
enum AutoRoleGroup {
    Table,
    Name,
}

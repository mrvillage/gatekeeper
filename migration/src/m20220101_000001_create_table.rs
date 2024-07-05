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
                            .default(100_000_000_00_i64),
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
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Member::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(XpChannel::Table).to_owned())
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

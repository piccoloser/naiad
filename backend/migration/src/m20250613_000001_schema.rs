use super::iden::*;
use sea_orm::Statement;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Sessions::Table)
                    .if_not_exists()
                    .col(string_uniq(Sessions::Token).primary_key())
                    .col(timestamp(Sessions::CreatedAt).default(Expr::current_timestamp()))
                    .col(timestamp_null(Sessions::RemovedAt))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(pk_auto(Users::Id))
                    .col(string_uniq(Users::Email).unique_key())
                    .col(string_null(Users::Username).unique_key())
                    .col(string(Users::PwHash))
                    .col(timestamp(Users::CreatedAt).default(Expr::current_timestamp()))
                    .col(timestamp(Users::UpdatedAt).default(Expr::current_timestamp()))
                    .col(timestamp_null(Users::RemovedAt))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Roles::Table)
                    .if_not_exists()
                    .col(pk_auto(Roles::Id))
                    .col(string_uniq(Roles::Title))
                    .to_owned(),
            )
            .await?;

        let db = manager.get_connection();
        db.execute(Statement::from_string(
            db.get_database_backend(),
            "INSERT INTO roles (title) VALUES ('admin'), ('user')",
        ))
        .await?;

        manager
            .create_table(
                Table::create()
                    .table(UserRoles::Table)
                    .if_not_exists()
                    .col(pk_auto(UserRoles::Id))
                    .col(integer(UserRoles::UserId))
                    .col(integer(UserRoles::RoleId))
                    .col(timestamp(UserRoles::CreatedAt).default(Expr::current_timestamp()))
                    .col(timestamp(UserRoles::UpdatedAt).default(Expr::current_timestamp()))
                    .col(timestamp_null(UserRoles::RemovedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_userroles_users")
                            .from(UserRoles::Table, UserRoles::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_userroles_roles")
                            .from(UserRoles::Table, UserRoles::RoleId)
                            .to(Roles::Table, Roles::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .index(
                        Index::create()
                            .name("uq_userroles_userid_roleid")
                            .table(UserRoles::Table)
                            .col(UserRoles::UserId)
                            .col(UserRoles::RoleId)
                            .unique(),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(UserRoles::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Roles::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Sessions::Table).to_owned())
            .await?;

        Ok(())
    }
}

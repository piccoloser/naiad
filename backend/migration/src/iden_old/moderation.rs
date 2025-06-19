use sea_orm_migration::prelude::*;

#[derive(Iden)]
pub enum OrgSuspensions {
    Table,
    OrgId,
    ModId,
    CreatedAt,
    ExpiresAt,
    Reason,
}

#[derive(Iden)]
pub enum UserSuspensions {
    Table,
    UserId,
    ModId,
    CreatedAt,
    ExpiresAt,
    Reason,
}

#[derive(Iden)]
pub enum UserBlocks {
    Table,
    BlockerId,
    BlockedId,
    CreatedAt,
    RemovedAt,
}

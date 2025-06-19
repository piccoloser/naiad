use sea_orm_migration::prelude::*;

#[derive(Iden)]
pub enum ResourceType {
    Table,
    Id,
    Title,
}

#[derive(Iden)]
pub enum Resources {
    Table,
    Id,
    OrgId,
    OwnerId,
    TypeId,
    Title,
}

#[derive(Iden)]
pub enum ResourceAccess {
    Table,
    ResourceId,
    UserId,
    RoleId,
    CreatedAt,
    RemovedAt,
}

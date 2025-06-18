use sea_orm_migration::prelude::*;

#[derive(Iden)]
pub enum OrgAttributes {
    Table,
    OrgId,
    Key,
    Value,
    TypeId,
    CreatedAt,
    UpdatedAt,
    RemovedAt,
}

#[derive(Iden)]
pub enum ResourceAttributes {
    Table,
    ResourceId,
    Key,
    Value,
    TypeId,
    CreatedAt,
    UpdatedAt,
    RemovedAt,
}

#[derive(Iden)]
pub enum UserAttributes {
    Table,
    UserId,
    Key,
    Value,
    TypeId,
    CreatedAt,
    UpdatedAt,
    RemovedAt,
}

#[derive(Iden)]
pub enum ValueType {
    Table,
    Id,
    Title,
}

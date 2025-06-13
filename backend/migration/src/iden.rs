use sea_orm_migration::prelude::*;

#[derive(Iden)]
pub enum Sessions {
    Table,
    Token,
    CreatedAt,
    RemovedAt,
}

#[derive(Iden)]
pub enum Users {
    Table,
    Id,
    Username,
    Email,
    PwHash,
    CreatedAt,
    UpdatedAt,
    RemovedAt,
}

#[derive(Iden)]
pub enum UserRoles {
    Table,
    Id,
    UserId,
    RoleId,
    CreatedAt,
    UpdatedAt,
    RemovedAt,
}

#[derive(Iden)]
pub enum Roles {
    Table,
    Id,
    Title,
}
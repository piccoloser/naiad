use sea_orm_migration::prelude::*;

#[derive(Iden)]
pub enum AuditLogs {
    Table,
    Id,
    UserId,
    Action,
    Target,
    PolicyId,
    Result,
    Context,
    CreatedAt,
}

#[derive(Iden)]
pub enum Pronouns {
    Table,
    Id,
    Sub,
    Obj,
    Pos,
    Ref,
    Plural,
}

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
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
pub enum UserDetails {
    Table,
    UserId,
    Username,
    FirstName,
    LastName,
    Email,
    SysPronouns,
    Bio,
}

#[derive(Iden)]
pub enum UserPronouns {
    Table,
    UserId,
    PronounsId,
}
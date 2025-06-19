use sea_orm_migration::prelude::*;

#[derive(Iden)]
pub enum Orgs {
    Table,
    Id,
    CreatedAt,
    RemovedAt,
}

#[derive(Iden)]
pub enum OrgDetails {
    Table,
    OrgId,
    Title,
    Bio,
}

#[derive(Iden)]
pub enum OrgPolicies {
    Table,
    OrgId,
    PolicyId,
    CreatedAt,
    UpdatedAt,
    RemovedAt,
}

#[derive(Iden)]
pub enum OrgUsers {
    Table,
    OrgId,
    UserId,
    CreatedAt,
    RemovedAt,
}

use sea_orm_migration::prelude::*;



#[derive(Iden)]
pub enum ValueType {
    Table,
    Id,
    Title,
}

#[derive(Iden)]
pub enum UserAttributes {
    Table,
    UserId,
    Key,
    Value,
    CreatedAt,
    UpdatedAt,
    RemovedAt,
}

#[derive(Iden)]
pub enum Orgs {
    Table,
    Id,
    CreatedAt,
    RemovedAt,
    Suspended,
}

#[derive(Iden)]
pub enum OrgAttributes {
    Table,
    OrgId,
    Key,
    Value,
    CreatedAt,
    UpdatedAt,
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
pub enum UserOrgs {
    Table,
    OrgId,
    UserId,
    CreatedAt,
    RemovedAt,
}



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

#[derive(Iden)]
pub enum ResourceAttributes {
    Table,
    ResourceId,
    Key,
    Value,
    CreatedAt,
    UpdatedAt,
    RemovedAt,
}

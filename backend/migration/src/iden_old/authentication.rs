use sea_orm_migration::prelude::*;

#[derive(Iden)]
pub enum Actions {
    Table,
    Id,
    Title,
}

#[derive(Iden)]
pub enum DelegationScopes {
    Table,
    Id,
    Title,
}

#[derive(Iden)]
pub enum Delegations {
    Table,
    GrantorId,
    GranteeId,
    ScopeId,
    OrgId,
    Description,
    CreatedAt,
    ExpiresAt,
    RemovedAt,
}

#[derive(Iden)]
pub enum Permissions {
    Table,
    Id,
    Title,
}

#[derive(Iden)]
pub enum PolicyTemplates {
    Table,
    Id,
    Title,
    BaseCondition,
    Description,
}

#[derive(Iden)]
pub enum Policies {
    Table,
    Id,
    ResourceSelector,
    SubjectSelector,
    Action,
    Condition,
    Effect,
}

#[derive(Iden)]
pub enum Roles {
    Table,
    Id,
    Title,
}

#[derive(Iden)]
pub enum RolePermissions {
    Table,
    RoleId,
    PermissionId,
    CreatedAt,
    RemovedAt,
}

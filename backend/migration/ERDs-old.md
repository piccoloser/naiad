## Core
```mermaid
erDiagram
    users {
        int id
        datetime created_at
        datetime updated_at
    }

    user_details {
        int user_id
        string username
        string first_name
        string last_name
        string email
        int sys_pronouns
        string bio
    }

    sessions {
        string token
        datetime created_at
        datetime removed_at
    }

    user_pronouns {
        int user_id
        int pronouns_id
    }

    pronouns {
        int id
        string sub
        string obj
        string pos
        string ref
        bool plural
    }

    audit_logs {
        int id
        int user_id
        string action
        string target
        int policy_id
        string result
        string context
        datetime created_at
    }

    users ||--|| user_details : has
    users ||--|| sessions : gets
    users ||--o{ user_pronouns : has
    pronouns ||--o{ user_pronouns : defines
    users ||--o{ audit_logs : triggers
```

## Orgs
```mermaid
erDiagram
    orgs {
        int id
        datetime created_at
        datetime removed_at
    }

    org_details {
        int org_id
        string title
        string bio
    }

    org_users {
        int org_id
        int user_id
        datetime created_at
        datetime removed_at
    }

    org_policies {
        int org_id
        int policy_id
        datetime created_at
        datetime updated_at
        datetime removed_at
    }

    policies {
        int id
        string resource_selector
        string subject_selector
        string action
        string condition
        string effect
    }

    policy_templates {
        int id
        string title
        string base_condition
        string description
    }

    orgs ||--|| org_details : has
    orgs ||--o{ org_users : contains
    orgs ||--o{ org_policies : has
    org_users ||--o{ "core:users" : has
    policies ||--o{ org_policies : includes
    policies ||..|| policy_templates : extends
```
## Resources
```mermaid
erDiagram
    resource_type {
        int id
        string title
    }

    resources {
        int id
        int org_id
        int owner_id
        int type_id
        string title
    }

    resource_access {
        int resource_id
        int user_id
        int role_id
        datetime created_at
        datetime removed_at
    }

    resource_type ||--o{ resources : classifies
    orgs ||--o{ resources : owns
    users ||--o{ resources : creates
    resources ||--o{ resource_access : grants
    users ||--o{ resource_access : "is granted"
    roles ||--o{ resource_access : via
```
## Attributes
```mermaid
erDiagram
    user_attributes {
        int user_id
        string key
        string value
        int type_id
        datetime created_at
        datetime updated_at
        datetime removed_at
    }

    org_attributes {
        int org_id
        string key
        string value
        int type_id
        datetime created_at
        datetime updated_at
        datetime removed_at
    }

    resource_attributes {
        int resource_id
        string key
        string value
        int type_id
        datetime created_at
        datetime updated_at
        datetime removed_at
    }

    value_type {
        int id
        string title
    }

    value_type ||--o{ user_attributes : "has type"
    value_type ||--o{ org_attributes: "has type"
    value_type ||--o{ resource_attributes : "has type"
```
## Auth
```mermaid
erDiagram
    actions {
        int id
        string title
    }

    roles {
        int id
        string title
    }

    permissions {
        int id
        string title
    }

    role_permissions {
        int role_id
        int permission_id
        datetime created_at
        datetime removed_at
    }

    delegations {
        int grantor_id
        int grantee_id
        int scope_id
        int org_id
        string description
        datetime created_at
        datetime expires_at
        datetime removed_at
    }

    delegation_scopes {
        int id
        string title
    }

    roles ||--o{ role_permissions : has
    permissions ||--o{ role_permissions : "included in"
    users ||--o{ delegations : grants
    users ||--o{ delegations : "is granted"
    orgs ||--o{ delegations : scoped
    delegation_scopes ||--o{ delegations : limits
```
## Moderation
```mermaid
erDiagram
    org_suspensions {
        int org_id
        datetime created_at
        datetime expires_at
        string reason
    }

    user_suspensions {
        int user_id
        datetime created_at
        datetime expires_as
        string reason
    }

    user_blocks {
        int blocker_id
        int blocked_id
        datetime created_at
        datetime removed_at
    }

    orgs ||--o{ org_suspensions : suspended
    users ||--o{ user_suspensions : suspended
    users ||--o{ user_blocks : blocks
    users ||--o{ user_blocks : "blocked by"
```
<!--
## Mermaid Reference
**Diagram Type:** `erDiagram` (Entity Relationship Diagram)
| Symbol      | Relationship       |   | Syntax           | Usage             |
| ----------- | ------------------ | - | ---------------- | ----------------- |
| `\|\|`      | exactly one        |   | `%%`             | Comment           |
| `o{` / `}o` | zero or more       |   | `ITEM { ... }`   | Model Definition  |
| `\|{`       | one or more        |   | `TYPE KEY_NAME`  | Attribute Def.    |
| `--`        | generic connection |   | `A -- B : LABEL` | Relationship Def. |
-->
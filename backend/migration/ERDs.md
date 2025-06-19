# Schema Design
## 0. Important Context
1. Fields are `NOT NULL` unless declared otherwise.
    1. All `removed_at` fields are `NULL`able, always.
2. If `id` is present, it is the primary key unless otherwise stated.
3. Foreign keys are shown inline as `"FK: ..."` in field descriptions.
4. Tables with `key` and `label` fields account for localization.

### 0.1 Terms
- **group** - any entity which may contain other entities (e.g. `orgs`, `depts`)

## 1. Core Domain
### 1.1 Tables
```mermaid
erDiagram
    entity_types {
        int id
        string key "e.g. user, dept, org, resource"
        string label_key "FK: localized_strings(key)"
        datetime created_at
    }

    entities {
        int id
        int type_id "FK: entity_types(id)"
        int creator_id "FK: entities(id)"
        string name
        string label_key "NULL, FK: localized_strings(key)"
        datetime created_at
        datetime updated_at
        datetime removed_at
    }

    event_logs {
        int id
        int actor_id "FK: entities(id)"
        int scope_id "FK: entities(id)"
        int action_id "FK: actions(id)"
        json context "NULL, arbitrary key-value data"
        string source "e.g. manual, automation, external_api"
        datetime created_at
        datetime removed_at "NULL"
        string redaction_reason "NULL"
    }

    depts {
        int entity_id "FK: entities(id)"
        string name
    }

    orgs {
        int entity_id "FK: entities(id)"
        string name
    }

    users {
        int entity_id "FK: entities(id)"
        string email
    }

    entity_types ||--o{ entities : defines
    entities ||--|| depts : is
    entities ||--|| orgs : is
    entities ||--|| users : is
```

### 1.2 Constraints
1. `UNIQUE(type_id, name)` on `entities`
2. `UNIQUE(key)` on `entity_types`

### 1.3 Indexes
1. `(type_id, id)` on `entities`
2. `creator_id` on `entities`
3. `name` on `entities`
4. `(actor_id, created_at)` on `event_logs`
5. `(scope_id, created_at)` on `event_logs`
6. `action_id` on `event_logs`
7. `created_at` on `event_logs`

## 2. Structure Domain
### 2.1 Tables
```mermaid
erDiagram
    actions {
        int id
        string key
        string label_key "NULL, FK: localized_strings(key)"
    }

    memberships {
        int id
        int member_id "FK: entities(id)"
        int group_id "FK: entities(id)"
        int created_by "FK: entities(id)"
        datetime created_at
        datetime removed_at
    }

    membership_logs {
        int membership_id "FK: memberships(id)"
        int updated_by "FK: entities(id)"
        int action_id "FK: actions(id)"
        string reason "NULL"
        datetime created_at
    }

    actions ||--o{ membership_logs : describes
    membership_logs }o--|| memberships : tracks
```

### 2.2 Constraints
1. `UNIQUE(key)` on `actions`
2. `UNIQUE(member_id, group_id, removed_at)` on `memberships`

### 2.3 Indexes
1. `created_by` on `memberships`
2. `group_id` on `memberships`
3. `member_id` on `memberships`
4. `removed_at` on `memberships`
5. `(member_id, group_id, removed_at)` on `memberships`
6. `created_at` on `membership_logs`
7. `membership_id` on `membership_logs`

## 3. Locale Domain
### 3.1 Tables
```mermaid
erDiagram
    locales {
        int id
        string code "e.g. en-US, fr-FR, es-ES"
        string name "Display name, e.g. English (US)"
    }

    localized_strings {
        int id
        string locale_code "FK: locales(code)"
        string key "e.g. action.invite, entity_type.org"
        string value
        datetime created_at
    }

    localized_string_logs {
        int string_id "FK: localized_strings(id)"
        int updated_by "FK: entities(id)"
        int action_id "FK: actions(id)"
        string reason "NULL"
        datetime created_at
    }

    locales ||--o{ localized_strings : contains
```

### 3.2 Constraints
1. `UNIQUE(code)` on `locales`
2. `UNIQUE(key, locale_code)` on `localized_strings`

### 3.3 Indexes
1. `code` on `locales`
2. `(key, locale_code)` on `localized_strings`
3. `locale_code` on `localized_strings`
4. `key` on `localized_strings`
5. `string_id` on `localized_string_logs`

## 4. Auth Domain
### 4.1 Tables
```mermaid
erDiagram
    permissions {
        int id
        string key "e.g. edit.user"
        string label_key "FK: localized_strings(key)"
        string description_key "FK: localized_strings(key)"
        datetime created_at
    }

    permission_actions {
        int permission_id "FK: permissions(id)"
        int action_id "FK: actions(id)"
    }

    permission_assignments {
        int id
        int grantor_id "FK: entities(id)"
        int grantee_id "FK: entities(id)"
        int scope_id "NULL, FK: entities(id)"
        int permission_id "FK: permissions(id)"
        datetime created_at
        datetime expires_at "NULL"
        datetime removed_at "NULL"
    }

    actions ||--o{ permission_actions : maps
    entities ||--o{ permission_assignments : grants
    permissions ||--o{ permission_actions : includes
    permissions ||--o{ permission_assignments : assigns
```

### 4.2 Constraints
1. `UNIQUE(key)` on `permissions`
2. `UNIQUE(permission_id, action_id)` on `permission_actions`

### 4.3 Indexes
1. `key` on `permissions`
2. `(permission_id, action_id)` on `permission_actions`
3. `action_id` on `permission_actions`
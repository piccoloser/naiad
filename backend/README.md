# Naiad Backend

This repository contains the backend for **Naiad**, a modular, production-oriented starter kit built with Rust, Rocket, SeaORM, and Svelte. Naiad is currently under active development and is designed to serve as a foundation for fullstack applications requiring:

- Role-based authentication and authorization
- Optional LDAP integration for enterprise environments
- Configurable, restartable runtime behavior
- Seamless frontend-backend integration via Rocket and Svelte
- Extensible database management using SeaORM
    - Applications use SQLite by default

---

## Initial Project Setup (Default Configuration)
### 1. Database Setup
#### Create a SQLite Database
```bash
> touch backend/data.db
```
Ensure the file is placed somewhere the application has permission to access. By default, it will look for the file at `backend/data.db`.

#### Apply Migrations
Write your migration files in `backend/migrations/m[date]_[migration_name].rs`.

Register each new migration in `backend/migrations/lib.rs`.

**Important:** If you plan to use another database (e.g. MySQL or Postgres), be sure to enable their respective feature(s) in `backend/migration/Cargo.toml`. You can also choose a different async runtime here if needed.

Then run:
```bash
sea-orm-cli migrate up --database-url sqlite://backend/data.db
```
> **Note:** You can set a `DATABASE_URL` environment variable and reuse it for brevity. Also, as long as your database URL is set in `config.ini` and accessible to the application, you can store it elsewhere.

#### Generate Entity Files
```bash
sea-orm-cli generate entity -o src/entities --database-url sqlite://backend/data.db --with-serde both
```
This will create all SeaORM entity definitions in `src/entities`.
> **Note:** As of writing, some modifications may be needed in order for your entities to work with SQLite due to a [known issue](https://github.com/SeaQL/sea-orm/issues/2051) with SeaORM. Specifically, you may need to manually change the generated datatype of some nullable fields.

---
### 2. LDAP Setup
Create a `.env` file in the `backend` directory with the following keys:
```
ldap_bind_username=YOUR_SERVICE_ACCOUNT_USERNAME
ldap_bind_password=YOUR_SECURE_PASSWORD
```
These values must be valid or LDAP bindings will fail and user authentication will break.

---
## To Do
- [x] Lifetime control routes (`/restart`, `/shutdown`)
- [x] Admin-only frontend configuration editor page
- [ ] Include frontend build/setup steps
- [ ] Expand documentation to include other database types

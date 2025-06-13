# Naiad

**Naiad** is a fullstack Rust + Svelte starter kit for building internal tools, admin apps, and lightweight dashboards — designed to be modular, production-oriented, and highly extensible. It combines:

- A Rust backend powered by Rocket+SeaORM
- A Svelte frontend with Routify 3
- Role-based authentication with optional LDAP support
- Live config editing and server lifecycle control

Whether you're building an internal admin panel or a custom dashboard for your team, Naiad helps you get from zero to functional with real-world features and sane defaults.

---
## Repository Structure
```txt
/project_root/
├── backend/     # Rocket + SeaORM backend (Rust)
├── frontend/    # Routify + Svelte frontend (TypeScript)
└── README.md    # You are here!
````

---

## Features
* **Auth Provider System**: Hot-swappable between database and LDAP (configurable at runtime)
* **JWT Session Management**: Token-based authentication with session clearing support
* **Frontend Config Editor**: An admin-only page for editing `config.ini`, with mostly-seamless live updates
* **Dynamic Routing (Frontend)**: File-based routing via Routify 3
* **Full CRUD Scaffold**: Uses `#[generate_crud_routes()]` macro for entity management
* **Rocket-Served SPA**: Rocket serves static files and SPA fallback directly

---

## Technologies Used

| Layer    | Tech Stack                       |
| -------- | -------------------------------- |
| Frontend | Svelte + Routify                 |
| Backend  | Rust + Rocket                    |
| Database | SeaORM (SQLite by default)       |
| Auth     | JWT (with optional LDAP support) |
| Routing  | FileServer + SPA fallback        |

---

## Naming & Attribution

Naiads are mythological water spirits — fluid, adaptive, and powerful.

> **Note:** Naiad has no affiliation with Microsoft’s discontinued project of the same name.

---

## Roadmap Highlights

* [x] Role-based auth system with LDAP support
* [x] Runtime config + restart logic
* [x] SPA integration via Rocket
* [x] Dev panel + local state inspector
* [ ] Dockerfile & production build scaffolding
* [ ] CLI tooling (`naiad new`, `naiad generate`)
* [ ] Unit + integration test harness

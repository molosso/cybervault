# CyberVault

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust)
![Tauri](https://img.shields.io/badge/Tauri-24C8DB?style=for-the-badge&logo=tauri)
![Vue.js](https://img.shields.io/badge/Vue.js-4FC08D?style=for-the-badge&logo=vue.js)
![SQLite](https://img.shields.io/badge/SQLite-003B57?style=for-the-badge&logo=sqlite)

CyberVault is a gothic-themed local password manager built with Rust, Tauri 2, Vue 3, and SQLite.

Inspired by dark fantasy and Souls-like aesthetics, CyberVault provides a secure offline environment to store, manage, and generate passwords through an immersive desktop interface.

The project explores modern desktop application development by combining Rust performance with a reactive frontend experience.

---

# Preview

Add screenshots here:


docs/
├── dashboard.png
├── vault.png
└── generator.png

Example:


CyberVault
 ⚔ Forgotten Digital Grimoire ⚔

 ┌─────────────────────────────┐
 │ Discord                     │
 │ melissa                     │
 │ ************                │
 │                             │
 │        [ Reveal ]           │
 └─────────────────────────────┘

---

# Features

## Vault Management

- Create password entries
- Store website credentials
- Display saved accounts
- Delete vault entries
- Offline local storage

## Password Generator

- Generate strong passwords
- Random secure generation
- Support for:
  - Uppercase letters
  - Lowercase letters
  - Numbers
  - Special characters

## Gothic Interface

- Dark fantasy inspired UI
- Souls-like atmosphere
- Medieval color palette
- Gold and crimson accents
- Animated vault cards

## Local Storage

- SQLite database
- No cloud dependency
- Fully offline application

---

# Architecture

CyberVault uses a modern desktop architecture where Vue handles the interface and Rust manages the application logic.

```mermaid
graph TD

    A[Vue 3 Frontend] -->|Tauri API Invoke| B[Rust Backend]

    B --> C[Command Handlers]

    C --> D[SQLite Database]

    D --> E[(vault.db)]

    B --> F[Password Generator]

    F --> G[Secure Random Generation]

    C --> H[Vault Management]

    H --> I[Add Credentials]

    H --> J[Delete Credentials]

    H --> K[Retrieve Credentials]


    style A fill:#42b883,color:white
    style B fill:#dea584,color:black
    style D fill:#003b57,color:white
    style E fill:#111,color:white

Project Structure
CyberVault

├── src
│
│   ├── App.vue
│   ├── main.js
│   │
│   └── assets
│       └── style.css
│
├── src-tauri
│
│   ├── src
│   │
│   │   ├── main.rs
│   │   ├── lib.rs
│   │   ├── database.rs
│   │   └── models.rs
│   │
│   └── Cargo.toml
│
└── package.json

Technology Stack
Layer	Technology
Desktop Framework	Tauri 2
Frontend	Vue 3
Language	JavaScript
Backend	Rust
Database	SQLite
Database Driver	Rusqlite
Build Tool	Vite
Package Manager	npm


Installation
Requirements
Install:
- Node.js
- Rust
- Cargo
- Tauri prerequisites
Verify:
node --version

rustc --version

cargo --version

Clone Repository
git clone https://github.com/yourusername/CyberVault.git

cd CyberVault

Install Dependencies
npm install

Run Development Mode
npm run tauri dev

The application will start as a native desktop application.
Database
CyberVault automatically creates a local SQLite database:
vault.db

Database structure:
vault

id
name
login
secret

All data is stored locally.
Security Roadmap
Future security improvements:
- AES-256 encryption
- Master password authentication
- Argon2 password hashing
- Automatic vault locking
- Secure clipboard management
- Password strength analysis
Roadmap
Version 1.0
Completed:
- Desktop application
- Vue interface
- Rust backend
- SQLite storage
- Password generator
- Gothic UI
Version 2.0
Planned:
- Encrypted database
- Master password
- Search system
- Categories
- Password visibility control
Version 3.0
Planned:
- Import/export vault
- Backup system
- Cross-platform releases
- Advanced security features
Learning Goals
CyberVault was created to explore:
- Rust desktop development
- Tauri architecture
- Frontend and backend communication
- SQLite integration
- Secure application design
- Modern UI development

GitHub:
https://github.com/molosso


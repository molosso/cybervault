# CyberVault

CyberVault is a gothic-themed local password manager built with Rust, Tauri, Vue 3, and SQLite.

The application provides a secure desktop environment to store, manage, and generate passwords through a dark fantasy inspired interface.

The goal of this project is to combine modern desktop development with Rust performance and a strong user experience.

---

## Screenshots

Add application screenshots here.

Example:


docs/
├── dashboard.png
├── vault.png
└── generator.png

---

# Features

## Password Management

- Add new credentials
- Store website, username, and password information
- Display saved entries
- Delete vault entries
- Local offline storage

## Password Generator

- Generate strong random passwords
- Customizable password length
- Support for:
  - Uppercase letters
  - Lowercase letters
  - Numbers
  - Special characters

## Gothic User Interface

- Dark fantasy inspired design
- Medieval / Souls-like atmosphere
- Gold and crimson color palette
- Animated cards
- Immersive desktop experience

## Local Database

- SQLite based storage
- No cloud dependency
- Fully offline application

---

# Tech Stack

## Frontend

- Vue 3
- JavaScript
- CSS3
- Tauri API

## Backend

- Rust
- Tauri 2
- SQLite
- Rusqlite

## Development Tools

- Cargo
- npm
- Vite

---

# Architecture


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

---

# Installation

## Requirements

Make sure you have installed:

- Node.js
- Rust
- Cargo
- Tauri prerequisites

Check versions:

```bash
node --version
rustc --version
cargo --version

Clone the repository
git clone https://github.com/yourusername/CyberVault.git

cd CyberVault

Install frontend dependencies
npm install

Run development mode
npm run tauri dev

The application will open as a desktop window.
Database
CyberVault automatically creates a local SQLite database:
vault.db

The database contains stored credentials locally.
Example structure:
vault

id
name
login
secret

Security Roadmap
Future security improvements:
- AES-256 encryption
- Master password authentication
- Argon2 password hashing
- Secure clipboard management
- Automatic vault locking
- Password strength analysis
Roadmap
Version 1.0
Completed:
- Desktop application
- SQLite storage
- Password generator
- Gothic interface
Version 2.0
Planned:
- Encrypted vault
- Master password
- Search system
- Categories
- Password visibility toggle
Version 3.0
Planned:
- Import/export vault
- Backup system
- Cross-platform builds
- Advanced security features
Why this project?
CyberVault was created to explore:
- Rust desktop development
- Tauri architecture
- Frontend/backend communication
- Local data management
- Secure application design
This project demonstrates how Rust can be used to build modern and efficient desktop applications.


GitHub:
https://github.com/molosso


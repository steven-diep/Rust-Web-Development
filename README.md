# Rust Web Example
Author: Steven Diep

## About
This repository contains my homework assignment submissions for the 'Rust Web Development' course at Portland State University.

This project currently contains a REST API implemented using Axum with the basic CRUD operations needed to manage a Postgres database of questions and answers for persistant storage. There is also a very basic frontend for displaying questions.

## Requirements
- Docker Desktop
- Rust

## Installation
To run this project, follow the steps below.

1. Clone this repository:

```Bash
git clone https://github.com/steven-diep/Rust-Web-Development.git
```

2. Create a file named `password.txt` in the `db` directory with your database password (see `password.example.txt` for reference) and a file named `api-url.txt` in the `frontend` directory with the url (see `api-url.example.txt for reference)

3. To run the backend:
```Bash
docker compose up --build
```

4. To stop the backend:
```Bash
docker compose down
```

5. To run the frontend:
```Bash
cd frontend
rustup target install wasm32-unknown-unknown
cargo install trunk
trunk serve
```

# Rust REST API Example

This project is a simple REST API built with Actix-web. It provides endpoints to check health, list items, and add new items.

## Endpoints
- `GET /health` - Returns API status
- `GET /items` - Returns a list of items
- `POST /items` - Adds a new item

## Getting Started

1. Install Rust: https://rustup.rs/
2. Install dependencies and run the server:
   ```sh
   cargo run
   ```

The server will start on http://127.0.0.1:8080

## Example Requests

- Health check:
  ```sh
  curl http://127.0.0.1:8080/health
  ```
- List items:
  ```sh
  curl http://127.0.0.1:8080/items
  ```
- Add item:
  ```sh
  curl -X POST -H "Content-Type: application/json" -d '{"name": "Item1"}' http://127.0.0.1:8080/items
  ```

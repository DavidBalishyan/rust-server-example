# Rust + React Fullstack Project

This repository contains a fullstack web application with a React frontend (built using [Rsbuild](https://rsbuild.rs)) and a Rust backend server (using [Axum](https://docs.rs/axum/latest/axum/)).

## Project Structure

- **frontend/** — React app, bundled with Rsbuild.
- **backend/** — Rust server, serves static files and provides API endpoints.

## Getting Started

### Prerequisites

- [Node.js](https://nodejs.org/) & [pnpm](https://pnpm.io/)
- [Rust](https://rust-lang.org/tools/install)

### Install dependencies

```bash
pnpm install --dir frontend
```

### Build and Run

Use the provided script to build both frontend and backend:

```bash
chmod +x ./build.sh
./build.sh
```

This will:
- Build the React frontend and output to `backend/dist`
- Build the Rust backend

### Development

- **Frontend:**  
  Start the dev server (hot reload, port 3000):

  ```bash
  cd frontend
  pnpm dev
  ```

- **Backend:**  
  Run the Rust server (serves API and static files):

  ```bash
  cd backend
  cargo run
  ```

## Usage

- Visit [http://localhost:3000](http://localhost:3000) to view the app.
- API endpoint example: [http://localhost:3000/api/data](http://localhost:3000/api/data)

## Scripts

- `pnpm build` — Build frontend for production
- `pnpm dev` — Start frontend dev server
- `pnpm preview` — Preview production build
- `pnpm lint` — Lint frontend code
- `pnpm format` — Format frontend code

## Learn More

- [Rsbuild documentation](https://rsbuild.rs)
- [Axum documentation](https://docs.rs/axum/latest/axum/)
- [Rust](https://rust-lang.org/)
- [React](https://react.dev/)
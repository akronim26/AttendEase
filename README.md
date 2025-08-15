# AttendEase

AttendEase is a full-stack web application for streamlined attendance management. It features a high-performance Rust backend and a modern, responsive React frontend styled with Tailwind CSS.

---

## Table of Contents

- [Features](#features)
- [Project Structure](#project-structure)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Backend Setup](#backend-setup)
  - [Frontend Setup](#frontend-setup)
- [Development](#development)
- [Environment Variables](#environment-variables)
- [API Endpoints](#api-endpoints)
- [Scripts](#scripts)
- [Contributing](#contributing)
- [License](#license)

---

## Features

- Modern, responsive frontend (React + TypeScript + Tailwind CSS)
- High-performance backend API (Rust + Axum + MongoDB)
- Environment-based configuration
- Easy local development and build
- Linting and formatting for code quality
- Validation for student data

---

## Project Structure

```
AttendEase/
  backend/    # Rust backend API
    src/
      models/
        student_model.rs
      routes/
        student_route.rs
      db.rs
      main.rs
      state.rs
    .env
    Cargo.toml
  frontend/   # React frontend
    src/
    package.json
    tailwind.config.js
```

---

## Getting Started

### Prerequisites

- [Node.js](https://nodejs.org/) (v16+ recommended)
- [npm](https://www.npmjs.com/) (comes with Node.js)
- [Rust](https://www.rust-lang.org/tools/install)
- [MongoDB](https://www.mongodb.com/)
- (Windows only) [Visual Studio C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)

---

### Backend Setup (Rust)

1. **Install dependencies:**
   ```sh
   cd backend
   cargo build
   ```
2. **Environment:**
   - Create a `.env` file in `backend/` with your environment variables (e.g., `MONGO_URI`).
3. **Run the backend:**
   ```sh
   cargo run
   ```

---

### Frontend Setup (React + Vite + Tailwind)

1. **Install dependencies:**
   ```sh
   cd frontend
   npm install
   ```
2. **Run the frontend in development mode:**
   ```sh
   npm run dev
   ```
3. **Open your browser:**
   - Visit the URL shown in the terminal (usually `http://localhost:5173`)

---

## Development

- **Backend:**
  - Rust code in `backend/src/`
  - Use `cargo run` for development
- **Frontend:**
  - React code in `frontend/src/`
  - Use `npm run dev` for hot-reloading
- **Linting/Formatting:**
  - Frontend: `npm run lint`, `npm run format`

---

## Environment Variables

- **Backend:**
  - Place a `.env` file in `backend/` with variables like:
    ```env
    MONGO_URI=<Your MongoDB URI>
    ```
- **Frontend:**
  - Place environment variables in `.env` files as needed (see Vite docs)

---

## API Endpoints

- `GET /`: Root endpoint, returns a message indicating the backend is running.
- `POST /students`: Adds a new student to the database.

---

## Scripts

### Frontend

- `npm run dev` — Start development server
- `npm run build` — Build for production
- `npm run preview` — Preview production build
- `npm run lint` — Lint code
- `npm run format` — Format code

### Backend

- `cargo run` — Run the backend server
- `cargo build` — Build the backend

---

## Contributing

1. Fork the repository
2. Create a new branch (`git checkout -b feature/your-feature`)
3. Commit your changes (`git commit -am 'Add new feature'`)
4. Push to the branch (`git push origin feature/your-feature`)
5. Open a Pull Request

---

## License

This project is licensed under the MIT License.

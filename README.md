# AttendEase

AttendEase is a full-stack web application designed for streamlined attendance management. It features a high-performance Rust backend using the Axum framework and a modern, responsive React frontend built with Vite and styled with Tailwind CSS.

---

## Table of Contents

- [Tech Stack](#tech-stack)
- [Features](#features)
- [Project Structure](#project-structure)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Backend Setup](#backend-setup)
  - [Frontend Setup](#frontend-setup)
- [Environment Variables](#environment-variables)
- [API Endpoints](#api-endpoints)
- [Available Scripts](#available-scripts)

---

## Tech Stack

- **Backend**: Rust, Axum, Tokio, MongoDB
- **Frontend**: React, TypeScript, Vite, Tailwind CSS
- **Database**: MongoDB

---

## Features

- **Full-Stack Application**: A complete solution with a separate frontend and backend.
- **High-Performance Backend**: Built with Rust and Axum for speed and reliability.
- **Modern Frontend**: A responsive and interactive user interface built with React and TypeScript.
- **Asynchronous Operations**: Leverages Tokio for non-blocking I/O in the backend.
- **Database Integration**: Uses MongoDB for flexible and scalable data storage.
- **Clear Project Structure**: Organized into distinct `frontend` and `backend` modules.

---

## Project Structure

```
AttendEase/
├── backend/        # Rust backend API
│   ├── src/
│   │   ├── models/ # Data models (Student, Teacher, Attendance)
│   │   ├── routes/ # API route handlers
│   │   ├── db.rs   # Database connection logic
│   │   └── main.rs # Application entry point
│   ├── .env        # Environment variables (requires MONGO_URI)
│   └── Cargo.toml  # Rust dependencies
└── frontend/       # React frontend application
    ├── src/        # Source files (TSX, CSS)
    ├── package.json  # Frontend dependencies and scripts
    └── vite.config.ts # Vite configuration
```

---

## Getting Started

### Prerequisites

- [Node.js](https://nodejs.org/) (v18+ recommended)
- [Rust](https://www.rust-lang.org/tools/install) and Cargo
- A running [MongoDB](https://www.mongodb.com/) instance

---

### Backend Setup (Rust)

1.  **Navigate to the backend directory:**
    ```sh
    cd backend
    ```
2.  **Create the environment file:**
    Create a `.env` file in the `backend` directory and add your MongoDB connection string:
    ```env
    MONGO_URI="your_mongodb_connection_string"
    ```
3.  **Build and run the server:**
    ```sh
    cargo run
    ```
    The backend server will start on `http://127.0.0.1:3000`.

---

### Frontend Setup (React)

1.  **Navigate to the frontend directory:**
    ```sh
    cd frontend
    ```
2.  **Install dependencies:**
    ```sh
    npm install
    ```
3.  **Run the development server:**
    ```sh
    npm run dev
    ```
    The frontend application will be available at `http://localhost:5173` (or the next available port).

---

## Environment Variables

-   **`MONGO_URI`**: **(Required)** The connection string for the MongoDB database. This is used by the backend to connect to your database instance. It should be placed in the `backend/.env` file.

---

## API Endpoints

The backend exposes the following REST API endpoints:

-   `GET /`: Root endpoint to confirm the backend is running.
-   `POST /students/add`: Adds a new student.
-   `POST /teacher/add`: Adds a new teacher.
-   `POST /attendance/mark`: Marks attendance for a student.

---

## Available Scripts

### Frontend (`/frontend`)

-   `npm run dev`: Starts the Vite development server with hot reloading.
-   `npm run build`: Compiles and bundles the app for production.
-   `npm run lint`: Lints the TypeScript and JavaScript files.
-   `npm run preview`: Serves the production build locally for previewing.

### Backend (`/backend`)

-   `cargo run`: Compiles and runs the backend server.
-   `cargo build`: Compiles the backend without running it.
-   `cargo check`: Checks the backend code for errors without compiling.
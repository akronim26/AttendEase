# CURRENTLY UNDER DEVELOPMENT

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
- **Frontend**: React, TypeScript, Tailwind CSS
- **Database**: MongoDB

---

## Features

- **Full-Stack Application**: Complete solution with separate frontend and backend
- **High-Performance Backend**: Built with Rust and Axum for speed and reliability
- **Modern Frontend**: React 19 with TypeScript and Vite for fast development
- **Asynchronous Operations**: Leverages Tokio for non-blocking I/O
- **MongoDB Integration**: Flexible and scalable NoSQL database storage
- **Comprehensive Error Handling**: Custom error types with proper HTTP status codes
- **Data Validation**: Input validation for roll numbers, emails, and subjects

---

## Project Structure

```
AttendEase/
├── backend/                    # Rust backend API
│   ├── src/
│   │   ├── models/            # Data models
│   │   │   ├── student_model.rs    # Student data structure
│   │   │   ├── teacher_model.rs    # Teacher data structure
│   │   │   └── attendance_model.rs # Attendance records
│   │   ├── routes/            # API route handlers
│   │   │   ├── student_route.rs    # Student CRUD operations
│   │   │   ├── teacher_route.rs    # Teacher management
│   │   │   └── attendance_route.rs # Attendance tracking
│   │   ├── db.rs              # MongoDB connection logic
│   │   ├── error.rs            # Custom error types and handling
│   │   ├── state.rs            # Application state management
│   │   └── main.rs             # Application entry point
│   ├── Cargo.toml              # Rust dependencies and metadata
│   └── .env                    # Environment variables (requires MONGO_URI)
└── frontend/                   # React frontend application
    ├── src/
    │   |── App.tsx             # Main application
    │   ├── main.tsx            # Application entry point
    │   ├── App.css             # Application styles
    │   └── index.css           # Global styles
    ├── package.json            # Frontend dependencies and scripts
    ├── tailwind.config.js      # Tailwind CSS configuration
    ├── vite.config.ts          # Vite build configuration
    └── tsconfig.json           # TypeScript configuration
```

---

## Getting Started

### Prerequisites

- [Node.js](https://nodejs.org/) (v18+ recommended)
- [Rust](https://www.rust-lang.org/tools/install) and Cargo
- A running [MongoDB](https://www.mongodb.com/) instance

---

### Backend Setup (Rust)

1. **Navigate to the backend directory:**

   ```sh
   cd backend
   ```

2. **Create the environment file:**
   Create a `.env` file in the `backend` directory and add your MongoDB connection string:

   ```env
   MONGO_URI="your_mongodb_connection_string"
   ```

3. **Build and run the server:**
   ```sh
   cargo run
   ```
   The backend server will start on `http://127.0.0.1:3000`.

---

### Frontend Setup (React)

1. **Navigate to the frontend directory:**

   ```sh
   cd frontend
   ```

2. **Install dependencies:**

   ```sh
   npm install
   ```

3. **Run the development server:**
   ```sh
   npm run dev
   ```
   The frontend application will be available at `http://localhost:5173` (or the next available port).

---

## Environment Variables

- **`MONGO_URI`**: **(Required)** The connection string for the MongoDB database. This is used by the backend to connect to your database instance. It should be placed in the `backend/.env` file.

---

## API Endpoints

The backend exposes the following REST API endpoints:

### Root

- **`GET /`**: Root endpoint to confirm the backend is running.
  - **Response**: `"Attendance portal backend is running"`

### Students

- **`POST /students/add`**: Adds a new student.

  - **Request Body**:
    ```json
    {
      "name": "John Doe",
      "email": "john.doe@example.com",
      "roll_number": 12345
    }
    ```
  - **Response**:
    - **201 Created**: Returns the created student with generated ID
    - **400 Bad Request**: If roll number is not positive
    - **409 Conflict**: If email already exists
    - **500 Internal Server Error**: Server-side errors

- **`GET /students/{student_id}`**: Retrieves a student by ID.
  - **Response**:
    - **200 OK**: Returns the student data
    - **404 Not Found**: If student doesn't exist
    - **500 Internal Server Error**: Server-side errors

### Teachers

- **`POST /teacher/add`**: Adds a new teacher.
  - **Request Body**:
    ```json
    {
      "name": "Jane Smith",
      "email": "jane.smith@example.com",
      "subject": "Mathematics"
    }
    ```
  - **Response**:
    - **201 Created**: Returns the created teacher with generated ID
    - **409 Conflict**: If email already exists
    - **500 Internal Server Error**: Server-side errors

### Attendance

- **`POST /attendance/mark`**: Marks attendance for a student.
  - **Request Body**:
    ```json
    {
      "student_id": "student_object_id",
      "subject": "Maths"
    }
    ```
  - **Response**:
    - **201 Created**: Returns the attendance record with timestamp
    - **400 Bad Request**: If subject is empty
    - **404 Not Found**: If subject is not in curriculum
    - **500 Internal Server Error**: Server-side errors

---

## Available Scripts

### Frontend (`/frontend`)

- **`npm run dev`**: Starts the Vite development server with hot reloading
- **`npm run build`**: Compiles and bundles the app for production
- **`npm run lint`**: Lints the TypeScript and JavaScript files
- **`npm run preview`**: Serves the production build locally for previewing

### Backend (`/backend`)

- **`cargo run`**: Compiles and runs the backend server
- **`cargo build`**: Compiles the backend without running it
- **`cargo check`**: Checks the backend code for errors without compiling

---

# Daily Time Planner

A modern time blocking application for planning your day with support for interruptions and re-planning. Built with Rust (Axum), SQLite, and SolidJS.

## Features

- **Single Day Planning**: Focus on one day at a time with customizable start/end times
- **15-Minute Increments**: Precise time blocking with 15-minute grid
- **Drag & Drop**: Create blocks by dragging, move them around easily
- **Interruption Management**: Mark interruptions to create new planning columns
- **Multiple Revisions**: Support for multiple interruptions in a single day
- **Time Alignment**: All columns show the same time range for easy comparison
- **Simple Authentication**: Password-based auth with session cookies
- **Persistent Storage**: SQLite database for reliable data storage

## Tech Stack

### Backend
- **Rust** with Axum web framework
- **SQLx** for database operations
- **SQLite** for data storage
- **Argon2** for password hashing
- **Session-based authentication**

### Frontend
- **SolidJS** for reactive UI
- **TypeScript** for type safety
- **@thisbeyond/solid-dnd** for drag-and-drop
- **date-fns** for date manipulation
- **Vite** for build tooling

## Getting Started

### Prerequisites

- Rust (1.70+)
- Node.js (18+)
- npm or yarn

### Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd Time-Block-Planner-
```

2. Set up the backend:
```bash
# Copy environment file
cp .env.example .env

# Build the backend
cargo build --release
```

3. Set up the frontend:
```bash
cd frontend
npm install
npm run build
cd ..
```

### Running the Application

#### Development Mode

1. Start the backend server:
```bash
cargo run
```

2. In a separate terminal, start the frontend dev server:
```bash
cd frontend
npm run dev
```

The frontend will be available at `http://localhost:5173` (Vite dev server)
The backend API runs on `http://localhost:3000`

#### Production Mode

1. Build the frontend:
```bash
cd frontend
npm run build
cd ..
```

2. Run the backend (it will serve the frontend static files):
```bash
cargo run --release
```

The application will be available at `http://localhost:3000`

## Database

The application uses SQLite with automatic migrations. The database file (`planner.db`) will be created automatically on first run.

### Schema

- **users**: User accounts with password hashes
- **sessions**: Active user sessions
- **plans**: Daily plans (one per user per day)
- **columns**: Plan columns (original + interruption revisions)
- **time_blocks**: Individual time blocks within columns

## Usage

### Creating a Plan

1. Log in or create an account
2. Select a date (defaults to today)
3. Set your day's start and end times (e.g., 8 AM - 5 PM)
4. Click and drag on the timeline to create time blocks
5. Name your blocks and assign colors

### Managing Interruptions

1. When an interruption occurs, click the "⚡ Mark Interruption" button
2. A new column appears starting at the current time
3. Drag blocks from the original plan to reschedule them
4. Create new blocks to handle the interruption
5. You can create multiple interruption columns as needed

### Editing Blocks

- **Move**: Click and drag blocks to different times
- **Edit**: Click on a block to edit its title
- **Delete**: Click the × button when editing
- **Complete**: Mark blocks as completed (coming soon)

## API Endpoints

### Authentication
- `POST /api/auth/register` - Create account
- `POST /api/auth/login` - Sign in
- `POST /api/auth/logout` - Sign out
- `GET /api/auth/me` - Get current user

### Plans
- `GET /api/plans/:date` - Get plan for date (creates if not exists)
- `POST /api/plans` - Create new plan
- `PUT /api/plans/:id` - Update plan
- `DELETE /api/plans/:id` - Delete plan

### Columns (Interruptions)
- `POST /api/columns` - Create interruption column
- `DELETE /api/columns/:id` - Delete column

### Time Blocks
- `POST /api/blocks` - Create time block
- `PUT /api/blocks/:id` - Update block
- `DELETE /api/blocks/:id` - Delete block

## Development

### Backend Structure
```
src/
├── main.rs           # Application entry point
├── db.rs            # Database setup and migrations
├── auth.rs          # Authentication logic
├── models.rs        # Data models
└── handlers/        # API route handlers
    ├── auth.rs
    ├── plans.rs
    ├── columns.rs
    └── blocks.rs
```

### Frontend Structure
```
frontend/src/
├── App.tsx          # Main application component
├── api/
│   └── client.ts   # API client
├── components/
│   ├── Login.tsx   # Authentication UI
│   ├── Planner.tsx # Main planner component
│   └── TimeGrid.tsx # Time grid with blocks
├── stores/
│   └── planStore.ts # Application state
└── types/
    └── index.ts    # TypeScript types
```

## License

MIT

## Contributing

Contributions welcome! Please open an issue or PR.

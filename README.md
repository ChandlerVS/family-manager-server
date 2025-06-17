# Family Manager
## Prelude
Due to the fact that I will become a father before the year ends, I began looking for open source software to keep myself and my family more organized. Features I wanted were meal planning, calendars, budgeting tools, etc. Unfortunutely I was unable to find an open source solution that had all the features I wanted, so instead I am going to take it on as a hobby project to build it myself.

I will focus on the features most important to me first, and build it module by module.

## Project Structure

The project is organized as a Rust workspace with three main components:

```
family-manager-server/
├── api/           # REST API layer and endpoints
├── application/   # Core business logic and domain models
├── database/      # Database interactions and models
└── Cargo.toml     # Workspace configuration
```

### Components

- **API**: Handles HTTP requests, input validation, and response formatting
- **Application**: Contains the core business logic, domain models, and use cases
- **Database**: Manages database connections, migrations, and data access

## Getting Started

### Prerequisites

- Rust (latest stable version)
- MySQL (for database)
- Docker (optional, for containerized development)

### Setup

1. Clone the repository:
   ```bash
   git clone https://github.com/ChandlerVS/family-manager-server.git
   cd family-manager-server
   ```

2. Install dependencies:
   ```bash
   cargo build
   ```

3. Set up the database:
   ```bash
   # Database setup instructions will be added as the project develops
   ```

4. Run the development server:
   ```bash
   cargo run
   ```

## Features (Planned)

- [ ] Meal Planning
- [ ] Family Calendar
- [ ] Budget Management
- [ ] Task Management
- [ ] Shopping Lists
- [ ] Recipe Management

## License

This project is licensed under the MIT License - see the LICENSE file for details.

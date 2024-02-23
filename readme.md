# Rust Sandbox

Welcome to the Rust Sandbox! This project serves as a playground for learning and experimenting with Rust functionalities. It consists of two separate Rust projects designed to work together:

- **api_project**: A backend API developed using [Rocket](https://rocket.rs/), a web framework for Rust.
- **rust_htmx_tera_project**: A frontend website developed with [HTMX](https://htmx.org/), [Tera](https://tera.netlify.app/), and [Actix](https://actix.rs/), providing a modern approach to building web applications in Rust.

## Getting Started

To get started with the Rust Sandbox, you'll need to have Rust installed on your machine. If you haven't already, you can install Rust by following the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).

### Prerequisites

- Rust toolchain (latest stable version recommended)
- A PostgreSQL database

### API Project

#### Configuration

Before running the API project, you need to set up your environment:

1. Create a file named `.env` within the `api_project` directory.
2. Add the following line to your `.env` file, replacing `username`, `password`, `localhost`, and `dbname` with your PostgreSQL credentials:

    ```
    DATABASE_URL=postgres://username:password@localhost/dbname
    ```

This environment variable will be used by the application to connect to your PostgreSQL database.

#### Running the Project

To run the API project, navigate to the `api_project` directory and execute the following command:

```bash
cargo run
```

Once the API project is running, Utoipa generates live documentation for the API endpoints. You can access the API documentation through the following URLs:

GET /redoc
GET /rapidoc
GET /swagger-ui/index.html
GET /api-docs/openapi.json
These URLs provide different views and formats for exploring and testing the API's endpoints, making it easier to understand and integrate with the backend services.

Rust HTMX Tera Project
The rust_htmx_tera_project directory contains the frontend of your application, developed with Actix, Tera, and HTMX. Make sure you have the API project running before you start the frontend to ensure they can communicate effectively.

Setup and Run
(TODO: Provide specific setup instructions for the Rust HTMX Tera project, such as installing dependencies, building the project, and running it.)

Contributing
We welcome contributions to the Rust Sandbox! Whether it's adding new features, fixing bugs, or improving documentation, your help makes this project better for everyone. Please read CONTRIBUTING.md for details on our code of conduct and the process for submitting pull requests to us.

License
This project is licensed under the MIT License - see the LICENSE.md file for details.
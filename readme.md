# Rust Sandbox

Welcome to the Rust Sandbox! This project serves as a playground for learning and experimenting with Rust functionalities. It consists of two separate Rust projects designed to work together:

- **api_project**: A backend API developed using [Rocket](https://rocket.rs/), a web framework for Rust.
- **leptos_project**: A frontend website developed with [Leptos](https://leptos.dev/), a modern framework for building web applications in Rust.

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

### Leptos Project

The `leptos_project` serves as the frontend for your application. Make sure you have the API project running before you start the frontend to ensure they can communicate effectively.

#### Setup and Run

(TODO: Provide specific setup instructions for the Leptos project, such as installing dependencies, building the project, and running it.)

## Contributing

We welcome contributions to the Rust Sandbox! Whether it's adding new features, fixing bugs, or improving documentation, your help makes this project better for everyone. Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct and the process for submitting pull requests to us.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.
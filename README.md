# Newsletter API

This is a Rust-based application that manages a newsletter API. It provides functionality for handling newsletter subscriptions, sending emails, and managing the associated data.

## Features

- User registration and subscription management
- Email sending using the Lettre library
- Database integration with PostgreSQL using SQLx
- RESTful API endpoints using Actix-web
- Environment configuration using dotenv
- Logging with tracing and tracing-subscriber
- UUID generation for unique identifiers
- Date and time handling with Chrono

## Dependencies

The main dependencies used in this project are:

- `actix-web`: Web framework for building the API endpoints
- `askama`: Template engine for rendering email templates
- `lettre`: Library for sending emails
- `sqlx`: Async PostgreSQL driver with compile-time checked queries
- `uuid`: Library for generating UUIDs
- `chrono`: Date and time library
- `serde`: Serialization and deserialization of JSON data
- `tracing` and `tracing-subscriber`: Logging and tracing functionality
- `dotenv`: Loading environment variables from a `.env` file
- `regex`: Regular expression support
- `secrecy`: Library for managing secrets and sensitive data

## Getting Started

1. Clone the repository:

   ```
   git clone https://github.com/dmcclung/newsletter-api.git
   ```

2. Set up the required environment variables in a `.env` file.

3. Run the database migrations:

   ```
   sqlx migrate run
   ```

4. Build and run the application:

   ```
   cargo run
   ```

5. The API will be available at `http://localhost:3000`.

## API Endpoints

- `POST /subscriptions`: Subscribe to the newsletter
- `GET /subscriptions`: Get all subscriptions
- `GET /subscriptions/{id}`: Get a specific subscription by ID
- `DELETE /subscriptions/{id}`: Unsubscribe from the newsletter
- `POST /newsletter`: Publish a newsletter

## Testing

To run the tests, use the following command:

```
cargo test
```

## Contributing

Contributions are welcome! If you find any issues or have suggestions for improvements, please open an issue or submit a pull request.

## License

This project is licensed under the [MIT License](LICENSE).

---

Feel free to customize and enhance the README based on your specific application's features and requirements.

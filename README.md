# GLabs Rust Library

Collection of rust helper library specific for GL rust framework. Unit test (`cargo test`) and documentation (`cargo doc --open`) included.

## Features

- `base` - Struct & implementations for api and web url
- `catchers` - 404 page and json response
- `ciphers` - Encryption and Decryption library
- `conversions` - Trivial conversions from one type to another
- `cors` - CORS middleware
- `databases` - DBPool enum that supports r2d2 which allows the actix web app to run with or without database connection
- `favicons` - Favicon handler
- `files` - Struct & implementations for commonly used file information
- `guards` - Guard related middlewares
    - `guards::Database` - Prevents routes from displaying an endpoint if database pool does not exist
    - `guards::Role` - create guard which handles role locking
- `hbs` - Handlebars specific functions
- `mailers` - SMTP sender
- `paseto` - Paseto generation and validation
- `payloads` - Payload struct and implementations and JSON configurations
- `s3` - S3 specific functions
- `scheduler` - CRON implementation
- `socials` - Social media logins specific struct and implementations
- `sse` - Server sent events helper
- `strings` - String specific helpers
- `tokens` - Token specific helpers
- `traits` - Custom traits
- `user_agent` - user agent parser middleware
- `validate` - Functions for validating types and fields
- `websocket` - Web socket helpers

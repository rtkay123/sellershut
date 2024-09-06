# sellershut-core

`sellershut-core` is a foundational library providing core types and entities used across the platform
## Build Dependencies
- `protoc`

## Features

`sellershut-core` provides various features organised by microservice and common functionalities. Enable the features you need through `cargo` feature flags

### Category Features

These features are related to the types and functionalities used by the [Categories API](../crates/api-categories):

- **`categories`**: Enables the `Category` as well as types related to it used in pagination
- **`rpc-client-categories`**: Generates a gRPC client implementations
    - Enables the `categories` feature
- **`rpc-server-categories`**: Generates a gRPC server implementations
    - Enables the `categories` feature

### Common Features

These features are used across multiple microservices or provide utility functions:

- **`serde`**: Derives `serde::Deserialize` and `serde::Serialize` for types
- **`id-gen`**: Exposes a function to generate a nano ID
- **`tracing`**: Enable structured and context-aware logging

## Usage

To use `sellershut-core` in your Rust project, add the dependency:

```toml
[dependencies]
sellershut-core = { path = "path/to/core-lib", features = ["rpc-client-categories"] }
```

## Examples
> [!NOTE]  
> Examples are configured at **workspace** level
- [`serde`](../../examples/serde_integration/)
Serialisation and deserialisation of a `Category` with serde:
    ```rust
    cargo run --example serde
    ```
- [`grpc_categories`](../../examples/grpc_categories/)
A gRPC client and server implementation for querying categories
    ```rust
    cargo run --example grpc_server_categories
    cargo run --example grpc_client_categories
    ```
> [!IMPORTANT]  
> Run the server first 

## Contributing

Contributions are welcome! Please follow these steps:

1. Fork the repository and clone it locally.
2. Create a new branch for your feature or fix.
3. Implement your changes and add tests if applicable.
4. Submit a pull request with a detailed description of your changes.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

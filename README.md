# Identeco Utoipa & Schemathesis Example

The [OpenAPI](https://www.openapis.org/) specification has become the industry standard for defining APIs.
It is clear that ensuring accurate and up-to-date API documentation is essential for both users and developers.
This example demonstrates how [Utoipa](https://github.com/juhaku/utoipa) and [Schemathesis](https://github.com/schemathesis/schemathesis) can be used in Rust projects to make the API documentation process less tedious and error-prone by automatically generating and validating an OpenAPI definition directly from code.

The code is based on real world usage of these tools at Identeco and their [Credential Checker](https://identeco.de/en/products/credential-check/) service.
For a walkthrough behind the showcased concepts, read the full associated blog article "Auto-Generating & Validating OpenAPI Docs in Rust: A Streamlined Approach with Utoipa and Schemathesis" on [Identeco's blog](https://identeco.de/en/blog/).

## Prerequisites

- [Rust & Cargo](https://www.rust-lang.org/tools/install)

## Usage

To generate the OpenAPI docs, use:

```bash
cargo run --bin gen_api
```

and to start the server on `localhost:8080`, use:

```bash
cargo run --bin server
```

Once the server is running you can view the Swagger UI at `localhost:8080/swagger-ui/`.

## Validation

To check if the `openapi.yml` file is up to date with the code, use:

```bash
cargo test
```

If you want to execute `schemathesis` locally, you can use the `api-tests.sh` script.
Naturally, `schemathesis` and `wait-for-it` has to be installed on your system for that.
An additional .`gitlab-ci.yml` file is included to demonstrate how to run the tests in a CI pipeline.

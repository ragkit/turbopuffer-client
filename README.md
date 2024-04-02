<h1 align="center"><code>turbopuffer-client</code>
</h1>

<div align="center">
  <a
    href="https://crates.io"
    style="text-decoration: none;">
    <img
      src="https://img.shields.io/crates/v/turbopuffer-client.svg"
      alt="crates.io badge">
  </a>
  <a
    href="https://github.com/ragkit/turbopuffer-client/actions"
    style="text-decoration: none;">
    <img
      src="https://github.com/ragkit/turbopuffer-client/actions/workflows/ci.yml/badge.svg"
      alt="ci badge">
  </a>
  <a
    href="./LICENSE"
    style="text-decoration: none;">
    <img
      src="https://img.shields.io/badge/license-MIT-blue"
      alt="license badge">
  </a>
</div>
<br>

# Overview

This is a Rust Client for interacting with the [Turbopuffer](https://turbopuffer.com/) vector database. For full API docs see: [https://turbopuffer.com/docs](https://turbopuffer.com/docs).

## Usage

Install via `Cargo.toml`:

```toml
[dependencies]
turbopuffer-client = "0.0.1"
```

Create a client using your API key from [turbopuffer.com](https://turbopuffer.com/):

```rust
let client = turbopuffer_client::Client::new(&api_key);
```

All operations are scoped to a namespace:

```rust
let ns = client.namespace("test");
```

Initialize the namespace with some vectors using `serde_json::json!()` to build the body, and `ns.upsert` to send the request:

```rust
let body = json!({
  "ids": [1, 2, 3, 4],
  "vectors": [[0.1, 0.1], [0.2, 0.2], [0.3, 0.3], [0.4, 0.4]],
  "attributes": {
    "my-string": ["one", null, "three", "four"],
    "my-uint": [12, null, 84, 39],
    "my-string-array": [["a", "b"], ["b", "d"], [], ["c"]]
  }
});

let res = ns.upsert(&vectors).await.unwrap();

// This is the response type.
assert!(matches!(
  res,
  UpsertResponse {
    status: response::Status::Ok
  }
));
```

Query the namespace similarly, but using `ns.query`:

```rust
let query = json!({
  "vector": [0.105, 0.1],
  "distance_metric": "euclidean_squared",
  "top_k": 1,
  "include_vectors": true,
  "include_attributes": ["my-string"],
});

let res = ns.query(&query).await.unwrap();

// Then you have access to the ResponseVectors:
let first = res.vectors.first().unwrap();
assert!(matches!(first.id, response::Id::Int(1)));
```

Finally, you can delete a namespace using `ns.delete`:

```rust
let res = ns.delete().await.unwrap();

// This is the response type.
assert!(matches!(
  res,
  DeleteResponse {
    status: response::Status::Ok
  }
));
```

# Contributing

We warmly welcome any contributions, for more info see: [`CONTRIBUTING.md`](./CONTRIBUTING.md)

# Code of Conduct

Contributions, including communications over issues, must follow our code of conduct. See: [`CONDUCT.md`](./CONDUCT.md).

# License

[MIT](./LICENSE)

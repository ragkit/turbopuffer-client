# `turbopuffer-client`

[![crates.io badge](https://img.shields.io/crates/v/turbopuffer-client.svg)](https://crates.io/crates/turbopuffer-client)
[![ci badge](https://github.com/ragkit/turbopuffer-client/actions/workflows/ci.yml/badge.svg)](https://github.com/ragkit/turbopuffer-client/actions)
[![license badge](https://img.shields.io/badge/license-MIT-blue)](./LICENSE)

# Overview

This is a Rust Client for interacting with the [`turbopuffer`](https://turbopuffer.com/) vector database.

_Note: Right now, this library doesn't validate the JSON body, it simply passes what you construct to turbopuffer. For a full list of options see [https://turbopuffer.com/docs](https://turbopuffer.com/docs)._

## Usage

Install via `Cargo.toml`:

```toml
[dependencies]
turbopuffer-client = "0.0.2"
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

let res = ns.upsert(&body).await.unwrap();

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

# vaultrs

> A rust crate for interacting with the Hashicorp Vault API

This crate encompasses functions for interacting with the HTTP API available on
[Hashicorp Vault](https://www.vaultproject.io/) servers. It uses 
[rustify](https://github.com/jmgilman/rustify) in order to construct accurate
representations of each of the endpoints available with the API. It then wraps
these into more usable functions intended to be consumed by users of this crate.

The following functionality is currently supported:

* [KV Secrets Engine V2](https://www.vaultproject.io/docs/secrets/kv/kv-v2)
* [PKI Secrets Engine](https://www.vaultproject.io/docs/secrets/pki)
* [Response Wrapping](https://www.vaultproject.io/docs/concepts/response-wrapping)

## Installation

```
cargo add vaultrs
```

## Usage

```rust
use vaultrs::api::pki::requests::GenerateCertificateRequest;
use vaultrs::client::{VaultClient, VaultClientSettingsBuilder};
use vaultrs::kv2;
use vaultrs::pki::cert;

// Create a client
let client = VaultClient::new(
    VaultClientSettingsBuilder::default()
        .address("https://127.0.0.1:8200")
        .token("TOKEN")
        .build()
        .unwrap()
).unwrap();

// Create and read secrets
struct MySecret {
    key: String,
    password: String,
}

let sec = MySecret {
    key: "super".to_string(),
    password: "secret".to_string(),
};
kv2::set(
    &client,
    "secret",
    "mysecret",
    &sec,
);

let sec = kv2::read::<MySecret>(&client, "secret" "mysecret");

// Generate a certificate using the PKI backend
let cert = cert::generate(
    &client,
    "pki",
    "my_role",
    Some(GenerateCertificateRequest::builder().common_name("test.com")),
);

```

## Error Handling

All errors generated by this crate are wrapped in the `ClientError` enum 
provided by the crate.

## Testing

See the the [tests](tests) directory for tests. Run tests with `cargo test`.

**Note**: All tests rely on bringing up a local Vault development server using
Docker. The Docker CLI must be installed on the machine running the tests and
you must have permission to start new containers. 

## Contributing

1. Fork it (https://github.com/jmgilman/vaultrs/fork)
2. Create your feature branch (git checkout -b feature/fooBar)
3. Commit your changes (git commit -am 'Add some fooBar')
4. Push to the branch (git push origin feature/fooBar)
5. Create a new Pull Request

The largest need in terms of contributions is adding support for more endpoints
offered by the Vault API. Luckily, a lot of the work involved with configuring
the endpoints is abstracted away by 
[rustify](https://github.com/jmgilman/rustify). Additionally, enough endpoints
have already been created and so incorporating new ones often means copying
existing ones. 

### Architecture

The architecture of the source directory is as such:

* `src/` - Crate root directory
* `src/api` - Root directory containing raw endpoints and supporting functions
* `src/client.rs` - Source for the client
* `src/error.rs` - Contains the common error enum for this crate
* `src/lib.rs` - Crate root file
* `src/*.rs` - Main API functions that wrap endpoints located in `src/api`

For example, the PKI engine is organized as such:

* `src/api/pki/requests.rs` - Contains all endpoints associated with this engine
* `src/api/pki/responses.rs` - Contains all responses from the endpoints
* `src/pki.rs` - Contains the high level functions for interacting with the engine

Aditionally, the `src/pki.rs` file is further organized into modules which help
break up the API functions available. For example, `pki::certs` contains
functions for working with certificates and `pki::roles` contains functions for
configuring roles. 

### Adding functionality

1. Create a new directory under `src/api` for the engine type if it's not
   already been added.
2. Add endpoints to `src/api/{engine}/requests.rs` and their associated
   responses to `src/api/{engine}/responses.rs`.
3. Add high level functions that use the endpoints in `src/{engine}.rs`. 
4. Add tests for each high level function in `tests/{engine}.rs`. 

See existing endpoints for examples on how to configure and document them. For
additional information see the 
[rustify documentation](https://docs.rs/rustify/0.1.0/rustify/).

All tests use a live instance of the Vault server for testing against since
mocking cannot verify the endpoint structures are accurate and valid. This also 
allows pinning to specific versions of Vault and adding support for newer 
versions as needed. While the tests are not intended to test the Vault server 
itself, it's recommended to  perform necessary setup to imitate end-user
behavior.

Bear in mind that the response of an endpoint may change based on the input
given by the user. For example, a different response is generated by the root
CA generation endpoint depending on if an internal or external CA is requested.
It's therefore important to mark fields as `Optional<>` where necessary. 

The underlying API functions will handle most errors for you and all that should
be needed is to propogate them up the stack. 
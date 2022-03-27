# onion-routing

An onion-routing echo server, developed by Erik Borgeteien Hansen og Oda Alida Fønstelien Hjelljord.

## Functionality

### Directory Authority

The directory Authority (DA) server waits for nodes to announce them selfs and send their public key. When a node starts up it sends its address and it's public key to the DA  which stores them in the list of live nodes.

### Client

The client reads messages from the user, encrypts them and send them to the first node  in the given path. When a response is received, it's decrypted and printed to the user.

### Server

This server is a basic echo server, upon getting a message it will return the same message to the sender.

### Node

To be added.

## Future work and current weaknesses

### Future work

Plans for future work can be found on the [issue board](https://github.com/erikbhan/onion-routing/issues). The main feature that we would like to implement in further work with this project is to change the current echo server/client setup into a proxy for browsers and a HTTP server. This may be done by implementing a socks protocol, but the exact strategy is not yet decided.

### Weaknesses

To be written about.

## External dependencies

- [Rand](https://docs.rs/rand/latest/rand/) used to generate random keys.

- [Tokio](https://docs.rs/tokio/latest/tokio/) used to manage async await.

- [Aes_gcm](https://docs.rs/aes-gcm/latest/aes_gcm/) used for encryption and decryption.

## Installation

- Clone repository

- Use ``cargo run --bin dir_auth`` run to start the directory authority, a DA must be operating for the nodes and client to work

- Use ``cargo run --bin node`` run to start a node, a client will make use of up to three nodes

- Use ``cargo run --bin client`` run to start the client

## How to run tests

To run tests use ``cargo test`` to run all tests (not ignored) in project.

One test is ignored due to needing user input that is not available in ci/cd. To run ignored tests use ``cargo test -- --ignored``.

## API documentation

Link goes here.

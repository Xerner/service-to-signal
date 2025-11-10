# Horn Service Kuksa

This component implements the COVESA uService for the Horn. It uses the Zenoh transport for Eclipse uProtocol.
The service supports several configuration options that can be provided on the command line or via environment variables.
Please use the `--help` switch to get all relevant information:

```bash
cargo run -- --help
```

## Server Architecture

This service is setup to use two threads for processing requests

- Thread A: An RPC server listens for incoming horn requests and sends them to a tokio channel for processing
- Thread B: The horn service processes incoming requests from the tokio channel and interacts with the horn actuator

## uProtocol Usage & Benefits

The underlying communication strategy is an RPC server implemented through the `up-rust` library. 

uProtocols `RequestHandler` trait interface is being used to define how to handle the incoming horn requests. This causes the request handling code to be unaware of what underlying RPC server implementation is being used, and only care about RPC server handling concepts such as resource IDs and request/response payloads. 

It is a good practice to abstract away implementations that have a well-defined public interface and expected usage (i.e. RPC servers). When something is well-defined and publicly available through multiple implementations, you never know when you might need to swap out your underlying implementation for another one. 

By using the `RequestHandler` trait, refactoring the code when swapping out the RPC server implementation becomes an issue isolated to just the `HornRpcServer` implementation. In contrast to having to refactor every single `RequestHandler` implementation.
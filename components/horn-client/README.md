# Horn Client

The `Horn Client` package implements a Client for the Horn service over Eclipse uProtocol from the COVESA uServices. This implementation relies on Eclipse Zenoh for the transport layer of the Eclipse uProtocol communication.

The client supports several configuration options that can be provided on the command line or via environment variables. In this directory, run...

To see all available options

```bash
cargo run -- --help
```

To start the client using the default configuration:

```bash
cargo run
```

To start the client using a different configuration file:

```bash
cargo run -- --config path/to/your/config/file
```

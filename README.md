# Talking to Eclipse Kanto via a unix socket

First checkout all submodules:

`git submodule update --init --recursive`

Set the unix the proper unix socket path in:

`src/main.rs`

Build

```bash
cargo build --release
```

Run as root so you can bind to the socket!!



```bash
sudo target/release/talk-to-kanto 
```

Example output:

```console
RESPONSE=Response { metadata: MetadataMap { headers: {"content-type": "application/grpc", "grpc-status": "0", "grpc-message": ""} }, message: ListContainersResponse { containers: [] }, extensions: Extensions }
```

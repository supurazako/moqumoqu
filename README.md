# moqumoqu

An educational implementation of Media over QUIC Transport (MOQT) in Rust.

This project is being developed to understand MOQT through implementing its wire format, protocol model, and state machine from scratch.

## Status

This implementation is incomplete and is not intended for production use. The target is `draft-ietf-moq-transport-22`, but the current protocol code originated as a draft-19 prototype and has not yet been reviewed for draft-22 conformance.

The current code contains initial wire-format primitives for variable-length integers, key-value pairs, and SETUP, plus Request ID allocation and validation. Most control messages, data framing, session state, and QUIC/WebTransport communication are not implemented yet.

## Testing

```bash
cargo test
```

## License

Licensed under either of

- Apache License, Version 2.0
- MIT License

at your option.

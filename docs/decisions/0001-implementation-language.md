# ADR 0001: Implementation language

- Status: Accepted
- Date: 2026-10-09

## Context

moqumoqu is an educational implementation of MOQT, targeting draft-22 and later revisions. The protocol has a binary wire format, distinct message variants, state transitions, and transport options over native QUIC or WebTransport.

## Decision

Use Rust for the implementation.

## Rationale

- Rust enums and exhaustive pattern matching fit MOQT's closed sets of messages and states. Newtypes and enums also make it practical to encode protocol distinctions in types and have the compiler check their handling.
- Quinn provides native QUIC, and wtransport provides WebTransport streams and datagrams. Rust can be used across both transport paths.
- Type-driven protocol modeling and QUIC/WebTransport integration are both useful parts of the project's learning goal.

TypeScript can also model message variants with discriminated unions and check exhaustive branches. The choice does not assume that TypeScript lacks explicit types. TypeScript types are erased at runtime, so its wire decoder must validate received bytes before treating them as typed protocol values. Rust also requires explicit wire validation; its benefit is compiler checking after decoding.

## Consequences

- The project will also involve learning Rust ownership, borrowing, and asynchronous programming. Keep abstractions and dependencies understandable so protocol behavior remains readable.
- This decision selects the language only. Choosing Quinn, wtransport, or another transport-library arrangement is a separate decision.

## References

- [MOQT transport draft-22](https://datatracker.ietf.org/doc/html/draft-ietf-moq-transport-22)
- [Rust Book: Enums and Pattern Matching](https://doc.rust-lang.org/book/ch06-00-enums.html)
- [Quinn documentation](https://quinn-rs.github.io/quinn/quinn.html)
- [wtransport documentation](https://docs.rs/wtransport/latest/wtransport/)
- [TypeScript Handbook: Unions and Intersection Types](https://www.typescriptlang.org/docs/handbook/unions-and-intersections.html)
- [TypeScript for the New Programmer](https://www.typescriptlang.org/docs/handbook/typescript-from-scratch)

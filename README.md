# yaws TLS Orbits

This repository consists of the [Blueprint] & [Orbit] implementations of Transport Layer Security (TLS) part of YAWS.

The below options are wired up:

| Suite  | Requires std? | Requires alloc? | BYO CryptoRng | BYO Crypto         |
| :---   | :---          | :---            | :---          | :---               |
| [rustls] | opt-in        | yes             | no            | yes                |
| [yTLS]   | no            | no              | yes           | yes                |

Testing and validation is done against OpenSSL

Typically these Orbits are used and configured through YAWS runtimes.

To use them, see examples from the [test_ab](./test_ab) sources.

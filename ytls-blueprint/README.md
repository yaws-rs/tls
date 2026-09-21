# yTLS Blueprint

This crate provides yTLS Blueprint

yTLS is typically the primary suite for `#[no_std]` + no-alloc (i.e. embedded) environments.

Find more about yTLS itself from it's [book](https://yolotls.github.io/book/) and [repository](https://github.com/yolotls/yolotls).

## Crypto Providers

yTLS allows to bring a pluggable CryptoProcessors:

| Processor         | Description         |
| :---              | :---                |
| [ytls-rustcrypto] | RustCrypto provider |

If you find out more alternatives implemented, please send a PR to list here.

## See More

See [more](https://github.com/yaws-rs/tls) information about the overall structure.

[ytls-rustcrypto]: https://github.com/yolotls/yolotls/tree/main/crypto/rustcrypto

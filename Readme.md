# hsm2descriptors

Converts [c-lightning](https://github.com/ElementsProject/lightning) `hsm_secret` into an [output descriptors](https://github.com/bitcoin/bitcoin/blob/master/doc/descriptors.md)


Note the `hsm_tool` included in [c-lightning](https://github.com/ElementsProject/lightning) similarly allows to create descriptors from `hsm_secret` but only in watch-only mode.

## Usage

Requires [rust](https://www.rust-lang.org/)

```
$ cargo install hsm2descriptors
$ cat ~/.lightning/testnet/hsm_secret | hsm2descriptors --network testnet
wpkh(tpubD9ZjaMn3rbP1cAVwJy6UcEjFfTLT7W6DbfHdS3Wn48meExtVfKmiH9meWCrSmE9qXLYbGcHC5LxLcdfLZTzwme23qAJoRzRhzbd68dHeyjp/0/0/*)
```


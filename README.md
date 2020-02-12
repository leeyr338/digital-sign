# digital sign

A speck256k1 Digital Signature demo for learner.


## Usage

### Clone and Build

```bash
$ git clone https://github.com/leeyr338/digital-sign.git
$ cd digital-sign
$ cargo install --path .
```

It will install to `~/.cargo/bin/digital-sign`.

### Examples

- Get help:

```shell
$ digital-sign --help
digital-sign 0.1.0

USAGE:
    digital-sign [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    sign       Sign a data.
    recover    Recover a signature
    help       Prints this message or the help of the given subcommand(s)
```

- Get a secp256k1 key from [cita-cli](https://github.com/citahub/cita-cli)

```shell
cita> key create
{
  "address": "0x65d1f3361e43a07da738d7ec104d12d0d753bac6",
  "private": "0x6b3b5b1d6077dd83c8f2b0adbf356939d76bdff2b94a14fdb4358daf369905d7",
  "public": "0x43604efa1c8a2e85cd67dd15e61a5c0a8d5973a5b667c93ce50f5c4ad37a9009c47c3debb278e3e951dc713cf1834b260b79acc5fd8a1da814bc9bab679bb8db"
}
```

- Sign some data:

```shell
$ digital-sign sign --data "abcdefg" --private-key "0x6b3b5b1d6077dd83c8f2b0adbf356939d76bdff2b94a14fdb4358daf369905d7"
Signature is : fb938f2257680f9fdf575e43c932f3bace491ddcc55fd00fbc252faadb066c551e8c1609c93a7ea89dc638aa7bb672734b699a6c5cfb1b0e301529fbf80ce13f01
```

- Recover signer from signature and data

```shell
$ digital-sign recover --data "abcdefg" --sign "fb938f2257680f9fdf575e43c932f3bace491ddcc55fd00fbc252faadb066c551e8c1609c93a7ea89dc638aa7bb672734b699a6c5cfb1b0e301529fbf80ce13f01"
Address: 0x65d1f3361e43a07da738d7ec104d12d0d753bac6
Public key: 0x43604efa1c8a2e85cd67dd15e61a5c0a8d5973a5b667c93ce50f5c4ad37a9009c47c3debb278e3e951dc713cf1834b260b79acc5fd8a1da814bc9bab679bb8db
```

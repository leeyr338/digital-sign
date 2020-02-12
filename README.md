# digital sign

A speck256k1 Digital Signature demo for learner.


## Usage

### Clone and Build

```bash
$ git clone git@git.rivtower.com:cita/digital-sign.git
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
    issue    Issue a cita license file.
    info     Show cita license info.
    help     Prints this message or the help of the given subcommand(s)
```

- Issue a CITA license:

```shell
$ digital-sign issue --days 30 --private-key 0x9002a5f72861f7a4344cbf83e480c99d8b951a0ee2653c512afd82ab4248be93 --type 2 --value "0x82416af358bacf594f82b18d94f272e1aa2466cda66cbf66d7835d5651f113af" --version v1.1.0
CITA license issued!

$ cat cita.license 
MDEgdjEuMS4wIDIgMHg4MjQxNmFmMzU4YmFjZjU5NGY4MmIxOGQ5NGYyNzJlMWFhMjQ2NmNkYTY2Y2JmNjZkNzgzNWQ1NjUxZjExM2FmIDIwMjAtMDItMDRUMDI6MTE6MDBaIDIwMjAtMDMtMDVUMDI6MTE6MDBasfPxcuTcZR3Ichia5tomeHFamqEjzwvIbLqPsRIiBg+o/p1fT85SIjjBzcxDnsvDwHYKq2qR9lbQrbhr1Br7h0W+QpSecYwHGiKq3ctW0B8buloyKeNvo2Su2jBqkh29AQ==
```

- Show the CITA license info for a license file:

```shell
$  digital-sign info --license-file cita.lic
========== CITA License Info ==========
License version  : 01
CITA version     : v1.1.0
License type     : 2
License value    : 0x82416af358bacf594f82b18d94f272e1aa2466cda66cbf66d7835d5651f113af
Effective date   : 2020-02-04T07:23:43Z
Expiration date  : 2020-03-05T07:23:43Z
Finger Print     : 0xe8908946670b6ef3475965bfb1eaea53bafe44da3a351433c2ca307cff96d0b0
Issuer           : 0xd4565c152aa8f786920846e49f08782e161e3254
=======================================
```


## Contributing

Please submit to [https://git.rivtower.com/cita/digital-sign](https://git.rivtower.com/cita/digital-sign).

## Follow us

[Weibo](https://weibo.com/u/7243995427)


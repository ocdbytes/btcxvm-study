# BEE VM & BTC SCRIPT ₿

```diff
!⠀⠀⠀ ⠀⠀⠀⠀⠀⣀⣤⣴⣶⣾⣿⣿⣿⣿⣷⣶⣦⣤⣀⠀⠀⠀⠀⠀⠀⠀⠀
!⠀⠀ ⠀⠀⠀⣠⣴⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣦⣄⠀⠀⠀⠀⠀
!⠀ ⠀⠀⣠⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣄⠀⠀⠀
!⠀ ⠀⣴⣿⣿⣿⣿⣿⣿⣿⠟⠿⠿⡿⠀⢰⣿⠁⢈⣿⣿⣿⣿⣿⣿⣿⣿⣦⠀⠀
! ⠀⣼⣿⣿⣿⣿⣿⣿⣿⣿⣤⣄⠀⠀⠀⠈⠉⠀⠸⠿⣿⣿⣿⣿⣿⣿⣿⣿⣧⠀
! ⢰⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡏⠀⠀⢠⣶⣶⣤⡀⠀⠈⢻⣿⣿⣿⣿⣿⣿⣿⡆
! ⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠃⠀⠀⠼⣿⣿⡿⠃⠀⠀⢸⣿⣿⣿⣿⣿⣿⣿⣷
! ⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡟⠀⠀⢀⣀⣀⠀⠀⠀⠀⢴⣿⣿⣿⣿⣿⣿⣿⣿⣿
! ⢿⣿⣿⣿⣿⣿⣿⣿⢿⣿⠁⠀⠀⣼⣿⣿⣿⣦⠀⠀⠈⢻⣿⣿⣿⣿⣿⣿⣿⡿
! ⠸⣿⣿⣿⣿⣿⣿⣏⠀⠀⠀⠀⠀⠛⠛⠿⠟⠋⠀⠀⠀⣾⣿⣿⣿⣿⣿⣿⣿⠇
! ⠀⢻⣿⣿⣿⣿⣿⣿⣿⣿⠇⠀⣤⡄⠀⣀⣀⣀⣀⣠⣾⣿⣿⣿⣿⣿⣿⣿⡟⠀
! ⠀⠀⠻⣿⣿⣿⣿⣿⣿⣿⣄⣰⣿⠁⢀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠟⠀⠀
! ⠀⠀⠀⠙⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠋⠀⠀⠀
! ⠀⠀⠀⠀⠀⠙⠻⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠟⠋⠀⠀⠀⠀⠀
! ⠀⠀⠀⠀⠀⠀⠀⠀⠉⠛⠻⠿⢿⣿⣿⣿⣿⡿⠿⠟⠛⠉⠀⠀⠀⠀⠀⠀⠀⠀
```

## Index 📇
- [BEE VM & BTC SCRIPT ₿](#bee-vm--btc-script-)
  - [Index 📇](#index-)
  - [Scripting (OP CODE explanations) 📖](#scripting-op-code-explanations)
  - [BEE VM 🐝](#bee-vm-)
  - [BTC Helper tool 🛠](#btc-helper-tool-)
    - [Subcommands](#subcommands)

## Scripting (OP CODE explanations)

- [Arithmetic Script](./scripts/arithmetic_ops.bscript)
- [Crypto Ops Script](./scripts/crypto_ops.bscript)
- [Fund Freezing Script](./scripts/fund_freezing.bscript)
- [OPCODES](./scripts/opcodes.bscript)
- [P2PKH](./scripts/p2pkh.bscript)
- [Stack Ops](./scripts/stack_ops.bscript)
- [Time Lock Ops](./scripts/timelock_ops.bscript)

## BEE VM 🐝

```diff
!         _
!        /_/_      .'''.
!     =O(_)))) ...'     `.
! BEE    \_\              `.    .'''
! VM                        `..'⠀⠀⠀⠀⠀⠀⠀

[In Testing 🧰🛠️]
```

- [x] arithmetic ops
- [x] stack ops
- [x] flow control
- [x] standard opcodes
- [x] crypto ops
- [x] script tests
  - [x] arithmetic opcodes test
  - [x] stack opcodes test
  - [x] flow control test
  - [x] check sig / check multi sig / OP_CODESEPARATOR tests
- [ ] Script execution tests (In progress...)
  - [x] P2SH (Pay To Script Hash)
- [ ] time lock ops (Not supported currently)

## BTC Helper tool 🛠

```shell
$ cd helpers
$ cargo run -- --help

==================
BTC helper tools:
==================

1. Random Address Generator (generate)
2. Sign message with a private key (sign -p <secret_key> -m <message>)


[⚠️ Message will be hashed before signing]

Usage: helpers [COMMAND]

Commands:
  generate  Generates a random BTC Address
  sign      Signs a message with a private key
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Subcommands

- `generate` : To generate a random BTC address.
  ```shell
  $ cargo run -- generate

  Generated BTC Address : 022343e5da3d15f21c78c5268f0d32b4b2467a51b53d0beab44afc72e7303857b9
  Private Key : 4e43b777826af4981bf0973569ce7606268724c8d013781563731a4b3ac2d180
  ```
- `sign` : To sign the message with the given private key.
  ```shell
  $ cargo run -- sign -p 4e43b777826af4981bf0973569ce7606268724c8d013781563731a4b3ac2d180 -m 'hello world'

  Signature: 3045022100f39d01d1907c2e3fbf835fead1fbe0cdadbc2753ff6718df91d1f70586bb005f0220302045a2b9d0f910c9ad493f78a290df46fb1e28e91915b456b0dd59f958d5d7
  ```

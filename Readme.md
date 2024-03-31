# BIT_FUN

<div align='center'>
    <img src="https://cryptologos.cc/logos/bitcoin-btc-logo.png?v=029" width=50 height=50></img>
</div>

Fun with BTC nothing to see here.....

OR

- OPCODES
- BASIC SCRIPTS
- STACK OPS

```sh

# To read the stack

STACK = TOP_OF_STACK-->[ 0x10 0x20 0x30 ]
top element = 0x10

```

## VM

### To run

```sh
cd btc-vm
cargo build
cargo run '1 4 OP_ADD'
```

### VM inputs

| opcode       | vm input |
| ------------ | -------- |
| OP_ADD       | OP_ADD   |
| OP_SUB       | OP_SUB   |
| OP_0 - OP-16 | 0 - 16   |

### Current VM state

```sh
$ cargo run '1 2 OP_ADD'

======================================================
BTC VM Simulator
======================================================

>>>>> OPERATIONS : ["1", "2", "OP_ADD"]

>>>> Processing code : "1"
STACK : []
>>>> Processing code : "2"
STACK : ["1"]
>>>> Processing code : "OP_ADD"
STACK : ["1", "2"]

--------------------STACK (final) : ["3"]

```

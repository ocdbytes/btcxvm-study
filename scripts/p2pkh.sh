#!/bin/bash

: '
P2PKH : pay 2 public key hash

DUP
HASH160
<BOB_PUB_KEY_HASH>
EQUAL
CHECKSIG
'

BOB_SIG=""
BOB_PUB_KEY_HASH=""

# executing the script using btcdeb.
btcdeb '[$BOB_SIG BOB_PUB_KEY_HASH OP_DUP OP_HASH160 BOB_PUB_KEY_HASH OP_EQUAL OP_CHECKSIG]'



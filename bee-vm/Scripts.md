# Bee VM BTC Script Versions

## P2SH (Pay to Script Hash)

### Bee VM Version

```shell
<public key>
OP_HASH160
<hash(public key)>
OP_EQUALVERIFY
<public key>
<signature(hash(redeem script))>
OP_CHECKSIG
```

### BTC Original Version

```shell
# Redeem script
OP_DUP
OP_HASH160
<pubKeyHash>
OP_EQUALVERIFY
OP_CHECKSIG

# P2SH scriptPubKey (locking script)
OP_HASH160
<redeemScriptHash>  # Hash of the above redeem script
OP_EQUAL

# ScriptSig (unlocking script)
<signature>
<publicKey>
<redeemScript>
```

# Rand - An updated version of the drand client as a CosmWasm smart contract  

## Build
```
make build
```

## Unit tests
```
make test
```

## Integration tests
```
make integration-test
```

## Production build
```
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.12.5
```
### Binary located in:
```
artifacts/rand.wasm
```

## Deployed Contract
This contract is deployed on the Juno mainnet at:
```
juno1shxdqedq06dqxrw2kxque8n6fnkpufuy3gge2fmyaxs6v9p8nmtq6ueqf7
```

## Example Transactions
An example `SetBounty` transaction can be found at txhash:
```
0A05B0E65F1395C69FFADFB049E3C84341D703A1A3A04BA90BE3706CBD019261
```

An example `Add` transaction can be found at txhash:
```
494157C2BD9FF34158DBA5B8455110367A5CBDACD201EA2920BAE91AFA978CE0
```
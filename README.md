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
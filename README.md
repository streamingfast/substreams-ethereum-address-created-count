## Substreams Ethereum Address Created Count

Follow instructions at https://substreams.streamingfast.io/getting-started, once all configured, you can do:

```
cargo build --release
substreams run -e mainnet.eth.streamingfast.io:443 substreams.yaml store_address_created -s 15822427
```

> Change 15822427 to get the count at a different block value

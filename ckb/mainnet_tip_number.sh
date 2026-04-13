#!/usr/bin/env bash

curl -s -X POST https://mainnet.ckb.dev -H 'Content-Type: application/json' -d '{ "id": 42, "jsonrpc": "2.0", "method": "get_tip_header", "params": [ ] }' | jq -r .result.number | xargs -I{} printf "%d\n" {}


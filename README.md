# power-grid


API for interacting with a fleet of LND nodes.

## Development


Run each command in a separate terminal.
```
cargo watch -x run

```

```
curl http://localhost:8000/v1/node/getinfo/carol | jq

{
  "status": "success",
  "message": {
    "version": "0.16.0-beta commit=v0.16.0-beta",
    "commit_hash": "51df3e5b1ea2f1f749652cecf26e7471d35aa629",
    "identity_pubkey": "0228c2d0c69469e4e3999881e7f3614d352f297ba9fed8de4d2c9bbeba7a335ae9",
    "alias": "carol",
    "color": "#3399ff",
    "num_pending_channels": 0,
    "num_active_channels": 1,
    "num_inactive_channels": 0,
    "num_peers": 2,
    "block_height": 125,
    "block_hash": "6a878d17edbd0c9016c2b69711a16b9271b9d86a1fc9cea79ccb9802632c8368",
    "best_header_timestamp": 1681258651,
    "synced_to_chain": false,
    "synced_to_graph": true,
    "uris": [
      "0228c2d0c69469e4e3999881e7f3614d352f297ba9fed8de4d2c9bbeba7a335ae9@172.21.0.3:9735"
    ],
    "require_htlc_interceptor": false,
    "store_final_htlc_resolutions": false
  }
}

```

Create invoices

```
curl -XPOST -H "Content-Type: application/json" http://localhost:8000/v1/invoice/create -d '{"node_name":"carol", "memo":"beer","millisat": 100000}' | jq

{
  "message": {
    "r_hash": "YT311QKvDxsLqJs5VBRZcR/qJRKktVnvkMjwuGSjmPQ=",
    "add_index": 21,
    "payment_addr": "1VocULn6QhOMloYvHJM7IPl7ShZ3gt0Qp/zm62hL4Go=",
    "payment_request": "lnbcrt1u1pjt397ypp5vy7lt4gz4u83kzagnvu4g9zewy075fgj5j64nmuserctse9rnr6qdq8vfjk2uscqzpgxqyz5vqsp564dpc59elfpp8ryksch3eyemyruhkjskw7pd6y98lnnwk6ztup4q9qyyssqqrgjrmp2xcdpkrhnvjwc7xvrglhwegc8xjn404qflpjegkaasje976f74q7lx4q3syd5l7eupma2l0469ajnma0egvevfkwc4m763xgp72tlfd"
  }
}
```

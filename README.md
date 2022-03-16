# 1. Build & Run

Build: ``cargo build --release``

Or [download](https://github.com/nulls-network/sign-order-service/releases) the compiled binaries.

Run: ``./binaryName [port]``

# 2. Use
## 2.1 Sign the order
URL： ``http://[yourIp]:[port]/sign``

Method： ``POST``

Content-Type： application/json

Request body:

```json
{
    "order_no": "202203160233113243",
    "chain_id": "100000001",
    "pay_token": "TR7NHqjeKQxGTCi8q8ZY4pL8otSzgjLj6t",
    "pay_amount": "1000000",
    "notify": "http://localhost:8080/notify",
    "private_key": "6704f9a70210bdaedd08fc89b7711c2b05fe68de91117886fd4931882232ac7f"
}
```

Response:

```json
{
  "order": {
    "order_no": "202203160233113243",
    "chain_id": "100000001",
    "pay_token": "TR7NHqjeKQxGTCi8q8ZY4pL8otSzgjLj6t",
    "pay_amount": "1000000",
    "notify": "http://localhost:8080/notify",
    "private_key": "6704f9a70210bdaedd08fc89b7711c2b05fe68de91117886fd4931882232ac7f"
  },
  "sign": "314afe82e3771a031120a2ee96e509743918802a9992debe790d37bbbe8a2ada6a8496b2702722ff815260879346c0ac4f6aaa62b356de600a579b23c2f627b81c"
}
```

## 2.2 Parse the order signature

URL： ``http://[yourIp]:[port]/recover``

Method： ``POST``

Content-Type： application/json

Request body:

```json
{
  "order": {
    "order_no": "202203160233113243",
    "chain_id": "100000001",
    "pay_token": "TR7NHqjeKQxGTCi8q8ZY4pL8otSzgjLj6t",
    "pay_amount": "1000000",
    "notify": "http://localhost:8080/notify",
    "private_key": "6704f9a70210bdaedd08fc89b7711c2b05fe68de91117886fd4931882232ac7f"
  },
  "sign": "314afe82e3771a031120a2ee96e509743918802a9992debe790d37bbbe8a2ada6a8496b2702722ff815260879346c0ac4f6aaa62b356de600a579b23c2f627b81c"
}
```

Response:

```json
{
  "pubKey": "5db351243a4e9c0166b34cc7250dfdc8cfbb7ee9"
}
```

# 3. Warn

Do not run this service remotely, otherwise there is a risk of leaking the private key.

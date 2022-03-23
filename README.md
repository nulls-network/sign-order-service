# 1. Build & Run

Build: ``cargo build --release``

Or [download](https://github.com/nulls-network/sign-order-service/releases) the compiled binaries.

Run: ``./binaryName [port]``

# 2. Use
## 2.1 Sign message
URL： ``http://[yourIp]:[port]/sign``

Method： ``POST``

Content-Type： application/json

Request body:

```json
{
  "data": [
    "202203160233113243",
    "100000001",
    "TR7NHqjeKQxGTCi8q8ZY4pL8otSzgjLj6t",
    "1000000",
    "Y"
  ],
  "private_key": "6704f9a70210bdaedd08fc89b7711c2b05fe68de91117886fd4931882232ac7f"
}
```

Response:

```json
{
  "data": [
    "202203160233113243",
    "100000001",
    "TR7NHqjeKQxGTCi8q8ZY4pL8otSzgjLj6t",
    "1000000",
    "Y"
  ],
  "sign": "0xf425fe95976653607cc79324eb6911b1ff642473eb5554c850eed3ba69419c125ea61d49f3c57a46da507ec86f24ab2b336f6b2b869e9a56c4ec30d4103406601b"
}
```

## 2.2 Parse the message signature

URL： ``http://[yourIp]:[port]/recover``

Method： ``POST``

Content-Type： application/json

Request body:

```json
{
  "data": [
    "202203160233113243",
    "100000001",
    "TR7NHqjeKQxGTCi8q8ZY4pL8otSzgjLj6t",
    "1000000",
    "Y"
  ],
  "sign": "0xf425fe95976653607cc79324eb6911b1ff642473eb5554c850eed3ba69419c125ea61d49f3c57a46da507ec86f24ab2b336f6b2b869e9a56c4ec30d4103406601b"
}
```

Response:

```json
{
  "pub_key": "0x5db351243a4e9c0166b34cc7250dfdc8cfbb7ee9"
}
```

# 3. Warn

Do not run this service remotely, otherwise there is a risk of leaking the private key.

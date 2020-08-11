# CBOR to JSON
stdin should be raw cbor. stdout will be pretty printed JSON.

## How to compile
```
cargo build --release
```
find the application in `./target/release/cborjson`

## Useful script
This can be used to send graphql to a specific ip:port.

eg `./graphql.sh 192.168.0.10:8080 "{ping}"`

You may need to add the binary to your PATH for ease of use

```
echo $2 | socat -t 0.1 - udp-datagram:$1 | ./cborjson/target/release/cborjson
```

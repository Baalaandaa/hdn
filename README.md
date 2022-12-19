
# Hash Delivery Network
HSE Rust course project
## What it does?
This is a simple KV storage, that accepts 2 requests:  
```store(key, hash)``` and ```load(key)```
## Protocol
### Store
**Request**
```json lines
{
  "request_type": "store",
  "key": "key",
  "hash": "value"
}
```
**Response**
```json lines
{
  "response_status": "success"
}
```
### Load
**Request**
```json lines
{
  "request_type": "load",
  "key": "some_key"
} 
```
**Response if key exists**
```json lines
{
  "response_status": "success",
  "requested_key": "some_key",
  "requested_hash": "0b672dd94fd3da6a8d404b66ee3f0c83",
}
```
**Response if key NOT exists**
```json lines
{
  "response_status": "key not found",
}
```

## How to run
```shell
cargo run
cargo run -- --help
```
Show docs:
```shell
cargo doc --open
```

Run tests:
```shell
cargo test
```
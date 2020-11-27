# rust-sqlx-grpc-nestjs
## Nodejs client
```
cd nestjsClientGrpc &&
yarn start
```

## Docker compose
```
cd docker &&
docker-compose up -d
```

## Rust server
```
cd rust_server_grpc &&
cargo run --bin server
```

## Requests
### GET users request
```
curl --request GET \
--url localhost:3001/user \
| json_pp
```
### GET user by id request
```
curl --request GET \
--url localhost:3001/user/1 \
| json_pp
```
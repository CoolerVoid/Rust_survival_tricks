
# Simple questbook example in Rust using Rocket framework

This example uses Diesel+R2D2 with SQLite, Rocket to get inputs and return json...


To run you need rust nightly...

Second step, run migrate:

```
DATABASE_URL=test.db diesel migration run
```

Configure your TLS cert in file "Rocket.toml", run cargo:

```
ROCKET_PORT=3721 cargo run
```


If you want, you can set env vars, for example:

```
ROCKET_TLS={certs="/path/to/certs.pem",key="/path/to/key.pem"} cargo run
```


To test the service follow examples:

Test List function
```
$ curl https://localhost:3721/list -k
```

Test Insert function:
```
curl -d 'uid=alien_nevada&message="area 51"' -X POST https://localhost:3721/insert_questbook -k -vv
```

Test Delete:
```
curl -d 'id=1' -X POST https://localhost:3721/delete_questbook -k -vv
```

Other options read the file main.rs in directory src.


Study the docs 

https://rocket.rs

http://diesel.rs/



# Running in alpine linux docker
Output:
```
Making GET request.
```
Image seems to crash before it can give an output, but does not give an error of any kind. This only appears to happen with hyper-tls for https connections. See `hyper-alpine-works` for example with just hyper for an http connection

# Cargo run
When running cargo run, output is:
```
Making GET request.
Received Response! Code 200 OK
```
Acts as expected
# Running in debian linux docker
Output:
```
Making GET request.
Received Response! Code 200 OK
```
Acts as expected

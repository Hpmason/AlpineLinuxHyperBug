# Running in alpine linux docker
Output:
```
Making GET request.
Received Response! Code 200 OK
```
Using hyper for an http request acts as expected. Using hyper-tls with an alpine image does not, refer to hyper-alpine-fails.

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

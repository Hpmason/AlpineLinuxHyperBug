# AlpineLinuxHyperBug
When using hyper-tls in an alpine linux image, the image seems to crash without an error.
In the repo, there are 3 versions of this: `hyper-alpine-fails`,`hyper-alpine-works`, `reqwest-alpine`.

## hyper-alpine-fails
This is an example where hyper-tls request fails on alpine linux.
Output:
```
Making GET request.
```
Expected output:
```
Making GET request.
Received Response! Code 200 OK
```

## hyper-alpine-works
This example succeeds and the program finishes through

## reqwest-alpine
This example uses reqwest, which makes use of hyper-tls. This also fails the same way the hyper-alpine-fails.

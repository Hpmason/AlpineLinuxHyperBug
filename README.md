# AlpineLinuxHyperBug
When using hyper-tls in an alpine linux image, the image seems to crash without an error.
In the repo, there are 3 versions of this: `hyper-alpine-fails`,`hyper-alpine-works`, `reqwest-alpine`.

## hyper-alpine-works
This example uses just hyper with an http request. It succeeds and the program finishes through.

Expected output (and actual output from all images)
```
Making GET request.
Received Response! Code 200 OK
```

## hyper-alpine-fails
This is an example where hyper-tls request fails on alpine linux.
Output from alpine linux image:
```
Making GET request.
```
No error is received and printed out.


Expected output (and output from debian image and cargo run (tested one windows)):
```
Making GET request.
Received Response! Code 200 OK
```


## reqwest-alpine
This example uses reqwest, which makes use of hyper-tls. This also fails the same way the hyper-alpine-fails (works with cargo run (windows) and debian image)

Output from alpine linux image:
```
Making GET request.
```
No error is received and printed out.


Expected output (and output from debian image and cargo run (tested one windows)):
```
Making GET request.
Received Response! Code 200 OK
```



# Example project for verifying and fixing iron:#582

## Replicate the problem

Verify we can replicate the problem using the current version of iron from github.

```
$ make run-default
...
```

In a second terminal, check the response:

```
$ make verify
curl -s -i --head http://localhost:3000 | grep content-length
content-length: 0
```

This verifies the problem. The response length is 0, but we're expecting to see a length of 123 (hardcoded in src/main.rs).

## Verifying fixes

Verify the fix by using my fork of iron at github.com/bplower/iron.

```
$ make run-patch
...
```

In the second terminal, rerun verify and we will get the expected content length of 123.

```
$ make verify
curl -s -i --head http://localhost:3000 | grep content-length
content-length: 123
```

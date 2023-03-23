# Introduction

## Features

### Placeholders everywhere 

️placeholders at the beginning of an url e.g.
```sh
curlz r --define 'host=https://httpbin.org' '{{host}}/get'
````

placeholders in HTTP Headers, e.g.

```sh
curlz r -H 'Username: {{ env.USER }}' https://httpbin.org/headers
```

### JSON Payload | `--json`

This is a shortcut for setting 2 HTTP Headers and sending data as with `-d | --data`

#### Example

```sh
curlz r --json '{ "foo": "bar" }' -X POST 'https://httpbin.org/anything'
```

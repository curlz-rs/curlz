# Template function documentation

## Prompt User Input - `prompt_for(name: string)`

- arguments:
  - `name`: a name that is printed before the user would input data
- output: string
- notes:
  - don't use this for passwords, consider `prompt_password()` for this
- examples:
  - let the user enter an arbitrary username

    ```shell
    curlz r https://api.github.com/user -- \
      -u '{{ prompt_for("GitHub Username") }}:{{ prompt_password() }}'
    ```

## Prompt User for Password - `prompt_password()`

- arguments: None
- output: string 

### Example

let the user enter an arbitrary username:
```sh
curlz r -u '{{ prompt_for("GitHub Username") }}:{{ prompt_password() }}' https://api.github.com/user
```

## Json Web Token - `jwt(claims: map, [jwt_signing_key: string])`

- arguments:
  - `claims`: to be a map of key value pairs like `{"uid": "1234"}` that
    are the payload of the JWT
  - `jwt_signing_key`: to be a string,
    this is optional and when omitted a variable named `jwt_signing_key` will be taken from the `.env` file,
    if that variable is missing an error is raised
  - Note: also key value arguments are possible: `jwt(uid="1234", jwt_signing_key="999")`
- output: string is a Json Web Token (JWT)
- notes:
  - the hash algorithm is `HS256` and the JWT header is `{"alg": "HS256", "typ": "JWT"}`
  - the claim `exp` expiry timestamp (UNIX-Timestamp) is set to expire in 15min by default
    - can be overwritten by a UNIX-Timestamp or a duration string like "15min" or "1h"  
  - the claim `iat` issued at timestamp is set automatically, can't be overwritten
  - you can find a full [list of commonly used claims at iana.org](https://www.iana.org/assignments/jwt/jwt.xhtml)

### Example

This example illustrates how a JWT Signing Key will be used from a `.env` file implicitly.
Given an `.env` file like this:

```plain
# .env
jwt_signing_key=123Secret123
```

The usage of `jwt` would look like this: 
```sh
curlz r -H 'Authorization: Bearer {{ jwt({"email": "john@dow.com", "sub": "some subject"}) }}' https://httpbin.org/headers
```

Alternatively, you can provide the claims as named arguments like:
```sh
curlz r -H 'Authorization: Bearer {{ jwt(email="john@dow.com", sub="some subject") }}' https://httpbin.org/headers

{
  "headers": {
    "Accept": "*/*",
    "Authorization": "Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjE2Nzk1MjQ2ODAsInN1YiI6InNvbWUgc3ViamVjdCIsImVtYWlsIjoiam9obkBkb3cuY29tIiwiaWF0IjoxNjc5NTIzNzgwfQ.bt6taB1YGyMc_43mZeq77dgS_teyglUWtr1dsObyXTg",
    "Host": "httpbin.org",
    "User-Agent": "curl/7.86.0",
    "X-Amzn-Trace-Id": "Root=1-641b7fd7-65e24681239a404d6a143187"
  }
}
```

## Basic Auth Token - `basic(username: string, password: string)`

- arguments:
  - `username`: the username as string
  - `password`: the password as string
- output: string is a base64 encoded credential `username:password`

### Example

send a basic auth header with `username` `joe` and `password` `secret`:
```sh
curlz r -H 'Authorization: Basic {{ basic("joe", "secret") }}' https://httpbin.org/headers

{
  "headers": {
    "Accept": "*/*",
    "Authorization": "Basic am9lOnNlY3JldA==",
    "Host": "httpbin.org",
    "User-Agent": "curl/7.86.0",
    "X-Amzn-Trace-Id": "Root=1-641b7fc0-1228f6242d06665a515e5cf9"
  }
}
```

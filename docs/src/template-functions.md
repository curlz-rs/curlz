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
    curlz r https://api.github.com/user -- -u '{{ prompt_for("GitHub Username") }}:{{ prompt_password() }}'
    ```

## Prompt User for Password - `prompt_password()`
- arguments: None
- output: string
- examples:
  - let the user enter an arbitrary username
    ```shell
    curlz r https://api.github.com/user -- -u '{{ prompt_for("GitHub Username") }}:{{ prompt_password() }}'
    ```

## Json Web Token - `jwt(claims: map, [signing_key: string])`
- arguments:
    - `claims`: to be a map of key value pairs like `{"uid": "1234"}` that are the payload of the JWT
    - `signing_key`: to be a string, this is optional and can be provided at the environment file with a variable named `jwt_signing_key`
- output: string is a Json Web Token (JWT)
- notes:
    - the hash algorithm is `HS256` and the JWT header is `{"alg": "HS256", "typ": "JWT"}`
    - the claim `exp` expiry time is set to in 15min by default, but can be overwritten
    - the claim `iat` issued at timestamp is set automatically and can't be overwritten

## Basic Auth Token - `basic(username: string, password: string)`
- arguments:
    - `username`: the username as string
    - `password`: the password as string
- output: string is a base64 encoded credential `username:password`
- examples:
  - send a basic auth header
  ```sh
  curlz r -H 'Authorization: Basic {{ basic("joe", "secret") }}' https://httpbin.org/headers 
  ```
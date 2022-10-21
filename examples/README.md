# Examples

Note: for testing you can `cargo install echo-server` and run `echo-server` 

- a request with a JWT authorization header, that has custom claims 
  ```sh
  curlz r 'http://localhost:8080/' -- -vvv --header 'authorization: Baerer {{ jwt({"foo": "bar"}) }}'
  ```
# Rust HTTP Server

An implementation of a server from scratch with Rust. For now, it only accepts HTTP 1.1, receiving requests and responding with the method of the request, the code status of the response and the body returned.
## How to use
On `main.rs` you can define the variable '*port*' to determine where the server will be listening.

By default, it's going to run on `PORT 3000`.

To run the server, simply run with cargo on the project's folder.

``` bash
  cargo run
```
## Project's scope
This project was used to understand the Rust language and learn about low level programming languages.

In the future, will be implemented the ability to handle request and response `HEADERS` and expand the HTTP Protocols accepted.

## Useful Links
* [Rust Book](https://doc.rust-lang.org/book/title-page.html)
* [Rust Reference](https://doc.rust-lang.org/reference/introduction.html)
* [MDN on HTTP](https://developer.mozilla.org/en-US/docs/Web/HTTP)
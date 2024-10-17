# axum_jsonapi
Minimal implementation of a scaffolding for a JSON:API v1.1-compliant Axum server.

## Description

This project is a quick starting point for anyone who wants to implement an HTTPS REST API service that follows the [JSON:API v1.1](jsonapi.org) standards. It is written in Rust, using the popular Axum framework.

This project, despite being minimal, contains TLS, logging, URL parameter extraction, and error handling examples.

## Pre-requisites

Aside from Rust, this project depends on the axum-server, which requires installing [NASM](https://nasm.us/) and [CMake](cmake.org).

It also requires setting up a TLS certificate. To create the required files, you may use [OpenSSL](openssl.org). The following interactive command will create the two required files (*key.pem* and *cert.pem*):

    openssl req -newkey rsa:2048 -nodes -keyout key.pem -x509 -days 365 -out cert.pem

The server expects these two files inside the root folder. You can change this under `config` in `main()`.

## Code
Inside your endpoint, create an `Output::new()` object. You can `add` the data that you want to send, e.g. with a struct, and then call `respond()`. If there are any errors, you can `add_error` and set some information like title, detail, and source. You can keep adding more errors and end with a call to `respond()` as well.

The `routes.rs` contains examples of three endpoints:

* `/fib?n=<n>`: Non-blocking, async calculation of the `n`-th fibonacci number. You can call it with e.g. n=40 and while it is running, call it again from a separate source with n=10. The n=10 should be almost instantaneous, showing the result before n=40 finishes.
Any value higher than n=40 will result in an error.

   **NOTE: The calls are limited to n=40 for a reason. A higher number will take a long time and may eventually overflow the call stack.** 
* `/errors`: Just an example of how to quickly create several errors and return the resulting list.

## Examples

Run the server and navigate to https://localhost:3443/fib?n=20. If your certificate is installed correctly, you should shortly see the response:

    {
        "data": {
            "n": 20,
            "result": 6765
        }
    }

Go instead to https://localhost:3443/fib?n=200 and you will receive this error:

    {
        "errors": [
            {
                "status": 406,
                "source": "/fib",
                "title": "Not Acceptable",
                "detail": "Number is too high: 200"
            }
        ]
    }

If you navigate to https://localhost:3443/errors, you will get a list of two JSON:API errors:

    {
        "errors": [
            {
                "status": 503,
                "source": "/errors",
                "title": "Service Unavailable",
                "detail": "First error"
            },
            {
                "status": 418,
                "title": "I'm a teapot",
                "detail": "Second, teapot error"
            }
        ]
    }
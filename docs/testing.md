---
title: Testing
---

import CodeBlock from "@site/src/components/code_block.js";

# Testing

Every application should be well tested. Ntex provides tools to perform unit and integration tests.

## Unit Tests

For unit testing, ntex provides a request builder type. [_TestRequest_][testrequest] implements a builder-like pattern. You can generate a `HttpRequest` instance with `to_http_request()` and call your handler with it.

<CodeBlock example="testing" file="main.rs" section="unit-tests" />

## Integration tests

There are a few methods for testing your application. Ntex can be used to run the application with specific handlers in a real HTTP server.

`TestRequest::get()`, `TestRequest::post()` and other methods can be used to send requests to the test server.

To create a `Service` for testing, use the `test::init_service` method which accepts a regular `App` builder.

> Check the [API documentation][ntexdocs] for more information.

<CodeBlock example="testing" file="integration_one.rs" section="integration-one" />

If you need more complex application configuration, testing should be very similar to creating the normal application. For example, you may need to initialize application state. Create an `App` with a `data` method and attach state just like you would from a normal application.

<CodeBlock example="testing" file="integration_two.rs" section="integration-two" />

## Stream response tests

If you need to test stream generation, it would be enough to call [`into_parts()`][resintoparts] and convert the resulting body into a future and execute it, for example when testing [_Server Sent Events_][serversentevents].

<CodeBlock example="testing" file="stream_response.rs" section="stream-response" />

[serversentevents]: https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events
[resintoparts]: https://docs.rs/ntex/latest/ntex/web/struct.WebResponse.html#method.into_parts
[ntexdocs]: https://docs.rs/ntex/latest/ntex/web/test/index.html
[testrequest]: https://docs.rs/ntex/latest/ntex/web/test/struct.TestRequest.html

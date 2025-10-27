# Headers and Status

One core aspect of server functions and server-side-rendering is customizing responses to user requests.

This could be for many different reasons:

- Return structured status codes (200, 404, 500, etc)
- Add cookies to the response for authentication
- Set custom headers of the response (Content-Type, Caching, Range, etc)

With server functions, this might be

## Response Headers with `SetHeader`

todo... waiting for changes in dioxus itself...

`SetHeader<T>` can set a specific header
`Redirect` can set the redirect header


## Setting Client-Side Headers

todo... waiting for changes in dioxus itself...

Currently not possible, sadly.

## Setting Server Headers

todo... waiting for changes in dioxus itself...

`FullstackContext::response_headers`

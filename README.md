### Design
1. User submits code
2. The code is added to a compilation queue
3. The server compiles the code one at a time with the same cached dependencies. (dioxus)
4. The server stores the compiled wasm and provides a temporary url to the page.
5. The server deletes the wasm after one-time use or if the user disconnects (websocket)
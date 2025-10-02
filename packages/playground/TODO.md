# Bugs
- [x] Spamming rebuild breaks it - this is just rate limiting
- [x] Loading the built page is very very slow
- [x] Hot reloading only works after the first rebuild
- [x] Build errors don't show up
- [x] Switch to storage based clear instead of timeout
- [x] Investigate ws rate limiting. Can you send multiple build requests from the same websocket?
- [x] When the build changes stuff, it sometimes never sends the finished build message
- [x] Limit memory of dx

# Features
- [ ] Get the component library working
- [x] Hot patching

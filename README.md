# monkeylang
This is a Rust implementation of Thorsten Ball's Monkey language with some extensions such as support for floating point numbers, rational numbers and interpolated strings. 
Currently the project is in a very early stage. Most of my work has been on the lexer. So far I have diverged quite a bit from the implementation outlined in Thorsten's book. Instead of just following his go code and translating it to Rust
I have decide to draw from rustc's lexer for insight. Thus, this implementationfollow some of rustc's idioms, albeit in a very, very simplified way. 

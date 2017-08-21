# SOLID OpenGL uniform handling in Rust

Any OpenGL shader program has uniforms, and they need to be set through the OpenGL API in our program. Instead of making hardcoded API calls which are indirectly tightly coupled to our shader program, it would be nice if we could build a high level abstraction that is reusable for any shader program.
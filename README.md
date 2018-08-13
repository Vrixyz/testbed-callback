# testbed-callback
Project to illustrate callback with data with nphysics testbed

Objective is to help with rapid prototyping with nphysics.

I'm discovering Rust, nphysics and specs.rs, so brace yourselves for beginner mistakes.

## What's my status
I have a side project aiming to become a 2d game server, featuring nphysics and specs.rs

For debugging the server, I have different run options :
- lightweight, only command line, fast and efficient
- graphical: the goal is to have a quick rendering of the server 2d state. I'd like to use nphysics testbed for this mode.

### What's my problem
 My systems need to have access to physics world to add entities to it (for a spawner per exemple)
 
 ## Conclusion
 This project is somewhat minimal codebase to reproduce my problem and provide example which might help others.
 
 If any evolution is needed from testbed to support mutability from another source, I'd be glad to help.

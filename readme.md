# determination

a portable and deterministic 2d physics engine

## why another physics engine

While most physics engines are deterministic in theory, [in practice they are not](https://randomascii.wordpress.com/2013/07/16/floating-point-determinism/). But why determinism anyway? Well, it is needed for [deterministic lockstep](https://gafferongames.com/post/deterministic_lockstep/) and [rollback engines like ggpo](https://en.wikipedia.org/wiki/GGPO), both scalable networking models for games that are on the rise again.

## goals

* Determinism across platforms **as long as the api calls are made in the same order**
* Cheap snapshots and restore of the physics world state, for rollbacks or server side interpolation

## why rust

Besides its memory safety, rust is portable to many platforms and operating systems while having no runtime and no garbage collector. The idea is to have this engine run on the web through wasm, inside engines like unity3d as a native plugin, or just as a static or dynamic library.

## contributing

This is an enormous Task, any help is very much appreciated. Best drop me a message on [twitter](https://twitter.com/vengarioth) or open an [issue](https://github.com/Vengarioth/determination/issues) first.

## licence

MIT, if you need this project under a different licence, please open an [issue](https://github.com/Vengarioth/determination/issues) describing your reasons so i may consider adding it.

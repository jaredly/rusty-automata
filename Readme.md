# Rusty Automata

This is an experiment in rust & cellular automata. [video](http://www.youtube.com/watch?v=lNFOnomruqk)

![screenshot](screenshot-2.png)

## Requirements
- libSDL (`yum install SDL-devel`, `apt-get install libsdl-dev`)
- the latest unstable version of rust (10-pre), from github's HEAD

## To build & run
```
make rust-sdl
make
./simulate
```

## Keyboard shortcuts
- d: cycle demos
- p: pause/play
- s: step (when paused)
- t: change theme (dark/light)
- c: change mouse color (the color created when drawing with the mouse)

## Supported Environments
Let's be real, this is a toy experiment; "support" isn't really part of the picture.

Nevertheless, I've compiled this just fine on Ubuntu 13.10 x86, and Fedora 19 x64.

Let me know if you have questions/comments in the github issues.

## License
Apache version 2

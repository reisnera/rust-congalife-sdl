# rust-congalife-sdl

A really bad client and viewer for [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) in [Rust](https://www.rust-lang.org).

Depends on my equally bad crate, [rust-congalife](https://github.com/apostrophest/rust-congalife).

Although most of the dependencies are managed by Cargo, you will also need to [install SDL2.0 development libraries for your platform](https://github.com/AngryLawyer/rust-sdl2#sdl20--development-libraries).

Additionally, the Cargofile for this project assumes that you have checked out [rust-congalife](https://github.com/apostrophest/rust-congalife) as a sibling folder to this project's folder.

For example,
```
$ cd /Users/scthompson/projects
$ git clone https://github.com/apostrophest/rust-congalife.git
$ git clone https://github.com/apostrophest/rust-congalife-sdl.git
$ ls
  .
  ..
  rust-congalife/
  rust-congalife-sdl/
```

Thanks to @AngryLawyer for the Rust bindings to SDL2!

### Running
```
$ cargo run --release
```

### License
MIT

### Known to work with
- Rust 1.14.0-nightly

build:
	@rustc -L../rust-sdl simulate.rs

tuned:
	@rustc -L../rust-sdl simulate.rs --opt-level=3 -o tuned


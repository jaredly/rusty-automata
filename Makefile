
RUST = $(wildcard *.rs)

build: ${RUST}
	@rustc -L../rust-sdl simulate.rs

tuned: ${RUST}
	@rustc -L../rust-sdl simulate.rs --opt-level=3 -o tuned


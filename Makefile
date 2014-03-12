
RUST = $(wildcard *.rs)

build: ${RUST} rust-sdl
	@rustc -L./rust-sdl_ttf -L./rust-sdl simulate.rs

tuned: ${RUST} rust-sdl
	@rustc -L./rust-sdl_ttf -L./rust-sdl simulate.rs --opt-level=3 -o tuned

rust-sdl:
	@cd rust-sdl && rustc src/sdl/lib.rs

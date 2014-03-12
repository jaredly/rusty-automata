
RUST = $(wildcard *.rs)

build: ${RUST}
	@rustc -L./rust-sdl_ttf -L./rust-sdl simulate.rs

tuned: ${RUST}
	@rustc -L./rust-sdl_ttf -L./rust-sdl simulate.rs --opt-level=3 -o tuned

sdl: rust-sdl
	@cd rust-sdl && rustc src/sdl/lib.rs

rust-sdl:
	@git clone https://github.com/brson/rust-sdl

.PHONY: rust-sdl sdl

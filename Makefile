
build: ${RUST}
	@rustc -L./rust-sdl_ttf -L./rust-sdl simulate.rs

tuned: ${RUST}
	@rustc -L./rust-sdl_ttf -L./rust-sdl simulate.rs --opt-level=3 -o tuned

rust-sdl:
	@cd rust-sdl && rustc src/sdl/lib.rs

.PHONY: rust-sdl

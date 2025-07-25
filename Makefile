# PRODUCTION MAKEFILE

rectangles:
	@cargo build --release

install: rectangles
	@if [ "$(shell whoami)" != "root" ]; then \
		echo "You must be root to install this package!"; \
		exit 1; \
	fi
	@install -Dm755 target/release/rectangles /usr/local/bin/rectangles
	@echo "Installed to /usr/local/bin/rectangles"

clean:
	@cargo clean


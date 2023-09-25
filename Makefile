prog :=rde1
server :=srde
client :=crde
cargo := $(shell command -v cargo 2> /dev/null)
cargo_v := $(shell cargo -V| cut -d ' ' -f 2)
rustup := $(shell command -v rustup 2> /dev/null)

check_cargo:
  ifndef cargo
    $(error cargo is not available, please install it! curl https://sh.rustup.rs -sSf | sh)
  else
	@echo "Make sure your cargo version is up to date! Current version is $(cargo_v)"
  endif

check_rustup:
  ifndef rustup
    $(error rustup is not available, please install it! curl https://sh.rustup.rs -sSf | sh)
  endif

# Deps install

install_windows_deps: update_rustup
	@rustup install stable-x86_64-pc-windows-gnu --force-non-host
	@rustup target add x86_64-pc-windows-gnu
	@rustup install stable-i686-pc-windows-gnu --force-non-host
	@rustup target add i686-pc-windows-gnu

install_macos_deps:
	@sudo git clone https://github.com/tpoechtrager/osxcross /usr/local/bin/osxcross || exit
	@sudo wget -P /usr/local/bin/osxcross/ -nc https://s3.dockerproject.org/darwin/v2/MacOSX10.10.sdk.tar.xz && sudo mv /usr/local/bin/osxcross/MacOSX10.10.sdk.tar.xz /usr/local/bin/osxcross/tarballs/
	@sudo UNATTENDED=yes OSX_VERSION_MIN=10.7 /usr/local/bin/osxcross/build.sh
	@sudo chmod 775 /usr/local/bin/osxcross/ -R
	@export PATH="/usr/local/bin/osxcross/target/bin:$PATH"
	@grep 'target.x86_64-apple-darwin' ~/.cargo/config || echo "[target.x86_64-apple-darwin]" >> ~/.cargo/config
	@grep 'linker = "x86_64-apple-darwin14-clang"' ~/.cargo/config || echo 'linker = "x86_64-apple-darwin14-clang"' >> ~/.cargo/config
	@grep 'ar = "x86_64-apple-darwin14-clang"' ~/.cargo/config || echo 'ar = "x86_64-apple-darwin14-clang"' >> ~/.cargo/config

install_linux_deps:update_rustup
	@rustup install stable-x86_64-unknown-linux-gnu --force-non-host
	@rustup target add x86_64-unknown-linux-gnu

install_cross:
	@cargo install --version 0.1.16 cross

update_rustup:
	rustup update

# Cleaning

clean:
	sudo rm -rf server/target client/target

# SRDE server

srde_release: check_cargo
	cargo build --release --manifest-path server/Cargo.toml
	cp server/target/release/$(server) ./$(server)_release
	@echo -e "[+] You can find \033[1;32m$(server)_release\033[0m release version in your current folder."

srde_debug: check_cargo
	cargo build --manifest-path server/Cargo.toml
	cp server/target/debug/$(server) ./$(server)_debug
	@echo -e "[+] You can find \033[1;32m$(server)_debug\033[0m debug version in your current folder."

srde_doc: check_cargo
	cargo doc --open --no-deps --manifest-path server/Cargo.toml

srde_build_windows_x64:
	RUSTFLAGS="-C target-feature=+crt-static" cargo build --release --target x86_64-pc-windows-gnu --manifest-path  server/Cargo.toml
	cp server/target/x86_64-pc-windows-gnu/release/$(server).exe ./$(server)_x64.exe
	@echo -e "[+] You can find \033[1;32m$(server)_x64.exe\033[0m in your current folder."

srde_build_windows_x86:
	RUSTFLAGS="-C target-feature=+crt-static" cargo build --release --target i686-pc-windows-gnu --manifest-path server/Cargo.toml
	cp server/target/i686-pc-windows-gnu/release/$(server).exe ./$(server)_x86.exe
	@echo -e "[+] You can find \033[1;32m$(server)_x86.exe\033[0m in your current folder."

srde_windows: check_rustup install_windows_deps srde_build_windows_x64

srde_windows_x64: check_rustup install_windows_deps srde_build_windows_x64

srde_windows_x86: check_rustup install_windows_deps srde_build_windows_x86

srde_build_linux_aarch64:
	cross build --target aarch64-unknown-linux-gnu --release --manifest-path server/Cargo.toml
	cp server/target/aarch64-unknown-linux-gnu/release/$(server) ./$(server)_aarch64
	@echo -e "[+] You can find \033[1;32m$(server)_aarch64\033[0m in your current folder."

srde_linux_aarch64: check_rustup install_cross srde_build_linux_aarch64

srde_build_linux_x86_64:
	RUSTFLAGS="-C target-feature=+crt-static" cargo build --release --target x86_64-unknown-linux-gnu --manifest-path server/Cargo.toml
	cp server/target/x86_64-unknown-linux-gnu/release/$(server) ./$(server)_x86_64
	@echo -e "[+] You can find \033[1;32m$(server)_x86_64\033[0m in your current folder."

srde_linux_x86_64: check_rustup install_linux_deps srde_build_linux_x86_64

srde_linux: check_rustup install_linux_deps srde_build_linux_x86_64

srde_build_macos:
	@export PATH="/usr/local/bin/osxcross/target/bin:$PATH"
	RUSTFLAGS="-C target-feature=+crt-static" cargo build --release --target x86_64-apple-darwin --manifest-path server/Cargo.toml
	cp server/target/x86_64-apple-darwin/release/$(server) ./$(server)_macOS
	@echo -e "[+] You can find \033[1;32m$(server)_macOS\033[0m in your current folder."

srde_macos: srde_build_macos

srde_arm_musl: check_rustup install_cross
	cross build --target arm-unknown-linux-musleabi --release
	cp server/target/arm-unknown-linux-musleabi/release/$(server) ./$(server)_arm_musl
	@echo -e "[+] You can find \033[1;32m$(server)_arm_musl\033[0m in your current folder."

srde_armv7: check_rustup install_cross
	cross build --target armv7-unknown-linux-gnueabihf --release
	cp server/target/armv7-unknown-linux-gnueabihf/release/$(server) ./$(server)_armv7
	@echo -e "[+] You can find \033[1;32m$(server)_armv7\033[0m in your current folder."

# CRDE client:

crde_release: check_cargo
	cargo build --release --manifest-path client/Cargo.toml
	cp client/target/release/$(client) ./$(client)_release
	@echo -e "[+] You can find \033[1;32m$(client)_release\033[0m release version in your current folder."

crde_debug: check_cargo
	cargo build --manifest-path client/Cargo.toml
	cp client/target/debug/$(client) ./$(client)_debug
	@echo -e "[+] You can find \033[1;32m$(client)_debug\033[0m debug version in your current folder."

crde_doc: check_cargo
	cargo doc --open --no-deps --manifest-path client/Cargo.toml

crde_build_windows_x64:
	RUSTFLAGS="-C target-feature=+crt-static" cargo build --release --target x86_64-pc-windows-gnu --manifest-path  client/Cargo.toml
	cp client/target/x86_64-pc-windows-gnu/release/$(client).exe ./$(client)_x64.exe
	@echo -e "[+] You can find \033[1;32m$(client)_x64.exe\033[0m in your current folder."

crde_build_windows_x86:
	RUSTFLAGS="-C target-feature=+crt-static" cargo build --release --target i686-pc-windows-gnu --manifest-path client/Cargo.toml
	cp client/target/i686-pc-windows-gnu/release/$(client).exe ./$(client)_x86.exe
	@echo -e "[+] You can find \033[1;32m$(client)_x86.exe\033[0m in your current folder."

crde_windows: check_rustup install_windows_deps crde_build_windows_x64

crde_windows_x64: check_rustup install_windows_deps crde_build_windows_x64

crde_windows_x86: check_rustup install_windows_deps crde_build_windows_x86

crde_build_linux_aarch64:
	cross build --target aarch64-unknown-linux-gnu --release --manifest-path client/Cargo.toml
	cp client/target/aarch64-unknown-linux-gnu/release/$(client) ./$(client)_aarch64
	@echo -e "[+] You can find \033[1;32m$(client)_aarch64\033[0m in your current folder."

crde_linux_aarch64: check_rustup install_cross crde_build_linux_aarch64

crde_build_linux_x86_64:
	RUSTFLAGS="-C target-feature=+crt-static" cargo build --release --target x86_64-unknown-linux-gnu --manifest-path client/Cargo.toml
	cp client/target/x86_64-unknown-linux-gnu/release/$(client) ./$(client)_x86_64
	@echo -e "[+] You can find \033[1;32m$(client)_x86_64\033[0m in your current folder."

crde_linux_x86_64: check_rustup install_linux_deps crde_build_linux_x86_64

crde_linux: check_rustup install_linux_deps crde_build_linux_x86_64

crde_build_macos:
	@export PATH="/usr/local/bin/osxcross/target/bin:$PATH"
	RUSTFLAGS="-C target-feature=+crt-static" cargo build --release --target x86_64-apple-darwin --manifest-path client/Cargo.toml
	cp client/target/x86_64-apple-darwin/release/$(client) ./$(client)_macOS
	@echo -e "[+] You can find \033[1;32m$(client)_macOS\033[0m in your current folder."

crde_macos: crde_build_macos

crde_arm_musl: check_rustup install_cross
	cross build --target arm-unknown-linux-musleabi --release
	cp client/target/arm-unknown-linux-musleabi/release/$(client) ./$(client)_arm_musl
	@echo -e "[+] You can find \033[1;32m$(client)_arm_musl\033[0m in your current folder."

crde_armv7: check_rustup install_cross
	cross build --target armv7-unknown-linux-gnueabihf --release
	cp client/target/armv7-unknown-linux-gnueabihf/release/$(client) ./$(client)_armv7
	@echo -e "[+] You can find \033[1;32m$(client)_armv7\033[0m in your current folder."

# Keys

export LITCRYPT_ENCRYPT_KEY:=$(shell echo `tr -dc A-Za-z0-9 < /dev/urandom | head -c 64`)

# Makefile help

help:
	@echo ""
	@echo "SRDE server:"
	@echo "usage: make srde_debug"
	@echo "usage: make srde_release"
	@echo "usage: make srde_windows"
	@echo "usage: make srde_windows_x64"
	@echo "usage: make srde_windows_x86"
	@echo "usage: make srde_linux"
	@echo "usage: make srde_linux_aarch64"
	@echo "usage: make srde_linux_x86_64"
	@echo "usage: make srde_macos"
	@echo "usage: make srde_arm_musl"
	@echo "usage: make srde_armv7"
	@echo ""
	@echo "CRDE client:"
	@echo "usage: make crde_debug"
	@echo "usage: make crde_release"
	@echo "usage: make crde_windows"
	@echo "usage: make crde_windows_x64"
	@echo "usage: make crde_windows_x86"$
	@echo "usage: make crde_linux"
	@echo "usage: make crde_linux_aarch64"
	@echo "usage: make crde_linux_x86_64"
	@echo "usage: make crde_macos"
	@echo "usage: make crde_arm_musl"
	@echo "usage: make crde_armv7"
	@echo ""
	@echo "Dependencies:"
	@echo "usage: make install_windows_deps"
	@echo "usage: make install_macos_deps"
	@echo ""
	@echo "Documentation:"
	@echo "usage: make srde_doc"
	@echo "usage: make crde_doc"
	@echo ""
	@echo "Cleaning:"
	@echo "usage: make clean"
	@echo ""
#!/bin/bash

echo 'Installing ARM cross-compilers...'
rustup target add aarch64-unknown-linux-gnu
sudo apt install gcc-aarch64-linux-gnu -y
echo 'Done'
echo 'Open or create the file <project-root>/.cargo/config.toml and add the following configurations if needed:'
echo '--------------------------------------'
echo '[target.armv7-unknown-linux-gnueabihf]'
echo 'linker = "arm-linux-gnueabihf-gcc"    '
echo '                                      '
echo '[target.aarch64-unknown-linux-gnu]    '
echo 'linker = "aarch64-linux-gnu-gcc"      '
echo '--------------------------------------'
read -n 1 -s -r -p "Press any key to continue"
echo 'Installing ARM64 cross-compilers...'
sudo dpkg --add-architecture arm64
echo 'You need to add the following line to /etc/apt/sources.list:'
echo '---------------------------------------------------------------------------------------------------------------'
echo 'deb [arch=armhf,arm64] http://ports.ubuntu.com/ubuntu-ports jammy main restricted                              '
echo 'deb [arch=armhf,arm64] http://ports.ubuntu.com/ubuntu-ports jammy-updates main restricted                      '
echo 'deb [arch=armhf,arm64] http://ports.ubuntu.com/ubuntu-ports jammy universe                                     '
echo 'deb [arch=armhf,arm64] http://ports.ubuntu.com/ubuntu-ports jammy-updates universe                             '
echo 'deb [arch=armhf,arm64] http://ports.ubuntu.com/ubuntu-ports jammy multiverse                                   '
echo 'deb [arch=armhf,arm64] http://ports.ubuntu.com/ubuntu-ports jammy-updates multiverse                           '
echo 'deb [arch=armhf,arm64] http://ports.ubuntu.com/ubuntu-ports jammy-backports main restricted universe multiverse'
echo 'deb [arch=armhf,arm64] http://ports.ubuntu.com/ubuntu-ports jammy-security main restricted                     '
echo 'deb [arch=armhf,arm64] http://ports.ubuntu.com/ubuntu-ports jammy-security universe                            '
echo 'deb [arch=armhf,arm64] http://ports.ubuntu.com/ubuntu-ports jammy-security multiverse                          '
echo '---------------------------------------------------------------------------------------------------------------'
read -n 1 -s -r -p "Press any key to continue"
echo 'Updating apt...'
sudo apt-get update && sudo apt-get upgrade -y
echo 'Installing required webkit dependencies...'
sudo apt install libwebkit2gtk-4.0-dev:arm64
echo 'Building the app for ARM64...'
export PKG_CONFIG_SYSROOT_DIR=/usr/aarch64-linux-gnu/
cd src-tauri && cargo tauri build --target aarch64-unknown-linux-gnu
#! /bin/bash

set -e

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )/" && pwd )"

export PREFIX="$SCRIPT_DIR/tools"
export TARGET=x86_64-elf
export BINUTILS_VERSION=2.29.1
export GCC_VERSION=7.2.0
export GRUB_VERSION=2.02
export PATH="$PREFIX/bin:$PATH"

silent() {
    while read line; do
        printf '.'
    done
    printf '\n'
}

run() {
    if [ "$CI" == "true" ]; then
        "$@" | silent
    else
        "$@"
    fi
}

download_tar() {
    if [ ! -f $1 ]; then
        echo "::: Downloading source"
        wget -c $2
    fi

    echo "::: Extracting source"
    tar -xf $1
}

download_git() {
    if [ ! -d $1 ]; then
        echo "::: Cloning source"
        git clone $2 $1
    fi
}

install_binutils() {
    echo "===== Binutils ====="
    download_tar binutils-$BINUTILS_VERSION.tar.gz ftp://ftp.gnu.org/gnu/binutils/binutils-$BINUTILS_VERSION.tar.gz

    mkdir -p build-binutils; pushd build-binutils

    echo "::: Configuring"
    run ../binutils-$BINUTILS_VERSION/configure --target=$TARGET --prefix="$PREFIX" \
        --disable-nls --disable-werror \
        --disable-gdb --disable-libdecnumber --disable-readline --disable-sim

    echo "::: Building"
    run make
    run make install

    popd

    echo "::: Done!"
}

install_gcc() {
    echo "===== GCC ====="
    download_tar gcc-$GCC_VERSION.tar.gz ftp://ftp.gnu.org/gnu/gcc/gcc-$GCC_VERSION/gcc-$GCC_VERSION.tar.gz

    echo "::: Downloading prerequisites"
    pushd gcc-$GCC_VERSION
    run ./contrib/download_prerequisites
    popd

    mkdir -p build-gcc; pushd build-gcc

    echo "::: Configuring"
    run ../gcc-$GCC_VERSION/configure --target=$TARGET --prefix="$PREFIX" \
        --disable-nls --enable-languages=c,c++

    echo "::: Building"
    run make all-gcc
    run make all-target-libgcc
    run make install-gcc
    run make install-target-libgcc

    popd

    echo "::: Done!"
}

install_grub() {
    echo "===== GRUB ====="
    download_tar grub-$GRUB_VERSION.tar.xz ftp://ftp.gnu.org/gnu/grub/grub-$GRUB_VERSION.tar.xz

    pushd grub

    echo "::: Configuring GRUB"
    run ./autogen.sh
    run ./configure --disable-werror TARGET_CC=$TARGET-gcc TARGET_OBJCOPY=$TARGET-objcopy \
            TARGET_STRIP=$TARGET-strip TARGET_NM=$TARGET-nm TARGET_RANLIB=$TARGET-ranlib \
            --target=$TARGET --prefix="$PREFIX"

    echo "::: Building GRUB"
    run make
    run make install

    popd
    echo "::: Done!"
}

install() {
    if [ ! -f $PREFIX/bin/$TARGET-ld ]; then
        install_binutils
    fi

    if [ ! -f $PREFIX/bin/$TARGET-gcc ]; then
        install_gcc
    fi

    if [ ! -f $PREFIX/bin/grub-mkrescue ]; then
        install_grub
    fi
}

mkdir -p $PREFIX/src; cd $PREFIX/src

install

if [ "$CI" = true ]; then
    echo "===== Cleanup ====="
    rm -rf "$PREFIX/src"
fi

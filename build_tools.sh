#! /bin/bash

set -e

silent() {
    while read line; do
        printf '.'
    done
    printf '\n'
}

run() {
    if [ $CI = true ]; then
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
    download_git objconv https://github.com/vertis/objconv.git
    download_git grub git://git.savannah.gnu.org/grub.git

    echo "::: Building Objconv"
    # Build objconv
    pushd objconv
    g++ -o objconv -O2 src/*.cpp
    cp objconv $PREFIX/bin
    popd

    pushd grub

    echo "::: Configuring GRUB"
    run ./autogen.sh
    run ../grub/configure --disable-werror TARGET_CC=$TARGET-gcc TARGET_OBJCOPY=$TARGET-objcopy \
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

cleanup() {
    rm -rf "$PREFIX/src"
}

main() {
    SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

    export PREFIX="$SCRIPT_DIR/tools"
    export TARGET=x86_64-elf
    export BINUTILS_VERSION=2.27
    export GCC_VERSION=6.3.0
    export PATH="$PREFIX/bin:$PATH"

    mkdir -p $PREFIX/src; cd $PREFIX/src

    install

    if [ "$CI" = true ]; then
        echo "===== Cleanup ====="
        cleanup
    fi
}

main

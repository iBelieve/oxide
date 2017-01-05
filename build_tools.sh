#! /bin/sh

set -e

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

export PREFIX="$SCRIPT_DIR/tools"
export TARGET=x86_64-elf
export BINUTILS_VERSION=2.27
export GCC_VERSION=6.3.0
export PATH="$PREFIX/bin:$PATH"

mkdir -p $PREFIX/src; cd $PREFIX/src

echo $PREFIX $TARGET $PATH

if [ ! -f $PREFIX/bin/$TARGET-ld ]; then
    # Fetch binutils
    wget -c ftp://ftp.gnu.org/gnu/binutils/binutils-$BINUTILS_VERSION.tar.gz
    tar -xvf binutils-$BINUTILS_VERSION.tar.gz

    # Build binutils
    mkdir build-binutils; pushd build-binutils
    ../binutils-$BINUTILS_VERSION/configure --target=$TARGET --prefix="$PREFIX" \
        --disable-nls --disable-werror \
        --disable-gdb --disable-libdecnumber --disable-readline --disable-sim
    make
    make install
    popd
fi

if [ ! -f $PREFIX/bin/$TARGET-gcc ]; then
    # Fetch GCC and dependencies
    wget -c ftp://ftp.gnu.org/gnu/gcc/gcc-$GCC_VERSION/gcc-$GCC_VERSION.tar.gz
    tar -xvf gcc-$GCC_VERSION.tar.gz
    pushd gcc-$GCC_VERSION
    ./contrib/download_prerequisites
    popd

    # Buld gcc
    mkdir build-gcc; pushd build-gcc
    ../gcc-$GCC_VERSION/configure --target=$TARGET --prefix="$PREFIX" --disable-nls --enable-languages=c,c++
    make all-gcc
    make all-target-libgcc
    make install-gcc
    make install-target-libgcc
    popd
fi

if [ ! -f $PREFIX/bin/grub-mkrescue ]; then
    # Fetch grub & dependencies
    if [ ! -d grub ]; then
        git clone git://git.savannah.gnu.org/grub.git
    fi
    if [ ! -d objconv ]; then
        git clone https://github.com/vertis/objconv.git
    fi

    # Build objconv
    pushd objconv
    g++ -o objconv -O2 src/*.cpp
    cp objconv $PREFIX/bin
    popd

    # Build grub
    pushd grub
    ./autogen.sh
    ../grub/configure --disable-werror TARGET_CC=$TARGET-gcc TARGET_OBJCOPY=$TARGET-objcopy \
            TARGET_STRIP=$TARGET-strip TARGET_NM=$TARGET-nm TARGET_RANLIB=$TARGET-ranlib \
            --target=$TARGET --prefix="$PREFIX"
    make
    make install
    popd
fi

#!/bin/sh

set -e

DIR="$(cd "$(dirname "$0")/../" && pwd)"
FLAGS="-Wall -Wextra -pedantic -I$DIR"
CFLAGS="-std=c99 $FLAGS"

if [ "$(uname)" = "Darwin" ]; then
	CXXFLAGS="-DWEBVIEW_COCOA -std=c++11 $FLAGS -framework WebKit"
else
	CXXFLAGS="-DWEBVIEW_GTK -std=c++11 $FLAGS $(pkg-config --cflags --libs gtk+-3.0 webkit2gtk-4.0)"
fi

if command -v clang-format >/dev/null 2>&1 ; then
	echo "Formatting..."
	clang-format -i \
		"$DIR/webview.h" \
		"$DIR/webview_test.cc" \
		"$DIR/examples/"*.c \
		"$DIR/examples/"*.cc
else
	echo "SKIP: Formatting (clang-format not installed)"
fi

if command -v clang-tidy >/dev/null 2>&1 ; then
	echo "Linting..."
	clang-tidy "$DIR/examples/basic.cc" -- $CXXFLAGS
	clang-tidy "$DIR/examples/bind.cc" -- $CXXFLAGS
	clang-tidy "$DIR/webview_test.cc" -- $CXXFLAGS
else
	echo "SKIP: Linting (clang-tidy not installed)"
fi

mkdir -p build/examples/c build/examples/cc build/examples/go || true

echo "Building C++ examples"
c++ examples/basic.cc $CXXFLAGS -o build/examples/cc/basic
c++ examples/bind.cc $CXXFLAGS -o build/examples/cc/bind

echo "Building C examples"
c++ -c $CXXFLAGS webview.cc -o build/webview.o
cc -c examples/basic.c $CFLAGS -o build/examples/c/basic.o
cc -c examples/bind.c $CFLAGS -o build/examples/c/bind.o
c++ build/examples/c/basic.o build/webview.o $CXXFLAGS -o build/examples/c/basic
c++ build/examples/c/bind.o build/webview.o $CXXFLAGS -o build/examples/c/bind


set -e

if [ "$2" = "" ]; then
  VERSION="debug"
else
  VERSION="$2"
fi

if [ ! -d release ]; then
  mkdir release
fi

clear_build() {
  if [ -d build ]; then
    rm -rf build
  fi
  mkdir build
}

# not work on my machine
release_aarch64_linux() {
  clear_build

  cargo build --release --target aarch64-unknown-linux-gnu

  cp target/release/upstream build/upstream
  cp -r assets build/assets

  cd build

  zip -r upstream-$VERSION-linux-aarch64.zip upstream assets

  cd ..

  mv build/upstream-$VERSION-linux-aarch64.zip release
  ls -alh release/upstream-$VERSION-linux-aarch64.zip
}

release_x86_64_linux() {
  clear_build

  cargo build --release

  cp target/release/upstream build/upstream
  cp -r assets build/assets

  cd build

  zip -r upstream-$VERSION-linux-x86_64.zip upstream assets

  cd ..

  mv build/upstream-$VERSION-linux-x86_64.zip release
  ls -alh release/upstream-$VERSION-linux-x86_64.zip
}

release_x86_64_windows() {
  clear_build

  cargo build --release --target x86_64-pc-windows-gnu

  cp target/x86_64-pc-windows-gnu/release/upstream.exe build/upstream.exe
  cp -r assets build/assets
  cp $(find /usr/x86_64-w64-mingw32/bin/*.dll) build

  cd build

  zip -r upstream-$VERSION-windows-x86_64.zip upstream.exe assets $(find *.dll)

  cd ..

  mv build/upstream-$VERSION-windows-x86_64.zip release
  ls -alh release/upstream-$VERSION-windows-x86_64.zip
}

case "$1" in
  "all")
    release_x86_64_linux
    release_x86_64_windows
    ;;
  "linux")
    release_x86_64_linux
    ;;
  "windows")
    release_x86_64_windows
    ;;
  *)
    echo "Unknown platform \`$1\`, which should be one of the following:"
    echo "all, linux, windows"
    exit 1
    ;;
esac

if [ "$1" = "" ]; then
  echo Please enter release version
  exit 1
else
  echo Start release $1.
fi

if [ -d build ]; then
  rm -rf build/
fi

mkdir build

# build linux version
cargo build --release
cp target/release/upstream build/upstream

# build windows version
cargo build --release --target x86_64-pc-windows-gnu
cp target/x86_64-pc-windows-gnu/release/upstream.exe build/upstream.exe

# copy assets
cp -r assets build/assets/

# copy DLLs
cp $(find /usr/x86_64-w64-mingw32/bin/*.dll) build

cd build

zip -r upstream-$1-linux-x86_64.zip upstream assets
zip -r upstream-$1-windows-x86_64.zip upstream.exe assets $(find *.dll)

rm -rf upstream upstream.exe assets $(find *.dll)

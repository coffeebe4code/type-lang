mkdir -p llvm/build-debug
cd llvm/build-debug
cmake .. \
  -DCMAKE_INSTALL_PREFIX=$HOME/local/llvm16-debug \
  -DCMAKE_PREFIX_PATH=$HOME/local/llvm16-debug \
  -DCMAKE_BUILD_TYPE=Debug \
  -DLLVM_ENABLE_LIBXML2=OFF \
  -DLLVM_ENABLE_TERMINFO=OFF \
  -DLLVM_PARALLEL_LINK_JOBS=1 \
  -G Ninja
ninja install
cd ../..

# LLD
mkdir lld/build-debug
cd lld/build-debug
cmake .. \
  -DCMAKE_INSTALL_PREFIX=$HOME/local/llvm16-debug \
  -DCMAKE_PREFIX_PATH=$HOME/local/llvm16-debug \
  -DCMAKE_BUILD_TYPE=Release \
  -DLLVM_PARALLEL_LINK_JOBS=1 \
  -DCMAKE_CXX_STANDARD=17 \
  -G Ninja
ninja install
cd ../..

# Clang
mkdir clang/build-debug
cd clang/build-debug
cmake .. \
  -DCMAKE_INSTALL_PREFIX=$HOME/local/llvm16-debug \
  -DCMAKE_PREFIX_PATH=$HOME/local/llvm16-debug \
  -DCMAKE_BUILD_TYPE=Release \
  -DLLVM_PARALLEL_LINK_JOBS=1 \
  -G Ninja
ninja install
cd ../..

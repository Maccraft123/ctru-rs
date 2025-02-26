#!/usr/bin/env bash

set -euxo pipefail

bindgen "$DEVKITPRO/libctru/include/3ds.h" \
    --rust-target nightly \
    --use-core \
    --distrust-clang-mangling \
    --must-use-type 'Result' \
    --no-layout-tests \
    --ctypes-prefix "::libc" \
    --no-prepend-enum-name \
    --generate "functions,types,vars" \
    --blocklist-type "u(8|16|32|64)" \
    --blocklist-type "__builtin_va_list" \
    --blocklist-type "__va_list" \
    --opaque-type "MiiData" \
    --with-derive-default \
    -- \
    --target=arm-none-eabi \
    --sysroot=$DEVKITARM/arm-none-eabi \
    -isystem$DEVKITARM/arm-none-eabi/include \
    -I$DEVKITPRO/libctru/include \
    -mfloat-abi=hard \
    -march=armv6k \
    -mtune=mpcore \
    -mfpu=vfp \
    -DARM11 \
    -D__3DS__ \
> src/bindings.rs

cargo run --package docstring-to-rustdoc -- src/bindings.rs

cargo fmt --all

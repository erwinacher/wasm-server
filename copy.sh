#!/bin/bash
set -e

for m in module1 module2 module3 module4; do
  cp wasm-modules/$m/target/wasm32-unknown-unknown/release/$m.wasm \
     wasm-server/static/
done

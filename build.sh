#!/bin/sh

export SMUGGLER_ID="dlinit"
export SMUGGLER_EVENT="click"
export SMUGGLER_FILENAME="totally_legit_file.zip"

wasm-pack build --target web --profile release --out-name "totally_legit"

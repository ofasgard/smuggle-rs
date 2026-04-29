#!/bin/sh

export SMUGGLER_ID="dlinit"
export SMUGGLER_EVENT="click"
export SMUGGLER_FILENAME="totally_legit_file.zip"
export SMUGGLER_PAYLOAD=$(cat demo/payload.zip | base64 -w 0)

wasm-pack build --target web --profile release

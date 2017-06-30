#!/usr/bin/env bash

LIBS=( libruse-read libruse-eval libruse-print libruse ruse )

for lib in "${LIBS[@]}"; do
    cd "${lib}" && cargo test
    cd ..
done


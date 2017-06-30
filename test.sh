#!/usr/bin/env bash

LIBS=( libruse-read libruse-eval libruse-print libruse ruse )
OUTPUT="test-output.log"
ERROR="test-error.log"

rm -f "${OUTPUT}" "${ERROR}"

for lib in "${LIBS[@]}"; do
    cd "${lib}" && cargo test 1>>../"${OUTPUT}" 2>>../"${ERROR}"
    cd ..
done


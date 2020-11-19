#!/bin/bash

cargo run --bin language-test-compiler indev/input.son indev/output.o && cargo run --bin language-test-harness

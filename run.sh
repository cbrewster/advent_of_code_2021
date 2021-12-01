#!/usr/bin/env bash

find . -maxdepth 1 -name 'day*' -type d -printf '%f\n' | fzy | xargs cargo run --bin

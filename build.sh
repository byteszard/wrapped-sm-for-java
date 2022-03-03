#!/usr/bin/env sh

set -e

cargo update -p yarism

cargo build --release
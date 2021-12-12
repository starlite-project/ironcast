#!/bin/bash

set -eu

timeout=${PLAYGROUND_TIMEOUT:-30}

modify-cargo-toml

timeout --signal=KILL ${timeout} "$@"

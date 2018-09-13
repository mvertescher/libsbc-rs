#!/usr/bin/env bash
~/.cargo/bin/bindgen /usr/include/sbc/sbc.h \
  -o src/bindings.rs -- -I/usr/include/sbc \

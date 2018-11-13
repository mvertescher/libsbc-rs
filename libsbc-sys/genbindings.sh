#!/usr/bin/env bash
~/.cargo/bin/bindgen sbc/sbc/sbc.h \
  -o src/bindings.rs -- -I sbc/sbc \

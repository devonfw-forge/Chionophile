#!/bin/sh
cargo run --release -- --log-file goose.log -g -H https://example.com/ \
   --debug-file debug.log --debug-format raw --requests-file requests.log --status-codes -u250 -v > metrics.log

# cargo run --release -- --hatch-rate .016666666
#!/bin/sh
USERNAME=foo PASSWORD=bar cargo run --release -- --log-file goose.log -g -H https://example.com/ \
    --debug-file debug.log --debug-format raw --requests-file requests.log --status-codes -u250 \
    --hatch-rate .016666666 -v > metrics.log
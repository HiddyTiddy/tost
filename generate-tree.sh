#!/bin/sh

cargo run
which dot && dot -Tsvg graph.dot -o foo.svg || (
    echo "dot (https://graphviz.org/) binary not installed." >&2
    exit 1
)

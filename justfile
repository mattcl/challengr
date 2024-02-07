build:
    cargo build && cargo build --release

check DAY:
    scripts/check.sh {{DAY}}

run DAY:
    ./target/debug/aoc2023-input {{DAY}}

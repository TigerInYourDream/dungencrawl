alias r:=run
default: flow

run:
    cargo r -r

build:
    cargo build

flow:
    cargo fmt
    cargo r -r
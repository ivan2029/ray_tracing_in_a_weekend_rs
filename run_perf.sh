#!/usr/bin/env bash

cargo build --release

perf record --call-graph=lbr ./target/release/raytracer_rs



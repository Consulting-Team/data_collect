#!/bin/bash

cd $(dirname $0)

cargo build -r

target/release/dp_to_csv \
-f res/DP_IMO9986104_20250530230013.xml \
-o DP_IMO9986104_20250530230013.csv
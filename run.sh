#!/bin/bash

cd $(dirname $0)

cargo build -r

###### Copy config files
CFG_PATH="target/release/config"
mkdir -p $CFG_PATH && cp log.yaml $CFG_PATH

CFG_PATH="target/debug/config"
mkdir -p $CFG_PATH && cp log.yaml $CFG_PATH


###### DP to csv
# target/release/dp_to_csv \
# -f res/DP_IMO9986104_20250530230013.xml \
# -o DP_IMO9986104_20250530230013.csv

###### concat data
target/release/data_concat \
--imo 9976927 \
--out /home/azure/workspace/HOcean/data_collect/out \
--date 20250601
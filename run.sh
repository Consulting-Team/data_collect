#!/bin/bash

cd $(dirname $0)

cargo build -r

###### Copy config files
CFG_PATH="target/release/config"
mkdir -p $CFG_PATH && cp log.yaml $CFG_PATH


###### DP to csv
# target/release/dp_to_csv \
# -f res/DP_IMO9986104_20250530230013.xml \
# -o DP_IMO9986104_20250530230013.csv

###### concat data
target/release/concat_data \
--imo 9976915 \
--out /home/azure/workspace/HOcean/data_collect/test_out \
--date 20250606
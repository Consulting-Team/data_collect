#!/bin/bash

cd $(dirname $0)

cargo build -r

##### Copy resource files to target directory
for TARGET in "release" "debug"; do
    CFG_PATH="target/$TARGET/config"
    mkdir -p $CFG_PATH && cp log.yaml $CFG_PATH

    cp .env "target/$TARGET"
done


###### DP to csv
# target/release/dp_to_csv \
# -f res/H2559_DP_IMO9986051_20250615093943.xml \
# -o out/H2559_DP_IMO9986051_20250615093943.csv

###### concat data
target/release/data_concat \
--imo 9976927 \
--out /home/azure/workspace/HOcean/data_collect/out \
--date 20250601

# --date 20250616
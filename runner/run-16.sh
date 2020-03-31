#!/bin/bash

export KROUND=30
export KTYPE=all
export KSIZE=30
for th in 16 12
do
        echo Running with KTHREAD=${th}
        KTHREAD=${th} cargo run --release &>log/output-T${th}.log
done;

shutdown -h now

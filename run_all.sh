#!/bin/bash

export KROUND=100
export KTYPE=qs

for th in 24 20 16 12 8 4 2 1 
do
	echo Running with KTHREAD=${th}
	KTHREAD=${th} cargo run --release
done;

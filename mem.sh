#!/usr/bin/env bash

if [ -f results.txt ];
then
  echo "Result file exists, deleting"
  rm results.txt
fi

for nthreads in {1..32}
do
  gcc -fopenmp -D_OPENMP stream.c -o stream;
  echo Running with $nthreads
  printf "\n[%s]\n" "$nthreads" >> results.txt
  printf "$(OMP_NUM_THREADS=${nthreads} ./stream | tail -n7 | head -n4 | awk '{print $2}')\n" >> results.txt
done

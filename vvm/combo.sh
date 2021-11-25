#!/bin/bash

for i in {5..50..5}; do
  for j in {5..50..5}; do
    for k in {5..50..5}; do
      for l in {5..50..5}; do
        cargo -q run ./V2-Official-Test-data.txt $i $j $k $l
      done
    done
  done
done

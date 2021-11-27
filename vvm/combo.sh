#!/bin/bash

function mega_loop() {
  for i in {5..15..5}; do
    for j in {5..15..5}; do
      for k in {5..15..5}; do
        for l in {5..15..5}; do
          [[ $(($i + $j + $k + $l)) -gt 50 ]] && continue
          cargo -q run ./V2-Official-Test-data.txt $i $j $k $l
        done
      done
    done
  done
}

mega_loop | sort | tee loop_output

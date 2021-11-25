#!/bin/bash

for x in {5..50..5}; do
  #echo -n "$x "
  cargo -q run ./V2-Official-Test-data.txt $x $x $x $x
done

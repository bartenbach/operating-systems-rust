#!/bin/bash
DATA_FILES=(~/doc/college/operating-systems/disk_data_100.txt \
  ~/doc/college/operating-systems/OfficalTestFile-disk_data.txt )
ALGORITHMS=('cscan')
QUEUE_SIZES=('10' '20' '30' '40' '50')

for i in ${DATA_FILES[@]}; do
  for j in ${ALGORITHMS[@]}; do
    for k in ${QUEUE_SIZES[@]}; do
      cargo -q run $j $k $i
    done
  done
done

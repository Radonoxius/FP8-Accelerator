#! /bin/bash

OPERATION="add"

cd dumps
diff soft_$OPERATION.csv fpga_$OPERATION.csv | grep '^>' | sed 's/^> //' > "$OPERATION"_diff.csv
cd ..
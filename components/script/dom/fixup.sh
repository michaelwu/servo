#!/bin/bash

for SRC in `ls *.rs`
do
	echo Converting $SRC &&
	sed -n -f dom2.sed $SRC > tmp.sed &&
	sed -i -f tmp.sed $SRC &&
	rm tmp.sed
done

#!/usr/bin/env bash

echo "digraph { " > out.dot
cat input.txt | sed -E 's/\([0-9]+\)//g' | sed -E 's/$/;/g' >> out.dot
echo "}" >> out.dot
dot out.dot -o out.svg -Tsvg
# TADA you can now open this file in firefox and find the answer to Part1

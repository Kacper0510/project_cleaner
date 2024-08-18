#!/bin/bash

test_dir="./test_dir"
mkdir -p "$test_dir"
cd "$test_dir"

for ((i=1; i<=10; i++))
do
    project_dir="project$i"
    cargo new "$project_dir"
    cd "$project_dir"
    cargo add ratatui
    cargo build &
    cd ..
done

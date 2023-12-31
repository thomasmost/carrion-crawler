#!/bin/bash

if which rustfmt &> /dev/null; then
    for rust_file in $(git diff --name-only --cached | grep ".*\.rs$"); do
        rustfmt $rust_file
        git add $rust_file
    done
else
    echo -e "\e[0;31mrustfmt is not installed\e[0m"
    exit 1
fi

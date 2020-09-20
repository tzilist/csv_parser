#!/usr/bin/env bash
set -eou pipefail

CSV_FILE=csv_generator/fake_people.csv

if [[ -f $CSV_FILE ]]; then 
    curl -X POST \
        -H "Content-Type: text/csv" \
        --data-binary @csv_generator/fake_people.csv \
        http://localhost:3000/api/parse
else
    printf "CSV File not found. Please generate the CSV file by runnning:\n\n"
    printf "cd csv_generator && npm install && npm run generate && cd -"
fi

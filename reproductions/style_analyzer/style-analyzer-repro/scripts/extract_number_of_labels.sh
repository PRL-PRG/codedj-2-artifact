    #!/bin/bash

if [ $# -eq 0 ] 
then
    echo "Usage: $0 REPORT_DIR"
    exit 1
fi

# Expect directory as first argument. This directory contains style-analyzer test and training reports for projects.
REPORT_DIR="$1"

# Get the relative path where the script is.
SCRIPT_DIR=$(dirname "$0")

# Expecting to have an accompanying `extract_number_of_labels.awk` parked next to the script.
EXTRACTION_AWK_SCRIPT="$SCRIPT_DIR/extract_number_of_labels.awk"

# Print out the CSV header
echo "repo, labels"

# Find all test reports in the report directory, expecting them to be named according to the pattern:
#
#   PROJECT-test_report.md.
#
for report in $(find "$REPORT_DIR" -name '*-test_report.md')
do
    # Extract the project name from the path
    project_name="$(basename $report .md)"
    project_name="${project_name%%-test_report}"
    
    # Skip the summary report
    if [ "$project_name" = "summary" ]
    then
        continue
    fi

    # Run the AWK scriptr for each report.
    labels=$(<"$report" awk -f $EXTRACTION_AWK_SCRIPT)   

    # Print out the extracted value
    echo "${project_name}, $labels"
done
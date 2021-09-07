#!/usr/bin/awk -f

# Collects the training sample count by style-analyzer from the report it generates. 
# It just produces a line for a CSV file containing the name of the repo (if provided) and two numbers. 
# Use extract_training_samples.sh to get a CSV for the whole dataset.
# Usage: <path/to/REPO-test-report.md extract_training_samples.awk -v repository=REPO

# The report is an MD file and it contains a table like this:
#
#   | Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
#   |------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
#   | `∅` | 0.981| 0.996| 0.995| 0.988| 0.988| 102225| 102307| 0.999 |
#   | `␣` | 0.957| 0.966| 0.960| 0.961| 0.958| 42569| 42840| 0.994 |
#   | `'` | 0.991| 0.973| 0.968| 0.982| 0.979| 18868| 18976| 0.994 |
#   | `⏎` | 0.852| 0.919| 0.825| 0.884| 0.838| 12826| 14285| 0.898 |
#   | `⏎␣⁺␣⁺` | 0.919| 0.836| 0.796| 0.875| 0.853| 4856| 5097| 0.953 |
#   | `⏎␣⁻␣⁻` | 0.889| 0.819| 0.783| 0.852| 0.833| 4155| 4343| 0.957 |
#   | `⏎⏎` | 0.886| 0.428| 0.231| 0.577| 0.366| 2387| 4421| 0.540 |
#   | `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 194| 197| 0.985 |
#   | `⏎⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 119| 238| 0.500 |
#   | `⏎␣⁺␣⁺␣⁺␣⁺␣⁺␣⁺␣⁺␣⁺␣⁺␣⁺␣⁺␣⁺␣⁺␣⁺␣⁺␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 65| 65| 1.000 |
#   | `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 46| 46| 1.000 |
#   | `⏎⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 41| 80| 0.512 |
#   | `⏎␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 38| 42| 0.905 |
#   | `⏎␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 33| 50| 0.660 |
#   | `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 32| 33| 0.970 |
#   | `⏎␣⁺␣⁺␣⁺␣⁺␣⁺␣⁺␣⁺␣⁺␣⁺␣⁺␣⁺␣⁺␣⁺␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 30| 30| 1.000 |
#   | `⏎␣⁺␣⁺␣⁺␣⁺␣⁺␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 24| 52| 0.462 |
#   | `⏎␣⁺␣⁺␣⁺␣⁺␣⁺␣⁺␣⁺␣⁺␣⁺␣⁺␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 18| 19| 0.947 |
#   | `␣␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 16| 42| 0.381 |
#   | `⏎⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 14| 14| 1.000 |
#   | `weighted avg` | 0.959| 0.963| 0.940| 0.960| 0.944| 188556| 193177| 0.976 |
#   | `macro avg` | 0.324| 0.297| 0.278| 0.306| 0.291| 188556| 193177| 0.976 |
#   | `micro avg` | 0.963| 0.963| 0.940| 0.963| 0.951| 188556| 193177| 0.976 |
#                                                         ^      ^
#                                                         |      |
#
# Each line represents a label, except weighed avg, macro avg, and micro avg, which are summaries.
# Also except the two rows representing the header and the alignment information.
# To get the number of labels, therefore, count all the rows except those five.
# We want to extract the Support and Full Support columns (so 8th and 9th) from the micro avg row.

# We will concentrate on MD tables and ignore any other contents of the file.
# So the field separator will be the column separator of MD tables: "|" and the
# row separator stays the same (new lines).
# So:
#
# | `⏎` | 0.852| 0.919| 0.825| 0.884| 0.838| 12826| 14285| 0.898 |
#
# OFS is set to commas for CSV line output
BEGIN {
    FS = "|";
    OFS = ",";
}

# We are interested only in the micro avg row.
$2 ~ /^ *`micro avg` *$/ {
    # Remove the whitespace padding and the ticks
    support = $8;
    full_support = $9;

    #print $0;

    # Remove whitespace
    gsub(/^ *`/, "", support);
    gsub(/` *$/, "", support);
    gsub(/^ *`/, "", full_support);
    gsub(/` *$/, "", full_support);

    # Store the label to an associative array: we're using it as a set
    # to make sure that we don't report on duplicate labels
    if (repository) {
        print repository, support, full_support;
    } else {
        print support, full_support;
    }
}
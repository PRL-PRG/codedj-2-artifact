#!/usr/bin/awk -f
# Given a summary test report MD table, creates a CSV table with the same numbers
# Usage: <path/to/summary-test-report.txt extract.summary.awk >path/to/output.csv

# Example summary table:
#
#   | repo               |   precision |   recall |   full_recall |    f1 |   full_f1 |   ppcr |   support |   full_support |   Rules Number |   Average Rule Len |
#   |:-------------------|------------:|---------:|--------------:|------:|----------:|-------:|----------:|---------------:|---------------:|-------------------:|
#   | telescope          |       0.826 |    0.826 |         0.457 | 0.826 |     0.588 |  0.553 |       172 |            311 |              3 |                2.7 |
#   | carlo              |       0.947 |    0.947 |         0.761 | 0.947 |     0.844 |  0.804 |      1601 |           1992 |             15 |                6.8 |
#   | reveal.js          |       0.954 |    0.954 |         0.778 | 0.954 |     0.857 |  0.815 |      8803 |          10799 |             39 |               10.9 |
#   | create-react-app   |       0.914 |    0.914 |         0.744 | 0.914 |     0.820 |  0.815 |      3505 |           4303 |             19 |                7.4 |
#   | axios              |       0.966 |    0.966 |         0.842 | 0.966 |     0.900 |  0.872 |      4976 |           5707 |             17 |                7.0 |
#   | redux              |       0.925 |    0.925 |         0.840 | 0.925 |     0.880 |  0.908 |      4956 |           5461 |             33 |                7.7 |
#   | citgm              |       0.917 |    0.917 |         0.904 | 0.917 |     0.910 |  0.985 |      5032 |           5107 |             86 |                6.3 |
#   | evergreen          |       0.925 |    0.925 |         0.864 | 0.925 |     0.893 |  0.934 |     19978 |          21381 |            342 |               11.1 |
#   | 30-seconds-of-code |       0.973 |    0.973 |         0.973 | 0.973 |     0.973 |  1.000 |     11493 |          11493 |             82 |                9.0 |
#   | express            |       0.958 |    0.958 |         0.890 | 0.958 |     0.922 |  0.929 |     14453 |          15557 |             42 |                8.7 |
#   | freeCodeCamp       |       0.929 |    0.929 |         0.894 | 0.929 |     0.911 |  0.962 |     23738 |          24670 |            236 |               12.5 |
#   | storybook          |       0.957 |    0.957 |         0.899 | 0.957 |     0.927 |  0.939 |     33970 |          36185 |             78 |                9.8 |
#   | jquery             |       0.969 |    0.969 |         0.933 | 0.969 |     0.950 |  0.963 |     46378 |          48173 |            289 |               12.2 |
#   | core-js            |       0.982 |    0.982 |         0.975 | 0.982 |     0.979 |  0.993 |     65836 |          66293 |             80 |                9.1 |
#   | atom               |       0.960 |    0.960 |         0.950 | 0.960 |     0.955 |  0.989 |     89662 |          90636 |            453 |                9.8 |
#   | react-native       |       0.933 |    0.933 |         0.912 | 0.933 |     0.922 |  0.978 |     88255 |          90259 |            820 |               12.0 |
#   | react              |       0.964 |    0.964 |         0.912 | 0.964 |     0.937 |  0.945 |     79004 |          83561 |            120 |               11.8 |
#   | meteor             |       0.915 |    0.915 |         0.864 | 0.915 |     0.888 |  0.944 |    136471 |         144540 |            661 |               13.3 |
#   | webpack            |       0.947 |    0.947 |         0.896 | 0.947 |     0.921 |  0.946 |     73084 |          77240 |            109 |               10.3 |
#   | node               |       0.963 |    0.963 |         0.940 | 0.963 |     0.951 |  0.976 |    188556 |         193177 |            701 |               13.1 |
#   | average            |       0.941 |    0.941 |         0.861 | 0.941 |     0.897 |  0.913 |     44996 |          46842 |            211 |                9.6 |
#   | weighted average   |       0.950 |    0.950 |         0.913 | 0.950 |     0.931 |  0.962 |           |                |                |                    |
#

# Since it's an MD table, set field separator to pipe. 
# Since the output is CSV, set output field separator to commas.
BEGIN {
    FS = "|";
    OFS = ",";
}

# Ignore the alignment specification expected in line 2
NR != 2 {
    # Skip the rows for average and weighted averages
    if ($2 ~ /average/) { next; }
    if ($2 ~ /weighted average/) { next; }

    # Save the value of each column into a map. We start with field two and end
    # with one field less than the nuymber of fields, since the table starts 
    # and ends with |
    for (i = 2; i < NF; i++) {
        field = $i;

        # Remove whitespace padding
        gsub(/^ +/,"",field);
        gsub(/ +$/,"",field);    

        # Print out.
        if (i == NF - 1) {
            printf("%s\n", field);
        } else {
            printf("%s, ", field);
        }
    }
}
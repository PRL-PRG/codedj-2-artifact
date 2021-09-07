#!/bin/bash

if [ $# -eq 0 ]
then
    # If no arguments were given, do evereything ireturned by the queries
    SELECTIONS=$(find "$(pwd)/selections/specs/" -iname '*.csv' | xargs -I{} basename "{}" .csv)
else
    SELECTIONS="$@"
fi

echo -e "server: 0.0.0.0:9930\ndb: sqlite:///database/lookout.sqlite\nfs: /tmp\n" > my_config.yml

time for selection in $SELECTIONS
do
   QUALITY_REPORT_REPOS="$(pwd)/selections/specs/$selection.csv"
   QUALITY_REPORT_DIR="$(pwd)/reproductions/$selection"

   mkdir -p ${QUALITY_REPORT_DIR};   
   chmod a+rw ${QUALITY_REPORT_DIR}; 

   echo "STARTING ${selection}"

    #-c my_config.yml \ why doesn't this work?
   time python3 -m lookout.style.format \
      --log-level DEBUG quality-report \
      -o ${QUALITY_REPORT_DIR} \
      -i ${QUALITY_REPORT_REPOS} \
      2>&1 | tee -a ${QUALITY_REPORT_DIR}/logs.txt

   echo "FINISHED ${selection}"
done

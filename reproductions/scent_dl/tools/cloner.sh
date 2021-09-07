INPUT=$1
OLDIFS=$IFS
IFS=','

[ ! -f $INPUT ] && { echo "$INPUT file not found"; exit 99; }

while read project_id
do
	~/codedj-parasite/target/release/./mistletoe --datastore ~/codedj-parasite/deep_learning/ export-project --id  $project_id --with-contents --into $2
done < $INPUT

IFS=$OLDIFS
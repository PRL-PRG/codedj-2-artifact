# run with -v output=csv or -v output=bib
# eg.:
#   <msr.tex awk -f msr.awk -v output=csv
#   <msr.tex awk -f msr.awk -v output=bib

BEGIN { 
    FS="\n"; 
    RS="\n\n";  

    if (output == "csv") {
        print "class, subclass, source, intuition, cite, title, authors, venue, year, doi, selected projects, notes"
    }
}

$0 ~ /^% >>/ { 

    # Clean up comment char and escape quotes
    for (i=1; i<=NF; i++) {
        sub("^% *", "", $i);
        gsub("\"", "\"\"", $i);
    }

    
    category = $1;
    source = $2;
    title = $3;
    all_authors = $4;
    venue = $5;
    doi = $6;
    projects = $7;
    justification = $8;

    # amalgamate the notes
    notes = ""
    for (i=9; i<=NF; i++) {
        if ($i ~ /[^ \t]/) {
            notes = notes $i;
            if (i != NF) {
                notes = notes "\n";
            }
        }
    }

    sub("^>> ", "", category); # remove garbage form the beginning of first line
    gsub(" ?, ?", " and ", all_authors); # convert commas to and in author lists

    if (output == "debug") {
        print "category ", category
        print "source ", source
        print "title ", title
        print "authors ", all_authors
        print "venue ", venue
        print "doi ", doi
        print "projects ", projects
        print "justification ", justification
    }

    # Extarct the last name of the first author.
    split(all_authors, authors, " and ")
    first_author=authors[1]
    split(first_author, first_author_names, " +")
    first_author_last_name=first_author_names[length(first_author_names)];   
            
    # Extract the name and year of the venue as separate entities.
    split(venue, venue_segments, " ");
    conference = venue_segments[1];
    year = venue_segments[2];

    sub("https://doi.org/", "", doi);
    sub("https://dl.acm.org/doi/", "", doi);

    split(category, classes, "|");
    altclass = classes[2];
    split(classes[1], class_segments, "(");
    class = class_segments[1];
    subclass = class_segments[2];
    gsub("\\)", "", subclass);

    if (output == "debug") {
        print "class ", class
        print "subclass ", subclass
        print "altclass ", altclass
        print "conference ", conference
        print "year ", year
        print "first_author ", first_author
        print "first_author_last_name ", first_author_last_name
        print "doi ", doi
        print "notes ", notes
        print ""
    }

    if (output == "csv") {    
        printf("\"%s\", \"%s\", \"%s\", \"%s\", \"%s:%d:%s\", \"%s\", \"%s\", \"%s\", \"%s\", \"%s\", %s, \"%s\"\n", 
            class, subclass, source, 
            justification,
            first_author_last_name, year, conference,
            title, all_authors, 
            conference, year, doi, projects, notes);
    }

    if (output == "bib") {        
        printf("@inproceedings{%s:%d:%s,\n", first_author_last_name, venue_segments[2], venue_segments[1])
        printf("  author = \"%s\",\n", all_authors);
        printf("  title = {\"%s\"},\n", title);
        printf("  doi = {%s},\n", doi);
        printf("  year = {%s},\n", venue_segments[2]);        
        printf("  booktitle = {Mining Software Repositories (MSR)},\n", doi);        
        printf("}\n\n");
    }
}

# @inproceedings{nakamaru:2020:MSR,
#   author = {Nakamaru, Tomoki and Matsunaga, Tomomasa and Yamazaki, Tetsuro and Akiyama, Soramichi and Chiba, Shigeru},
#   title = {An Empirical Study of Method Chaining in {Java}},
#   year = {2020},
#   doi = {10.1145/3379597.3387441},
#   booktitle = {Mining Software Repositories (MSR)},
# }
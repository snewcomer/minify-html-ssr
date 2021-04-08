#!/bin/bash

count=$( cat test.html | wc -c )
count_gzip=$( cat test.html | gzip | wc -c )
size=$( du -sh test.html | cut -f 1)
rm -rf test.html.gz
gzip -c test.html > test.html.gz
size_gzip=$( du -sh test.html.gz | cut -f 1)

count_min=$( cat test.min.html | wc -c )
count_min_gzip=$( cat test.min.html | gzip | wc -c )
size_min=$( du -sh test.min.html | cut -f 1)
rm -rf test.min.html.gz
gzip -c test.min.html > test.min.html.gz
size_min_gzip=$( du -sh test.min.html.gz | cut -f 1)

echo "----
Date: $( date +"%m-%d-%y" )

· Large
Char Count: $count
Char Count Gzip: $count_gzip
Size: $size
Size Gzip: $size_gzip

· Minified
Char Count: $count_min
Char Count Gzip: $count_min_gzip
Size: $size_min
Size Gzip: $size_min_gzip
----
" >> log.txt

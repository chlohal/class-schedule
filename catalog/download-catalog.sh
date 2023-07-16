#!/bin/bash

i=1

catid=$1
navid=$2

while true 
do

    echo "Page $i"
    courses=$(curl --globoff --silent "https://catalog.clarku.edu/content.php?catoid=$catid&catoid=$catid&navoid=$navid&filter[item_type]=3&filter[only_active]=0&filter[3]=0&filter[cpage]=$i" 2>&1 | grep -o 'preview_course_nopop.php?catoid=[0-9]*&coid=[0-9]*' | sed 's/preview_course_nopop/preview_course/g')
    have_course=0
    for course_page in $courses 
    do
        catalog_id=$(echo "$course_page" | grep -o 'catoid=[0-9]*' | cut -c8-)
        course_id=$(echo "$course_page" | grep -o 'coid=[0-9]*' | cut -c6-)

        if [ "$course_id" != "" ]
        then have_course=1
        fi

        if [ ! -d "courses/$catalog_id" ]
        then 
            mkdir "courses/$catalog_id"
        fi

        if [ ! -e "courses/$catalog_id/$course_id" ]
        then
            wget -O "courses/$catalog_id/$course_id" "https://catalog.clarku.edu/ajax/$course_page&show"
        fi
    done

    if [ $have_course = 0 ]
    then break
    fi

    i=$((i+1))
done
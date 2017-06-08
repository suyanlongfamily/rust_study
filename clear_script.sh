#!/usr/bin/env bash
project_dir=$(pwd)
mv_tmp=$(mkdir -p $project_dir)
clear_items=("target" "Cargo.lock" "cmake-build-debug" "CMakeLists.txt" ".idea")

function is_clear(){
    for item in ${clear_items[*]}
    do
        echo "item = $item"
        if test $1 = $item
        then
            return 1;
        fi;
    done;
    echo "false"
    return 0;
}
# test is_clear
# is_clear "Cargo.lock"

function clear(){
    echo `ls -a $1`
    for item in `ls -a $1`
    do
        if test $item = "."
        then
           continue;
        fi;
        if test $item = ".."
        then
           continue;
        fi;
        if test $item = ".git"
        then
           continue;
        fi;

        tmp_item="${1}/${item}"
        echo $tmp_item
        is_clear "$item"
        is_ok=$?
        echo ${is_ok}
        if test ${is_ok} -eq 1
        then
            echo "mv -f $tmp_item /tmp/"
            sudo mv -f $tmp_item /tmp/ | rm -rf $tmp_item;
        else
            if test -d $tmp_item
            then
                echo "cd $tmp_item"
                clear "$tmp_item"
            fi;
        fi;
    done;
}

clear "$project_dir"
#echo `ls $project_dir`
#echo "$project_dir";


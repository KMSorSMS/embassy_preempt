#!/bin/bash
rm count_exclude.txt
# 定义需要忽略的文件夹，是个数组
exclude_files=(
    "src/bin"
    "target"
)
# 将exclude_files数组中的元素写入到count_exclude.txt文件中
for exclude_file in ${exclude_files[@]}; do
    echo $exclude_file >> count_exclude.txt
done
find . -name "*.yml" >> count_exclude.txt
find . -name "*.yaml" >> count_exclude.txt
find . -name "*.json" >> count_exclude.txt
cloc . --exclude-list-file=count_exclude.txt

#!/bin/bash

[ -z "$1" ] && echo "Gimme a maildir" && exit 1
[ -z "$2" ] && echo "Gimme a query" && exit 1

export LC_ALL='C'

: > dump.raw
for file in $(ag Subject: "$1" | ag "$2" | cut -d: -f1,2); do
  mu extract --parts=1 --overwrite $file
  awk '/^[^>]/ { print }' 1.msgpart |\
    awk 'NR==1,/--/' |\
    grep -v 'wrote:$' |\
    grep -v '^On' |\
    grep -v '^Em [^S]' |\
    grep -v 'GMT' |\
    cat \
    >> dump.raw
done

sort -u dump.raw | iconv -f utf-8 -t utf-8 -c > dump
echo "All on dump"

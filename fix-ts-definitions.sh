#!/bin/bash

path=$1

if [[ $path == *.d.ts ]]
then
    BValues=(BUndefined BNull BBoolean BNumber BString BArray BObject BUnion)
    joined=$(IFS=, ; echo "${BValues[*]}")

    declare -a values=("BUndefined" "BNull" "BBoolean" "BNumber" "BString" "BArray" "BObject" "BUnion" "BTuple")

    # sed -i "1s|^|import {$joined} from '../index'\n\n|" $path;
    sed -i "1s|^|import { type ParseSafe } from './index'\n\n|" $path;

    # change class names
    for value in ${values[@]}
    do
        # sed -i "s/export class $value/export class Nt$value/g" $path;
        sed -i "s/export class $value/export class $value<R>/g" $path;
    done

    # add generics to merge methods
    sed -i "s/merge/merge<T>/g" $path;
fi

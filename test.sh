#!/usr/bin/bash

ERR=`cargo run 2>&1 |grep Backtrace`

if [ $ERR = "Backtrace:" ]
then
   exit 1
fi

exit 0













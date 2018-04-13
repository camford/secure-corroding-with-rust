#!/bin/sh

# 
# is~bo::main[1]                                                                                                                                                                                       
# 0x00007a10                                                                                                                                                                                                         
# [0x00000000]> s 0x00007a10                                                                                                                                                                                         
# [0x00007a10]> pdf

r2 -A -q -c 'pdf @ sub.signal_110' ../../target/debug/bo > uaf-c-bugged.txt

#!/bin/bash

btcdeb '[OP_2 OP_1 OP_ADD]'

: '

Stack : 

[2]
[1]        =====>    [3]
[+]

'

btcdeb '[OP_6 OP_2 OP_SUB OP_4 OP_EQUAL]'

: '

Stack :

[6]
[2]                  [4]
[-]        =====>    [4]    =====>    [1]
[4]                  [=]
[=]

'

# script   |  stack 
# ---------+--------
# 6        | 
# 2        | 
# OP_SUB   | 
# 4        | 
# OP_EQUAL | 
# #0000 6
# btcdeb> step
# *                 <> PUSH stack 06
# script   |  stack 
# ---------+--------
# 2        |      06
# OP_SUB   | 
# 4        | 
# OP_EQUAL | 
# #0001 2
# btcdeb> step
# *                 <> PUSH stack 02
# script   |  stack 
# ---------+--------
# OP_SUB   |      02
# 4        |      06
# OP_EQUAL | 
# #0002 OP_SUB
# btcdeb> step
# *                <> POP  stack
# *                <> POP  stack
# *                <> PUSH stack 04
# script   |  stack 
# ---------+--------
# 4        |      04
# OP_EQUAL | 
# #0003 4
# btcdeb> step
# *                <> PUSH stack 04
# script   |  stack 
# ---------+--------
# OP_EQUAL |      04
#          |      04
# #0004 OP_EQUAL
# btcdeb> step
# *                <> POP  stack
# *                <> POP  stack
# *                <> PUSH stack 01
# script   |  stack 
# ---------+--------
#          |      01
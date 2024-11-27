# README

## Description

Oever's Simple Digital Logic Simulator (OSD), is a simple digital logic simulator which aims at easily simulating logic gates with a simple domain-specific-language (DSL) called Leverscript. Leverscript/OSD end with .osd

## Features

- Basic Logic Gates (AND, OR, NOT, NAND, NOR, XOR and XNOR gates)

## Planned Features

- Advanced Logic Components like flip-flops
 
## Leverscript Grammar

```ENBF
Program ::= inputs_section outputs_section component_list
inputs_section ::= "INPUTS" identifier_list "NEWLINE"
outputs_section ::= "OUTPUTS" identifier_list "NEWLINE"

identifier_list ::= identifier { "," identifier }*

identifier     ::= letter { letter | digit | "_" }*

component_list ::= { component "NEWLINE" }*

component      ::= gate_type identifier "IN" "(" identifier_list ")" "OUT" "(" identifier ")"

gate_type      ::= "AND" | "OR" | "NOT" | "NAND" | "NOR" | "XOR" | "XNOR"

letter         ::= "a" | "b" | ... | "z" | "A" | "B" | ... | "Z"

digit          ::= "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9"

NEWLINE        ::= "\n"
```
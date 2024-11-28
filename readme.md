# Description

Oever's Simple Digital Logic Simulator (OSD), is a simple digital logic simulator which aims at easily simulating logic gates with a simple domain-specific-language (DSL) called Leverscript. Leverscript/OSD files end with .osd

# Installation

## Windows

Put the osd.exe from releases in your PATH folder, and you're all the done.

## Linux and MacOS

Not supported

# Usage

Run ```osd generate {file_path}``` in your terminal and it will generate a truth table from your .osd file.

# Available Components

- AND 
- OR 
- NOT 
- NAND 
- NOR 
- XOR 
- XNOR

# Planned Features

- More Advanced Logic Components

# Leverscript Syntax

Program Structure:

```Leverscript
INPUTS {input_list}                                # Defines your inputs
OUTPUTS {output_list}                              # Defines your outputs

{component_list}                                   # Defines your components
```

Components:

```Leverscript
{type} {identifier} IN({input_list}) OUT({output}) # Defines a component, with a a type, identifier and the inputs and output
```

## Example
```Leverscript
INPUTS a, b, c                  # Defines three inputs: a, b and c
OUTPUTS r                       # Defines the output: r

AND gate1 IN(a, b) OUT(t)       # Defines an AND gate with identifier gate1, inputs a and b and output t
OR  gate2 IN(t, c) OUT(r)       # Defines an OR gate with identifier gate2, inputs t and c and output r
```
## Leverscript Grammar

```ENBF
Program         ::= inputs_section outputs_section component_list
inputs_section  ::= "INPUTS" identifier_list NEWLINE
outputs_section ::= "OUTPUTS" identifier_list NEWLINE

identifier_list ::= identifier { "," identifier }*

identifier      ::= letter { letter | digit | "_" }*

component_list  ::= { component NEWLINE }*

component       ::= type identifier "IN" "(" identifier_list ")" "OUT" "(" identifier ")"

type            ::= "AND" | "OR" | "NOT" | "NAND" | "NOR" | "XOR" | "XNOR"

letter          ::= "a" | "b" | ... | "z" | "A" | "B" | ... | "Z"

digit           ::= "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9"

NEWLINE         ::= "\n"
```
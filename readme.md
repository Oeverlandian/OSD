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
- Subcircuits

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
SUBCIRCUIT full_adder                       # Declares a subcircuit "full_adder"

INPUTS a, b Cin                             # Declares the inputs for the subcircuit
OUTPUTS Sum, Cout                           # Declares the outputs for the subcircuit

XOR gate1 IN(a, b) OUT(temp1)               # Declares a XOR gate
XOR gate2 IN(temp1, Cin) OUT(Sum)           # Declares a XOR gate

AND gate3 IN(temp1, Cin) OUT(temp2)         # Declares an AND gate
AND gate4 IN(a, b) OUT(temp3)               # Declares an AND gate
OR gate5 IN(temp2, temp3) OUT(Cout)         # Declares an OR gate

END                                         # Ends the subcircuit declaration

INPUTS a0, b0, a1, b1, a2, b2, a3, b3, Cin  # Declares the global inputs
OUTPUTS s0, s1, s2, s3, Cout                # Declares the global outputs

full_adder IN(a0, b0, Cin) OUT(s0, C0)      # Declares a full_adder subcircuit as a component
full_adder IN(a1, b1, C0) OUT(s1, C1)       # Declares a full_adder subcircuit as a component

full_adder IN(a2, b2, C1) OUT(s2, C2)       # Declares a full_adder subcircuit as a component
full_adder IN(a3, b3, C2) OUT(s3, Cout)     # Declares a full_adder subcircuit as a component
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
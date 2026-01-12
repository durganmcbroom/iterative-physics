// Derived from https://itnext.io/writing-a-mathematical-expression-parser-35b0b78f869e

grammar math;

expression
    : addition '=' addition
    | addition
    ;

addition
    : addition ('+' | '-') multiplication
    | multiplication
    ;

multiplication
    : multiplication ('*' | '/') signed
    | multiplication (('(' addition ')') | exponentiation) // TODO just added this, might be wrong but want to handle implicit multiplication
    | signed
    ;

signed
    : '-' exponentiation
    | exponentiation
    ;

exponentiation
    : exponentiation '^' atom
    | atom
    ;

atom
    : NUMBER
    | ID
    | '(' addition ')'
    | ID '(' addition (',' addition)* ')' // a function
    ;


NUMBER
    : [0-9]+ ('.' [0-9]+)?
    ;

ID
    : [a-zA-Z_]+
    ;

WS
    : [ \t\r\n]+ -> skip
    ;


# simple_interpreter

https://ruslanspivak.com/lsbasi-part1/

## Basic operator prescedence parsing

```
expr : term ((PLUS | MINUS) term)*
term : factor ((MUL | DIV) factor)*
factor : integer
```

## Basic operator prescedence parsing with parentheses

```
expr : term ((PLUS | MINUS) term)*
term : factor ((MUL | DIV) factor)*
factor : number (POWER factor)*
number : '(' expr ')' | integer
```

## Add unary operators

```
expr : term ((PLUS | MINUS) term)*
term : factor ((MUL | DIV) factor)*
factor : number (POWER factor)*
number : LParen expr RParen | (PLUS | MINUS) factor | integer
```

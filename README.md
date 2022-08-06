# simple_interpreter

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
factor : integer
```

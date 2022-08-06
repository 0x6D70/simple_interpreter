# simple_interpreter

## Basic operator prescedence parsing

```
expr : factor ((PLUS | MINUS) factor)*
factor : term ((MUL | DIV) term)*
term : integer
```
Example: 1 * 2 + 3 * 4

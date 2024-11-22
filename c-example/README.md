## Example binary written in pure C

Goal of this code is to quickly verify C API and also to catch leaks early without rust involvement.

Recommended valgrind run
```
valgrind --leak-check=full --suppressions=./valgrind.sup ./main 1>/dev/null
```

# Search-In-File

A command-line tool for searching for a pattern within a file.

## Example

File content of 'fruits.txt':
```
apple
banana
blueberries
cherries cherries cherries
lemon
raspberries and bananas
```

Calling `sif` command:
```
# sif fruits.txt banana
Ln 2, Col 1
Ln 6, Col 17

# sif fruits.txt lemon
Ln 5, Col 1

# sif fruits.txt ^banana$
Ln 2, Col 1

# sif fruits.txt cherries
Ln 4, Col 1
Ln 4, Col 10
Ln 4, Col 19
```

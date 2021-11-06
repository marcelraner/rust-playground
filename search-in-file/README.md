# Search-In-File

A command-line tool for searching for a pattern within a file.

## Example

File content of 'some_text.txt':
```
apple
banana
blueberries
cherries
lemon
raspberries and bananas
```

Calling `sif` command:
```
# sif some_text.txt banana
Ln 2, Col 1
Ln 6, Col 17

# sif some_text.txt lemon
Ln 5, Col 1
```

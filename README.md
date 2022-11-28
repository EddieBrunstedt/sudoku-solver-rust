# Solve any Sudoku using a backtracking algorithm.

## Usage
```
sudoku-solver -s <values>
```
or:
```
sudoku-solver -f <file_path>
```

## Options
```
  -s, --input-from-stdin <values>    Each cell's value from left to right
  -f, --input-from-file <file_path>  Read values from a file
  -h, --help                         Print help information
  -V, --version                      Print version information
```

## Format of input
```
Values 0-9 where 0 represents an empty cell.
Any other character will be stripped away.

Examples:
530070000-600195000-098000060-800060003-400803001-700020006-060000280-000419005-000080079
002810700/400200000/080009060/000600020/005040300/100007008/090000050/006000400/300000001

Tip:
If you use values from file (with '-f' flag), feel free to use line breaks
in the file as they will be stripped away automatically.
```

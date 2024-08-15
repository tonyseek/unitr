# unitr - translate or delete unicode characters

**unitr** is similar to [**tr**][tr-manpages] but works with all unicode
characters, not just ASCII characters.

[tr-manpages]: https://man7.org/linux/man-pages/man1/tr.1.html

## Installation

_TBD_

## Usage

### Synopsis

```
unitr [OPTION]... STRING1 [STRING2]
```

### Description

> Translate, squeeze, and/or delete characters from standard input, writing to
> standard output.

`STRING1` and `STRING2` specify arrays of characters `ARRAY1` and `ARRAY2` that
control the action.

| Option                     | Description                                                                                                                          |
| -------------------------- | ------------------------------------------------------------------------------------------------------------------------------------ |
| `-c`, `-C`, `--complement` | Use the complement of `ARRAY1`                                                                                                       |
| `-d`, `--delete`           | Delete characters in `ARRAY1`, do not translate                                                                                      |
| `-s`, `--squeeze-repeats`  | Replace each sequence of a repeated character that is listed in the last specified ARRAY, with a single occurrence of that character |
| `-t`, `--truncate-set1`    | First truncate `ARRAY1` to length of `ARRAY2`                                                                                        |
| `--version`                | Output version information and exit                                                                                                  |

ARRAYs are specified as strings of characters.

## Compatibility

_TBD_

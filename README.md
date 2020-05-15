# @dislick/pwgen

Basic CLI password generator. I wasn't satisfied with the speed of [my other pasword generator](https://github.com/dislick/ts-pwgen), so I decided to learn Rust to make it _blazing fast_ 🔥.

### To do

- [x] Parse arguments
- [x] Add more charsets
- [ ] Copy to clipboard feature
- [x] Support more parameters
- [ ] Documentation
- [ ] Tests

## Usage

```
pwgen 0.1.0
Patrick Muff <muff.pa@gmail.com>
Generates random passwords

USAGE:
    pwgen [FLAGS] [OPTIONS] [SUBCOMMAND]

FLAGS:
    -a, --alphabet              Use ALPHABET (a-z) charset
    -A, --alphabet-uppercase    Use ALPHABET_UPPERCASE (A-Z) charset
    -n, --numbers               Use NUMBERS (0-9) charset
    -s, --special               Use SPECIAL (*, %, -, ...) charset
    -h, --help                  Prints help information
    -V, --version               Prints version information

OPTIONS:
    -c, --count <COUNT>      Amount of passwords
    -l, --length <LENGTH>    Length of passwords

SUBCOMMANDS:
    help      Prints this message or the help of the given subcommand(s)
    secret    Creates secret with at least 256 bits of entropy
```

## Defaults

If you run `pwgen` without a `--length` option it will determine the length based on the available charsets to get **at least 256 bits of entropy**. The less different characters that are available through those charsets, the longer the password will get.

Without charset flags (`-a`, `-A`, `-n`, or `-s`) present, it uses all possible charsets.

## Examples

`pwgen`

```bash
# length defaults to 40, because 91 distinct chars are available
s7YRg_L!5,b10e9{R57842/5]/3@2Fc3y57i[3g\
```

`pwgen -a`

```bash
# length defaults to 55, because 26 distinct chars are available
cqkptjdawtbzmcdojiqwtcabvkragecntdctxwvwhtwbhgxdnfshvyj
```

`pwgen -A`

```bash
# length defaults to 55, because 26 distinct chars are available
GZWHIQEZMWJSAPPIULBYHIRMGJHYFKERLDYTABWFIKICTCJACTGIWEZ
```

`pwgen -Aa`

```bash
# length defaults to 45, because 52 distinct chars are available
qvaPlUtPsYpTDTYOPwBbFclwYNDjLUqdrAhRRLdQoyEZu
```

`pwgen -n`

```bash
# length defaults to 78, because 10 distinct chars are available
292040492503533901251005568137604050532386930398662181203682665536587940100830
```

`pwgen -s`

```bash
# length defaults to 53, because 29 distinct chars are available
)}:!([_[^<&;@>}>-~;=({@[\@>,[,~<\=?.=,*]#_(};.?;<~@^[
```

`pwgen -l 20`

```bash
# length is hard-set to 20
C4Y0iN77K*4^SzFeG{HY
```

`pwgen -l 20 -c 5`

```
X!Lv2|^;Z]Y[436Ven.F
)({]+)30O*5mDi1UZl|l
0`;OqE|r3~<v@=N8P5r}
pQ6UmkIu3_7p#T_ZSDUa
kucJ%~45g69Zan7-,Y7Q
```

`pwgen secret`

```
Lp47SqOH6BJYg59HDvYTP1zo1BpKa0hN51U6Rlrcb6n
```

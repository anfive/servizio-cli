# `servizio-cli`
A command-line utility to encode/decode style scores.

This utility has three modes of operation: decode, encode, and file processing.

## Decode

Decodes a Style Code and prints the style judgement to standard output.
Usage:

    servizio-cli <code> [--value=<value>] [--raw]

where
 * `code` is the style code
 * `value` (optional) specifies the value of the style judgement to be output. It can be one of `score`, `bas`, `mov`, `din`, `com`, `sapd`, `gcc`, `dif`, `sog`, `pen` (case insensitive).

 If `--value` is not specfied, the program will print all values, one per line.
 If `--raw` is used, the program will only print the values, one per line, without other text. This is useful if the program is used in scripts.

 The program will have a nonzero exit code in case of error (e.g. invalid style code).

Examples:

    > servizio-cli g13dm12
    Decoding input code: g13dm12
    Score: 7.2
    BAS  : 1
    MOV  : 3
    DIN  : 2
    COM  : 1
    SAPD : 3
    GCC  : 1
    DIF  : 2
    SOG  : 1
    PEN  : 2


    > servizio-cli g13dm12 --raw
    7.2
    1
    3
    2
    1
    3
    1
    2
    1
    2


    > servizio-cli g13dm12 --value=sapd
    Decoding input code: g13dm12
    sapd : 3

 ## Encode

 Encodes a style judgement into a Style Code.
 Usage:

    servizio-cli --encode=<judgement> [--raw]

where `judgement` is a style judgement represented as a comma-separated list of points in different categories, without spaces. Categories not mentioned in the style judgement are assumed to be zero. For example, a vlaid judgement string is `bas=1,mov=3,gcc=2,pen=1`.

If `--raw` is used, the program will only print the value, one per line, without other text. This is useful if the program is used in scripts.

 The program will have a nonzero exit code in case of error (e.g. invalid judgement string).

Examples: 

    > servizio-cli --encode=bas=1,mov=3,gcc=2,pen=1
    Encode: bas=1,mov=3,gcc=2,pen=1
    r6k01


    > servizio-cli --encode=bas=1,mov=3,din=2,com=1,sapd=3,gcc=1,dif=2,sog=1,pen=2
    Encode: bas=1,mov=3,din=2,com=1,sapd=3,gcc=1,dif=2,sog=1,pen=2
    g13dm12


    > servizio-cli --encode=bas=1,mov=3,gcc=2,pen=1 --raw
    r6k01

## File Processing

This mode processes an entire `csv` file, bulk-decoding a column of style codes and writing the results to another file.
The program expects exactly one column containing style codes and will append the decoded values at the end of each row.

Usage:

    servizio-cli --infile=<infile> --outfile=<outfile> [--headers] [--column=<col>] [--delimiter=<d>]

where
* `infile` is the input `csv` file.
* `outfile` is the output file. The program will overwrite existing files without warning. The input and output files must be distinct.
* If `headers` (optional) is specified, the program will treat the first line of the input file as column headers and will not attempt to decode any record; instead, it will append column headers for the decoded values at the end of the first row.
* `column` (optional) can be used to specify the (zero-based) index of the column to decode. If `column` is not specified, the program will attempt to decode the last column.
* `delimiter` (optional) can be used to specify the delimiter (a single character) to use in the `csv` files. The default is comma: `,` . Note that you might need to put the delimiter in quotes, e.g. `--delimiter=";"`.

Example 1:

    > servizio-cli --infile=in.csv --outfile=out.csv

Given `in.csv`:

| | |
|---|---|
| Mario | n4 |
| Luigi | d6r |
| Paolo | h4a |

the program will decode the last column, producing the output file `out.csv`:

| | | | | | | | | | | | |
|---|---|---|---|---|---|---|---|---|---|---|---|
|Mario|n4|6.3|1|1|1|0|0|1|0|0|0|
|Luigi|d6r|6.7|1|0|2|1|0|2|0|0|0|
|Paolo|h4a|6.3|1|1|1|0|0|0|1|0|0|

Example 2:


    > servizio-cli --infile=in.csv --outfile=out.csv --headers --delimiter=";" --column=1

Given `in.csv`:

| Name | Style Code | Rank |
|---|---|---|
| Mario | n4 | Iniziato |
| Luigi | d6r | Accademico |
| Paolo | h4a | Cavaliere |

the program will decode column with zero-based index `1` (the second one) and will add headers to the first row, producing the output file `out.csv` is:

| Name | Style Code | Rank | Score | BAS | MOV | DIN | COM | SAPD | GCC | DIF | SOG | PEN |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
|Mario|n4| Iniziato |6.3|1|1|1|0|0|1|0|0|0|
|Luigi|d6r| Accademico |6.7|1|0|2|1|0|2|0|0|0|
|Paolo|h4a| Cavaliere |6.3|1|1|1|0|0|0|1|0|0|
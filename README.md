# Fpas


fpas is command line software helps you create password from some text 


## Installation

 ```bash
 cargo install fpas
 ```

## Usage

create password with text

```bash
fpas your_text
```

and loop feature

```bash
fpas -l 5 your_text
```

create password from binary file

```bash
fpas -f ./file
```

using `--chain` with `-l` (or `--loop`)
to generate long password

```bash
fpas --chain -l 10 your_text
```

## Use cases

You might want to create a compressed file with a password. To ensure the strongest security, you need to use a very long password that includes a variety of characters.

So, I created this software to solve this problem and create strong passwords with a simple method. However, it actually just transferred the weakness to other points.

There is example cases with command line.

```bash
7z a file.7z file -p$(fpas helloG)
```

```bash
zip file.zip file -P $(fpas hello)
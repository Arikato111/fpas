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
cat ./file | fpas
```

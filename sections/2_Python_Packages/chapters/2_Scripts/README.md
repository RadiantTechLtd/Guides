# Scripts

## 0 - Add run.py

Our package will be a command line tool, so we'll need a way to run it.
Create a directory called [`scripts`](./scripts):

```bash
mkdir scripts
```

And then within this directory, create a file called [`run.py`](./scripts/run.py):

```bash
touch scripts/run.py
```

## 1 - Add run() function

Inside `run.py` we'll add some code to parse the command line arguments, and then call our `sample` function:

```python
import argparse
import mandy

parser = argparse.ArgumentParser()
parser.add_argument("real", type=float)
parser.add_argument("imag", type=float)
args = parser.parse_args()

iterations = mandy.sample(args.real, args.imag)

print(f"({args.real}, {args.imag}) -> {iterations}")
```

## Try it out

We can run the script by running the following command:

```bash
poetry run python scripts/run.py 0 0
```

The first argument is the path to the script, and the remaining arguments are the values for `real` and `imag`.

This should print out the following:

```bash
(0.0, 0.0) -> 100
```

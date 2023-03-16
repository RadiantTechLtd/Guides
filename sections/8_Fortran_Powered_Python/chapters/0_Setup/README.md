# Project Setup

We'll be using the `F2PY` tool to compile `Fortran` code into a `Python` module.
This will allow us to use `Fortran` code in our `Python` code.

## Poetry Init

```shell
poetry init
```

-   I'm going to call this project `adder`
-   Starting at version `0.0.0`
-   My description will be: `Calling Fortran from Python.`
-   I am: `FreddyWordingham <freddy@digilab.co.uk>`
-   I'm not going to add a license
-   Compatible with `Python` versions `3.10` and above
-   And I'm not going to add any dependencies right now

This will generate a [`pyproject.toml`](./pyproject.toml) file containing the project metadata.

Then install the packages:

```shell
poetry install
```

## Add .gitignore

I'm going to add a [`.gitignore`](./.gitignore) file to my project.
`Python` projects use a lot of files that we don't want to commit to `git`, so we can use a template to generate a `.gitignore` file for us.

I'm going to pull the code from https://www.toptal.com/developers/gitignore/api/python into a `.gitignore` file using the `curl` command:

```bash
curl -L https://www.toptal.com/developers/gitignore/api/python,fortran > .gitignore
```

Alternatively, you can go to [gitignore.io](https://gitignore.io/) and generate a template more tailored to your needs.

## Add Dependencies

```shell
poetry add numpy
```

## Library

We'll create a directory called `adder` and inside that, we'll create a file called `__init__.py`:

```shell
mkdir adder
touch adder/__init__.py
```

Inside the [`__init__.py`](./adder/__init__.py) file we'll add the following code:

```python
from . import fortran
```

Ready to import our `Fortran` module in a moment.

## Add Fortran

We'll also add a `src` directory for our compiled `Fortran` code.
Inside it, we'll create the signature `main.f90` file:

```shell
mkdir src
touch src/main.f90
```

Inside `main.f90` we'll add the following code:

```fortran
program hello
    print *, 'Hello, World!'
end program hello
```

## Build

We can now build our `Fortran` code into a `Python` module:

```shell
poetry run f2py -c src/main.f90 -m adder.fortran
```

This will create a `*.so` file in the `adder` directory.

## Scripts

Finally, we'll create a `scripts` directory for our `Python` scripts.

```shell
mkdir scripts
touch scripts/run.py
```

And inside [`run.py`](./scripts/run.py) we'll add the following code:

```python
import adder

adder.fortran.hello()
```

## Try it out

Let's run the script:

```shell
poetry run python scripts/run.py
```

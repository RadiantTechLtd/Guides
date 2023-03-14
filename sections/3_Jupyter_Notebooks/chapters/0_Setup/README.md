# Setup

We'll create the minimal file structure for a `Python` project using the `Poetry` package manager with the `Jupyter` tool dependency installed.

## Add pyproject.toml

Create a new `Poetry` project:

```shell
poetry init
```

-   I'm going to call this project `my_notes`
-   Starting at version `0.0.0`
-   My description will be: `Jupyter notebook example.`
-   I am: `FreddyWordingham <freddy@digilab.co.uk>`
-   I'm not going to add a license
-   Compatible with `Python` versions `3.10` and above
-   And I'm not going to add any dependencies right now

This will generate a [`pyproject.toml`](./pyproject.toml) file containing the project metadata.

## Create a package directory

Although we won't use it immediately, we're going to create a `my_notes` directory to contain the code for our package.

```shell
mkdir my_notes
touch my_notes/__init__.py
```

We'll add an empty [`__init__.py`](./my_notes/__init__.py) file to the directory so that `Python` recognises it as a package.

## Install the package

We can install the package by running the following command:

```shell
poetry install
```

## Add .gitignore

I'm going to add a [`.gitignore`](./.gitignore) file to my project.
`Python` projects use a lot of files that we don't want to commit to `git`, so we can use a template to generate a `.gitignore` file for us.

I'm going to pull the code from https://www.toptal.com/developers/gitignore/api/python into a `.gitignore` file using the `curl` command:

```shell
curl -L https://www.toptal.com/developers/gitignore/api/python > .gitignore
```

Alternatively, you can go to [gitignore.io](https://gitignore.io/) and generate a template more tailored to your needs.

## Add dependencies

We're going to need the `jupyter` tool dependency to get started.

```shell
poetry add jupyter
```

## Try it out

We can run the application by running the following command:

```shell
poetry run jupyter notebook --port 8888
```

This will start a notebook server on port 8888.
If you go to http://localhost:8888, you should see the Jupyter notebook interface.

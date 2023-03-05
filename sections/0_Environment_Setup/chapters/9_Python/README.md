# Python - Language Interpreter

`Python` is a programming language that is used to write high-level code.

## Installation

Although a version of the `Python` interpreter is already installed on your computer, you can install a newer version using `Homebrew`:

```bash
brew install python
```

By default, the interpreter will be installed under the command `python3`, but you can create an alias for the `Python` command to make it easier to use:

```bash
echo 'alias python="python3"' >> ~/.zshrc
```

## Usage

If you were to use `Python` as is it will write everything into the global workspace, which is not ideal.

Instead, you should use `Poetry` (explained in the next chapter) to create a new project and install/manage the dependencies you need.

If you REALLY to use `Python` without `Poetry`, you can create a new project using the following command:

```bash
python -m venv my_project
```

You can then activate the project using the following command:

```bash
source my_project/bin/activate
```

You can then install a dependency using the following command:

```bash
pip install some_dependency
```

You can then run a script using the following command:

```bash
python path/to/my_script.py
```

## Details

`Python` is a high-level, general-purpose programming language that was first released in 1991 by Guido van Rossum.
It is designed to be easy to read and write, with a simple syntax and a wide range of built-in functions and modules.

`Python` is popular among developers for several reasons:

-   Easy to learn: `Python` has a simple and intuitive syntax, making it easy to learn for beginners.
-   Versatile: `Python` can be used for a wide range of applications, from web development to data analysis and scientific computing.
-   Large community: `Python` has a large and active community of developers, which means that there are many resources and libraries available for developers to use and learn from.
-   Cross-platform: `Python` is available for a wide range of operating systems, including Windows, macOS, and Linux.
-   Interpreted: `Python` code is interpreted, which means that it can be executed on any platform without the need for compilation.

`Python` is also known for its extensive standard library, which provides developers with a wide range of modules and tools for working with files, networking, databases, and many other tasks.

Overall, `Python` is a versatile and popular programming language that is used by developers for a wide range of applications, from web development to data analysis and scientific computing.
Its simple syntax, large community, and an extensive standard library make it a great choice for beginners and experienced developers alike.

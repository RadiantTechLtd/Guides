# MPI for Python

Example `MPI` with `Python`.

## Dependencies

You'll need `Poetry`:

```shell
brew install poetry
```

and need `MPI`:

```shell
brew install open-mpi
```

## Quickstart

Clone the repository and change into the directory:

```bash
git clone https://github.com/Guides/utility/MPI_for_Python
cd MPI_for_Python
```

Install the `Poetry` package dependencies:

```bash
poetry install
```

Run the `MPI` example:

```bash
mpirun -n 4 python scripts/0_basic.py
```

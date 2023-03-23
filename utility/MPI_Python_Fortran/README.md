# MPI Connecting Python and Fortran

## Install dependencies

```shell
poetry install
```

## Compile the Fortran code

```shell
mpif90 main.f90 -o main.exe
```

## Run the Fortran code

```shell
mpirun -n 2 poetry run python run.py : -n 2 main.exe
```

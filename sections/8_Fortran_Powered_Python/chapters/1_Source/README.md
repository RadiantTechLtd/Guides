# Source Code

Let's add a `Fortran` function that accepts some arguments.

## Fibonacci

Add the following code to `main.f90`:

```fortran
subroutine fibonacci(A,N)

    INTEGER N
    REAL*8 A(N)

    DO I=1,N
        IF (I.EQ.1) THEN
        A(I) = 0.0D0
        ELSEIF (I.EQ.2) THEN
        A(I) = 1.0D0
        ELSE
        A(I) = A(I-1) + A(I-2)
        ENDIF
    ENDDO

END
```

## Build

We can now build our `Fortran` code into a `Python` module:

```shell
poetry run f2py -c src/main.f90 -m adder.fortran
```

## Run

Then we can run the script:

```shell
poetry run python scripts/run.py
```

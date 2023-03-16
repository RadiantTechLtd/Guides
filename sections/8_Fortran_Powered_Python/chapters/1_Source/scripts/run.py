import numpy as np

import adder

a = np.ones(8, "d")
adder.fortran.fibonacci(a)
print(a)

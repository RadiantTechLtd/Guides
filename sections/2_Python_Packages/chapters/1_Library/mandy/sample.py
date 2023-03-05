def sample(real, imag, max_iterations=100):
    c = real + imag * 1j
    z = 0j

    for i in range(max_iterations):
        z = z * z + c

        if abs(z) > 2.0:
            return i

    return max_iterations

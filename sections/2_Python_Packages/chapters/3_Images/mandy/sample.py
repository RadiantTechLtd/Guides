import numpy as np


def point(real, imag, max_iterations=100):
    c = real + imag * 1j
    z = 0j

    for i in range(max_iterations):
        z = z * z + c

        if abs(z) > 2.0:
            return i

    return max_iterations


def area(real, imag, width, height, scale, max_iterations=100):
    re = np.linspace(real - 0.5 * scale * width, real + 0.5 * scale * width, width)
    im = np.linspace(imag - 0.5 * scale * height, imag + 0.5 * scale * height, height)

    mandelbrot_set = np.zeros((height, width))

    for i in range(height):
        for j in range(width):
            mandelbrot_set[i, j] = point(re[i], im[j], max_iterations)

    return mandelbrot_set

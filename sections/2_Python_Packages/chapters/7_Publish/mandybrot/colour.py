from PIL import Image
import numpy as np


def grayscale(value, max_iters):
    x = value / max_iters * 255
    return (x, x, x)


def jet_map(value, max_iters):
    v = value / max_iters

    r, g, b = 0, 0, 0

    if v < 0.125:
        r = 0
        g = 0
        b = 0.5 + 0.5 * (4 * v)
    elif v < 0.375:
        r = 0
        g = 4 * (v - 0.125)
        b = 1
    elif v < 0.625:
        r = 4 * (v - 0.375)
        g = 1
        b = 1 - 4 * (v - 0.375)
    elif v < 0.875:
        r = 1
        g = 1 - 4 * (v - 0.625)
        b = 0
    else:
        r = 1 - 0.5 * (v - 0.875)
        g = 0
        b = 0

    return (int(r * 255), int(g * 255), int(b * 255))


def image(data, max_iters, colouring):
    height, width = data.shape
    img = np.zeros((height, width, 3), dtype=np.uint8)

    for i in range(height):
        for j in range(width):
            img[i, j] = colouring(data[i, j], max_iters)

    return img


def encode(data):
    return Image.fromarray(data)

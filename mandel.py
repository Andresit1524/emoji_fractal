import math
import time
from itertools import product

from numba import njit

SIZE = 340
RES = 0.00005
MAX_ITERS = 1500
CENTER = complex(-0.104894547, 0.927887283)


def get_color_emoji(iters: int) -> str:
    colors = "⬛🟦🟩🟨🟧🟥🟫"

    # Si llegó al máximo, lo pintamos como el centro (oscuro)
    if iters == MAX_ITERS:
        return "⬛"

    # Normalizamos el valor de 0 a la cantidad de colores
    index: int = math.floor(iters * len(colors) / MAX_ITERS)
    return colors[index]


@njit
def mandelbrot(z: complex) -> int:
    c: complex = z

    for i in range(MAX_ITERS + 1):
        if abs(z) > 2:
            return i

        z = z * z + c

    return MAX_ITERS


start = time.perf_counter()

with open("result.txt", "w", encoding="utf-8") as f:
    for i, j in product(range(SIZE), range(SIZE)):
        point = complex((2 * j / SIZE) - 1, -(2 * i / SIZE) + 1) * RES + CENTER
        iters: int = mandelbrot(point)
        char: str = get_color_emoji(iters)

        f.write(char)
        f.write("\n" if j == SIZE - 1 else "")

elapsed = time.perf_counter() - start
print(f"Hecho en {elapsed} s")

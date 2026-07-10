from itertools import product
import math


# region Definitions


class Complex:
    y: float
    x: float

    def __init__(self, x, y):
        self.x = x
        self.y = y

    def length(self):
        return self.x**2 + self.y**2

    def __add__(self, other):
        return Complex(self.x + other.x, self.y + other.y)

    def __sub__(self, other):
        return Complex(self.x - other.x, self.y - other.y)

    def __mul__(self, other):
        return Complex(self.x * other, self.y * other)

    def __pow__(self, other):
        if other != 2:
            raise ValueError(f"Exponents not defined: {other}")

        return Complex(self.x**2 - self.y**2, 2 * self.x * self.y)

    def __str__(self):
        return f"({self.x} + {self.y}i)"


# endregion


SIZE = 340
RES = 0.00001
MAX_ITERS = 1500
CENTER = Complex(-0.104894547, 0.927887283)


def get_color_emoji(iters: int) -> str:
    colors = "⬛🟦🟩🟨🟧🟥🟫"

    # Si llegó al máximo, lo pintamos como el centro (oscuro)
    if iters == MAX_ITERS:
        return "⬛"

    # Normalizamos el valor de 0 a la cantidad de colores
    index: int = math.floor(iters * len(colors) / MAX_ITERS)
    return colors[index]


def mandelbrot(z: Complex) -> int:
    c: Complex = z

    for i in range(MAX_ITERS + 1):
        if z.length() > 2:
            return i

        z = z**2 + c

    return MAX_ITERS


with open("result.txt", "w", encoding="utf-8") as f:
    for i, j in product(range(SIZE), range(SIZE)):
        point = Complex((2 * j / SIZE) - 1, -(2 * i / SIZE) + 1) * RES + CENTER
        iters: int = mandelbrot(point)
        char: str = get_color_emoji(iters)

        f.write(char)
        f.write("\n" if j == SIZE - 1 else "")

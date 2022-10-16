"""
This script generates a Rust file that defines
1-dimensional Gauss-Legendre quadrature rules
on the unit interval [0, 1] up to degree 100.
"""

import numpy as np

# np.set_printoptions(16)

nmax = 100

max_len = nmax * (nmax + 1) // 2

points = np.zeros(max_len, dtype=np.float64)
weights = np.zeros(max_len, dtype=np.float64)

counter = 0
for n in range(1, nmax + 1):
    p, w = np.polynomial.legendre.leggauss(n)
    sorted_indices = np.argsort(p)
    points[counter : (counter + n)] = p[sorted_indices]
    weights[counter : (counter + n)] = w[sorted_indices]
    counter += n

points = 0.5 * (1 + points)
weights *= 0.5

with open("gauss_legendre.rs", "w") as f:
    f.write("//! Definition of Gauss-Legendre points up to order 100 in the interval [0, 1].\n")
    f.write("//!\n")
    f.write("//! The points for the n-th rule start have start index n * (n-1) / 2.\n")
    f.write("\n")
    f.write(f"pub(crate) static GAUSS_POINTS: [f64; {len(points)}] = [\n")
    for p in points:
        f.write(f"{p},\n")
    f.write("];\n")

    f.write("\n")
    f.write(f"pub(crate) static GAUSS_WEIGHTS: [f64; {len(weights)}] = [\n")
    for w in weights:
        f.write(f"{w},\n")
    f.write("];\n")

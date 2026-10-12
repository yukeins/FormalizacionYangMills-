import math
from fractions import Fraction
def _fact_int(x): return math.factorial(int(x))
def _delta(a, b, c):
    num = _fact_int(a + b - c) * _fact_int(a - b + c) * _fact_int(-a + b + c)
    den = _fact_int(a + b + c + 1)
    return math.sqrt(num / den)
def sixj(j1, j2, j3, j4, j5, j6):
    pref = (_delta(j1, j2, j3) * _delta(j1, j5, j6)
            * _delta(j4, j2, j6) * _delta(j4, j5, j3))
    k_min = max(j1 + j2 + j3, j1 + j5 + j6, j4 + j2 + j6, j4 + j5 + j3)
    k_max = min(j1 + j2 + j4 + j5, j2 + j3 + j5 + j6, j3 + j1 + j6 + j4)
    total = 0.0
    for k in range(int(k_min), int(k_max)+1):
        num = (-1)**int(k) * _fact_int(k + 1)
        args = [k - j1 - j2 - j3, k - j1 - j5 - j6,
            k - j4 - j2 - j6, k - j4 - j5 - j3,
            j1 + j2 + j4 + j5 - k, j2 + j3 + j5 + j6 - k,
            j3 + j1 + j6 + j4 - k]
        den = 1
        for a in args: den *= _fact_int(a)
        total += num / den
    return pref * total
print("0.5:", sixj(0.5, 0.5, 0.5, 0.5, 0.5, 0.5))
print("1.0:", sixj(1.0, 1.0, 1.0, 1.0, 1.0, 1.0))

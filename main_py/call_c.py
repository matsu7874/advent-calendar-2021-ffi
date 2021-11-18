import ctypes

legend = ctypes.CDLL('../legendary_c_lib/legend.so')
legend.gcd.argtypes = [ctypes.c_int, ctypes.c_int]
legend.gcd.restype  = ctypes.c_int

assert legend.gcd(120, 16) == 8


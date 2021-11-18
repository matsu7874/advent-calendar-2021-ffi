import ctypes

rust = ctypes.CDLL('../modest_rs_lib_for_c/target/release/libmodest_rs_lib_for_c.so')
rust.gcd.argtypes = [ctypes.c_int, ctypes.c_int]
rust.gcd.restype  = ctypes.c_int
assert rust.gcd(120, 16) == 8

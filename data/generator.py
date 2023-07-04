import numpy as np
import math
import matplotlib.pyplot as plt

def sieve(n):
    i = 2
    while i <= math.sqrt(n):
        if n % i == 0:
            return False
        i += 1
    return True

# ~n primes between 2..(N+) (log uniform)
def gen_prim_log(n, N):
    res = np.zeros(n)

    res = np.array(np.int32(2**((np.log2(N) - 1)*np.random.rand(n) + 1)))
    
    for i in range(n):
        while True:
            if sieve(res[i]):
                break
            res[i] += 1
    
    return np.unique(res)

# ~n composites between 2..(N+) (log uniform)
def gen_comp_log(n, N):
    res = np.zeros(n)

    res = np.array(np.int32(2**((np.log2(N) - 1)*np.random.rand(n) + 1)))
    
    for i in range(n):
        while True:
            if not sieve(res[i]):
                break
            res[i] += 1
    
    return np.unique(res)
    
        
pr = gen_prim_log(1000, 1000000)
print(pr, len(pr))
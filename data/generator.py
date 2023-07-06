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
    
    pr = np.unique(res)
    f_write = open('data/prim_log/prim_log.txt', 'w')
    for p in pr:
        f_write.write(str(p) + '\n')
    return pr

# ~n composites between 2..(N+) (log uniform)
def gen_comp_log(n, N):
    res = np.zeros(n)

    res = np.array(np.int32(2**((np.log2(N) - 1)*np.random.rand(n) + 1)))
    
    for i in range(n):
        while True:
            if not sieve(res[i]):
                break
            res[i] += 1
    
    cp = np.unique(res)
    f_write = open('data/comp_log/comp_log.txt', 'w')
    for c in cp:
        f_write.write(str(c) + '\n')
    return cp
    

pr = gen_prim_log(1000, 1000000)
cp = gen_comp_log(1000, 1000000)
print(pr, len(pr))
print(cp, len(cp))
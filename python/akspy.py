import numpy as np
import math

def phi(x):
    res = 0        
    for k in range(1, x + 1):
        if math.gcd(x, k) == 1:
            res += 1
    return res

def mul_mod(p, q, r, n):
    degp = len(p) - 1
    degq = len(q) - 1
    res = np.zeros(min(degp + degq + 1, r))
    
    for i in range(degp + 1):
        for j in range(degq + 1):
            if i + j < r:
                res[i + j] += p[i]*q[j]
            else:
                res[i + j - r] += p[i]*q[j]
    return res % n

def pow_mod(p, r, n):
    res = np.array([1])
    bits = int(math.log2(n)) + 1
    for i in range(bits, -1, -1):
        res = mul_mod(res, res, r, n)
        if n >> i & 1 == 1:
            res = mul_mod(res, p, r, n)
    return res

def aks(n):
    n = int(n)
    B = np.ceil(np.log2(n))
    
    #Step 1
    b = 2
    while b <= B:
        a = np.ceil(n**(1/b))
        if a**b == n:
            #print('s1')
            return False
        b += 1
        
    #Step 2
    maxk = B**2
    nextR = True
    
    r = 2
    while nextR:
        nextR = False
        k = 1
        while (not nextR) and k <= maxk:
            nextR = (n**k % r == 1) or (n**k % r == 0)
            k += 1
        r += 1
    r -= 1
    
    print('r:', r)
    
    #Step 3
    for a in range(r, 1, -1):
        gcd = math.gcd(a, n)
        if 1 < gcd and gcd < n:
            return False
        
    #Step 4
    if n <= r:
        return True
    
    #Step 5
    maxa = np.floor((B + 1)*(np.sqrt(phi(r)) + 1))
    a = 1
    while a <= maxa:
        p = pow_mod(np.array([a, 1]), r, n)
        p[n % r] -= 1
        p[0] -= a
        p %= n
        
        if not np.all(p == 0):
            return False
        
        a += 1

    return True

def sieve(n):
    i = 2
    while i <= math.sqrt(n):
        if n % i == 0:
            return False
        i += 1
    return True

if __name__ == '__main__':
    import time
    while True:
        n = input('n: ')
        start = time.perf_counter()
        prime = aks(n)
        end = time.perf_counter()
        print(prime)
        print('t:', end - start, 's')

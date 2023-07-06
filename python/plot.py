import matplotlib.pyplot as plt
import numpy as np
from akspy import sieve, aks
import aks as aksr
import time
import random


plt.figure(dpi=300)

def get_values(n, m):
    values = np.zeros(n, dtype=int)
    i = 0
    while i < n:
        newv = random.randint(2, m)
        
        #if not newv in values:
        if not newv in values and sieve(newv):
            values[i] = newv
            i += 1
    values.sort()
    return values


def test_aks(values):  
    n = len(values)  

    times = np.zeros(n)
    results = np.zeros(n)

    for i in range(n):
        start = time.perf_counter()
        results[i] = aks(values[i])
        times[i] = time.perf_counter() - start
    
    means = np.zeros(n)
    h = 2
    for i in range(n):
        means[i] = np.mean(times[max(i - h, 0):min(i + h + 1, n)])

    plt.plot(values, times, '.', label='AKS-Algorithmus')
    plt.plot(values, means, color='blue')
    
    return results


def test_aksr(values):  
    n = len(values)  

    times = np.zeros(n)
    results = np.zeros(n)

    for i in range(n):
        start = time.perf_counter()
        results[i] = aksr.prime(values[i])
        times[i] = time.perf_counter() - start
    
    means = np.zeros(n)
    h = 2
    for i in range(n):
        means[i] = np.mean(times[max(i - h, 0):min(i + h + 1, n)])

    plt.plot(values, times, '.', color='black', label='AKS-Algorithmus (Rust)')
    plt.plot(values, means, color='black')
    
    return results


def test_sieve(values):
    n = len(values)    

    times = np.zeros(n)
    results = np.zeros(n)

    for i in range(n):
        start = time.perf_counter()
        results[i] = sieve(values[i])
        times[i] = time.perf_counter() - start
    
    means = np.zeros(n)
    h = 3
    for i in range(n):
        means[i] = np.mean(times[max(i - h, 0):min(i + h + 1, n)])

    #plt.plot(np.log2(values), times, '.', markeredgewidth=0.5, markersize=5, color='orange', label='Sieb des Eratosthenes')
    plt.plot(values, means, color='orange', label='Probedivision')
    #plt.plot(np.log2(values), means, color='orange', label='Probedivision')
    #plt.xlabel('$log (n)$')
    #plt.plot(values, means, color='orange', label='Sieb des Eratosthenes')


    return results

def test_combi():
    n = 50
    m = 1000
    values = get_values(n, m)

    test_aks(values)
    test_aksr(values)
    test_sieve(values)
    
    plt.legend()
    plt.xlabel('n')
    plt.ylabel('$t$ in s')
    plt.savefig('data/combi2')
    plt.show()


def test_small():
    for i in range(2, 100):
        print(i, aks(i), sieve(i))

def check(r1, r2, values):
    for i in range(len(r1)):
        if r1[i] != r2[i]:
            print(i, r1[i], r2[i], values[i])
            return False
    return True

def test_single():

    start = time.perf_counter()
    prime = aks(15485863)
    print(prime, time.perf_counter() - start)    


test_combi()
#test3()
#test_small()
#test_single()
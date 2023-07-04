import numpy as np
import matplotlib.pyplot as plt

f_num = open('data/test.txt', 'r')
f_t = open('data/times_small.txt', 'r')

x = np.array([int(l) for l in f_num.read().splitlines()])
y = np.array([float(l) for l in f_t.read().splitlines()])

A = np.vstack([np.log2(np.log2(x))[100:], np.ones(len(x[100:]))]).T
m, c = np.linalg.lstsq(A, np.log2(y)[100:], rcond=None)[0]

print(m, c)


plt.loglog(np.log2(x), 2**(np.log2(np.log2(x))*m+c))
plt.loglog(np.log2(x), y, 'k.', markersize=3)
plt.xlabel('$\log(n)$')
plt.ylabel('$t$ in ms')
plt.show()

plt.plot(np.log2(x), y, 'k.', markersize=3)
plt.plot(np.log2(x), np.log2(x)**m*2**c)
plt.xlabel('$\log(n)$')
plt.ylabel('$t$ in ms')
plt.show()
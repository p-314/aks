import numpy as np
import matplotlib.pyplot as plt

name = 'prim_log'
f_num = open('data/' + name + '/' + name + '.txt', 'r')
f_t = open('data/' + name + '/times_' + name + '.txt', 'r')

x = np.array([int(l) for l in f_num.read().splitlines()])
y = np.array([float(l) for l in f_t.read().splitlines()])

ign = 50
A = np.vstack([np.log2(np.log2(x))[ign:], np.ones(len(x[ign:]))]).T
m, c = np.linalg.lstsq(A, np.log2(y)[ign:], rcond=None)[0]

print(m, c)

plt.figure(dpi=300)

plt.loglog(np.log2(x), y, 'kx', markeredgewidth=0.5, markersize=5, label='Laufzeit AKS-Algorithmus')
plt.loglog(np.log2(x), 2**(np.log2(np.log2(x))*m+c), label='Asymptotische Annäherung')
plt.xlim(0.9, 25)
plt.xlabel('$\log(n)$')
plt.ylabel('$t$ in ms')
plt.legend()
plt.savefig('data/' + name + '/' + name + '1app')
plt.show()


plt.figure(dpi=300)

plt.plot(np.log2(x), y, 'kx', markeredgewidth=0.5, markersize=5, label='Laufzeit AKS-Algotithmus')
plt.plot(np.log2(x), np.log2(x)**m*2**c, label='Asymptotische Annäherung')
plt.xlabel('$\log(n)$')
plt.ylabel('$t$ in ms')
plt.legend()
plt.savefig('data/' + name + '/' + name + '2app')
plt.show()
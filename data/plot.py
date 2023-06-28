import numpy as np
import matplotlib.pyplot as plt

f_num = open('big.txt', 'r')
f_t = open('times.txt', 'r')

x = [int(l) for l in f_num.read().splitlines()]
y = [float(l) for l in f_t.read().splitlines()]

plt.plot(x, y, '.')
plt.xscale('log')
plt.show()
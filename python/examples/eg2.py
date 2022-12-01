from searchsorted._internal import cartesian_5d, searchsorted_5d 

import time
from itertools import product
from random import choice, sample
from string import ascii_lowercase, digits

chars = ascii_lowercase + digits
lst1 = [''.join(choice(chars) for _ in range(2)) for _ in range(100)]
lst2 = [''.join(choice(chars) for _ in range(2)) for _ in range(10)]
lst3 = [''.join(choice(chars) for _ in range(2)) for _ in range(10)]
lst4 = [''.join(choice(chars) for _ in range(2)) for _ in range(10)]
lst5 = [''.join(choice(chars) for _ in range(2)) for _ in range(10)]

def pysearchsorted(full_list, sample_list):
    fullhashtable = {tupl: idx for idx, tupl in enumerate(full_list)}
    return [fullhashtable[tupl] for tupl in sample_list]


init = time.time()
full_list = cartesian_5d(lst1,lst2,lst3,lst4,lst5)
diff = round((time.time()-init)*1000000)
print(f"Rust-python   eTime:{diff: >8} μs.  cartesian  {full_list[:2]=}")
init = time.time()
full_list = list(product(lst1,lst2,lst3,lst4,lst5))
diff = round((time.time()-init)*1000000)
print(f"Pure python   eTime:{diff: >8} μs.  cartesian  {full_list[:2]=}")

sample_list = sample(full_list, 1000)

init = time.time()
index = searchsorted_5d(full_list, sample_list)
diff = round((time.time()-init)*1000000)
print(f"Rust-python   eTime:{diff: >8} μs.  searchsorted  {index[:2]=}")
init = time.time()
index = pysearchsorted(full_list, sample_list)
diff = round((time.time()-init)*1000000)
print(f"Pure python   eTime:{diff: >8} μs.  searchsorted  {index[:2]=}")



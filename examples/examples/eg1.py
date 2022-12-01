from searchsorted._internal import cartesian_3d, searchsorted_3d 


sample = [('a1','b2','c1'),('a2','b3','c2')]

dim1 = ['a1','a2']
dim2 = ['b1','b2','b3']
dim3 = ['c1','c2']

full = cartesian_3d(dim1,dim2,dim3)

index = searchsorted_3d(full, sample)

print(f"Location of sample tuples in full: {index}")

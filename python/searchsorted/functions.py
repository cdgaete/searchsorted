
from ._internal import (
    cartesian_1d,
    cartesian_2d,
    cartesian_3d,
    cartesian_4d,
    cartesian_5d,
    cartesian_6d,
    cartesian_7d,
    cartesian_8d,
    searchsorted_1d,
    searchsorted_2d,
    searchsorted_3d,
    searchsorted_4d,
    searchsorted_5d,
    searchsorted_6d,
    searchsorted_7d,
    searchsorted_8d,
)
from itertools import product

def cartesian_product(*args):
    assert all([isinstance(arg, (list, np.ndarray)) for arg in args])
    container = []
    if all([isinstance(arg, list) for arg in args]):
        for arg in args:
            if len(arg) > 0:
                if isinstance(arg[0], str):
                    container.append(arg)
                else:
                    container.append([str(elem) for elem in arg])
    elif all([isinstance(arg, np.ndarray) for arg in args]):
        for arg in args:
            if arg.ndim == 1:
                if issubclass(arg.dtype.type, np.object_):
                    container.append(arg.tolist())
                else:
                    container.append(arg.astype(str).tolist())
            else:
                raise Exception("ndarray must be of one dimension only")
    else:
        raise Exception("Arguments must be list o ndarray")
 
    items_nr = len(container)
    if items_nr == 0:
        return list()
    elif items_nr == 1:
        return cartesian_1d(*container)
    elif items_nr == 2:
        return cartesian_2d(*container)
    elif items_nr == 3:
        return cartesian_3d(*container)
    elif items_nr == 4:
        return cartesian_4d(*container)
    elif items_nr == 5:
        return cartesian_5d(*container)
    elif items_nr == 6:
        return cartesian_6d(*container)
    elif items_nr == 7:
        return cartesian_7d(*container)
    elif items_nr == 8:
        return cartesian_8d(*container)
    else:
        print("Efficient method applies for arrays with eight or less dimensions")
        return list(product(*container))


def searchsorted(unique_array, sample_array):
    if len(unique_array) == len(sample_array) == 0:
        return list()
    assert len(unique_array[0]) == len(sample_array[0])
    items_nr = len(unique_array[0])
    if items_nr == 1:
        return searchsorted_1d(unique_array, sample_array)
    elif items_nr == 2:
        return searchsorted_2d(unique_array, sample_array)
    elif items_nr == 3:
        return searchsorted_3d(unique_array, sample_array)
    elif items_nr == 4:
        return searchsorted_4d(unique_array, sample_array)
    elif items_nr == 5:
        return searchsorted_5d(unique_array, sample_array)
    elif items_nr == 6:
        return searchsorted_6d(unique_array, sample_array)
    elif items_nr == 7:
        return searchsorted_7d(unique_array, sample_array)
    elif items_nr == 8:
        return searchsorted_8d(unique_array, sample_array)
    else:
        print("Efficient method applies for arrays with eight or less dimensions")
        hshTbl = {ai: idx for idx, ai in enumerate(unique_array)}
        return [hshTbl[bi] for bi in sample_array]
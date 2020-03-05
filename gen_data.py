import numpy as np
from argparse import ArgumentParser

if __name__ == "__main__":
    parser = ArgumentParser(description="Generate Data")
    parser.add_argument("--min", type=int, metavar="min value", default=0)
    parser.add_argument("--max", type=int, metavar="max value", default=100)
    parser.add_argument("--size", type=int, metavar="number of output", default=1000)
    parser.add_argument("--name", type=str, metavar="File name")
    
    args = parser.parse_args()
    min_val = args.min
    max_val = args.max
    size = args.size
    fn = args.name
    fn = fn + ".csv" if ".csv" not in fn else fn

    np.savetxt(fn, np.random.randint(min_val, max_val, size), delimiter=",", fmt="%d")
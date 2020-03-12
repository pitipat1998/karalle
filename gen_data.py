from argparse import ArgumentParser
from functools import partial
from multiprocessing import Pool, cpu_count
from os import listdir, mkdir

import numpy as np


def smart_gen_util(ssize, minv, maxv):
    return np.random.randint(minv, maxv, ssize)


def smart_gen(ssize, minv, maxv, threshold=1_000_000):
    # print("size", ssize)
    rounds = ssize // threshold
    p = Pool(cpu_count())
    mainc = [threshold] * rounds
    leftover = [ssize % threshold]
    if leftover[0] > 0:
        mainc += leftover
    # print("mainc", mainc)
    ret = p.map(partial(smart_gen_util, minv=minv, maxv=maxv),
                mainc)
    print("Finish generating, reducing array")
    to_r = np.concatenate(ret).ravel()
    print("resulting size", len(to_r))
    return to_r


def do_gen(ssize, type_for, minv, maxv):
    data_dir = listdir("data")
    if type_for in ["map", "filter"]:
        fn = f"data/{type_for}/size-{ssize}.csv"
        np.savetxt(fname=fn,
                   X=np.random.randint(minv, maxv, ssize),
                   delimiter=",",
                   fmt="%d")
        print("File saved to ", fn)
    elif type_for == "flatten":
        if "flatten" not in data_dir:
            mkdir("data/flatten")
        print("Generating data for size : ", ssize)
        fn = f"data/{type_for}/size-{ssize}.csv"
        with open(fn, "w") as f:
            for line in range(ssize):
                lst = [np.random.randint(minv, maxv) for i in range(np.random.randint(3, 200))]
                to_write = ",".join(str(i) for i in lst) + "\n"
                f.write(to_write)
        print("File saved to ", fn)


def gen(t, minv, maxv, ifrom=0, ito=0, size=0):
    print(f"Generating data with size 2**{ifrom} to 2**{ito}")
    if ito > 0:
        p = Pool(min(cpu_count() - 1, ito - ifrom))
        p.map(partial(do_gen, type_for=t, minv=minv, maxv=maxv),
              [2 ** i for i in range(ito, ifrom, -1)])
    else:
        do_gen(size, t, minv, maxv)


if __name__ == "__main__":
    parser = ArgumentParser(description="Generate Data")
    parser.add_argument("--min", type=int, metavar="min value", default=3)
    parser.add_argument("--max", type=int, metavar="max value", default=1000)
    parser.add_argument("--size", type=int, metavar="2**<size> generated data", default=0)
    parser.add_argument("--type", type=str, metavar="<map | flatten | filter | all>", default="map")
    parser.add_argument("-a", "--all", default=False, action='store_true')
    parser.add_argument("--ifrom", type=int, metavar="from 2^<from>", default=2)
    parser.add_argument("--ito", type=int, metavar="to 2^<to>", default=10)
    parser.add_argument("--smart", default=False, action='store_true')
    #     parser.add_argument("--name", type=str, metavar="File name")

    args = parser.parse_args()
    min_val = args.min
    max_val = args.max
    size = 2**args.size
    smart = args.smart
    gen_type = args.type
    ifrom = args.ifrom
    ito = args.ito

    dir = listdir("data")
    if gen_type not in dir:
        mkdir(f"data/{gen_type}")
    if smart and gen_type in ["map", "filter"]:
        for i in range(10, 12):
            fn = f"data/{gen_type}/size-{2**i}.csv"
            np.savetxt(fn, smart_gen(2**i, min_val, max_val), fmt="%d", delimiter=",")
        exit(0)

    if ito > 0 and size > 0:
        print("Using all option, ignoring size")
    elif all(i == 0 for i in [size, ifrom, ito]):
        print("Please specify size or provide --ifrom --ito")
        exit(1)

    if gen_type not in ["map", "flatten", "filter", "all"]:
        print("Only these types supported: map, flatten, filter, all")
        exit(1)

    if gen_type == "all":
        for i in ["map", "flatten", "filter"]:
            gen(i, min_val, max_val, ifrom=args.ifrom, ito=args.ito, size=size)
    else:
        gen(gen_type, min_val, max_val, ifrom=args.ifrom, ito=args.ito, size=size)

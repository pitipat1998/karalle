from argparse import ArgumentParser
from functools import partial
from multiprocessing import Pool, cpu_count
from os import listdir, mkdir

import numpy as np


def do_gen(size, gen_type, min_val, max_val):
    data_dir = listdir("data")
    if gen_type in ["map", "filter"]:
        if gen_type not in data_dir:
            mkdir(f"data/{gen_type}")
        fn = f"data/{gen_type}/size-{size}.csv"
        np.savetxt(fn, np.random.randint(min_val, max_val, size), delimiter=",",
                   fmt="%d")
        print("File saved to ", fn)
    elif gen_type == "flatten":
        if "flatten" not in data_dir:
            mkdir("data/flatten")
        ret = [[np.random.randint(min_val, max_val) for i in range(np.random.randint(3, 20))] for x in range(size)]
        fn = f"data/{gen_type}/size-{size}.csv"
        with open(fn, "w") as f:
            for line in ret:
                to_write = ",".join(str(i) for i in line) + "\n"
                f.write(to_write)
        print("File saved to ", fn)


def gen(t, ifrom=0, ito=0, size=0):
    if ito > 0:
        p = Pool(min(cpu_count() - 1, ito - ifrom))
        p.map(partial(do_gen, gen_type=t, min_val=min_val, max_val=max_val), [2 ** i for i in range(ito, ifrom, -1)])
    else:
        do_gen(t, min_val, max_val, size)


if __name__ == "__main__":
    parser = ArgumentParser(description="Generate Data")
    parser.add_argument("--min", type=int, metavar="min value", default=0)
    parser.add_argument("--max", type=int, metavar="max value", default=100)
    parser.add_argument("--size", type=int, metavar="number of output", default=0)
    parser.add_argument("--type", type=str, metavar="<map | flatten | filter | all>", default="map")
    parser.add_argument("-a", "--all", default=False, action='store_true')
    parser.add_argument("--ifrom", type=int, metavar="from 2^<from>", default=0)
    parser.add_argument("--ito", type=int, metavar="to 2^<to>", default=0)
    #     parser.add_argument("--name", type=str, metavar="File name")

    args = parser.parse_args()
    min_val = args.min
    max_val = args.max
    size = args.size
    gen_type = args.type
    ifrom = args.ifrom
    ito = args.ito

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
            gen(i, ifrom=args.ifrom, ito=args.ito, size=size)
    else:
        gen(gen_type, ifrom=args.ifrom, ito=args.ito, size=size)

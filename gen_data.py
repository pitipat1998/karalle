from argparse import ArgumentParser
from os import listdir, mkdir

import numpy as np


def do_gen(gen_type, min_val, max_val, size):
    data_dir = listdir("data")
    if gen_type not in ["map", "flatten", "filter"]:
        print("Only these types supported: map, flatten, filter")
        exit(1)
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


if __name__ == "__main__":
    parser = ArgumentParser(description="Generate Data")
    parser.add_argument("--min", type=int, metavar="min value", default=0)
    parser.add_argument("--max", type=int, metavar="max value", default=100)
    parser.add_argument("--size", type=int, metavar="number of output", default=0)
    parser.add_argument("--type", type=str, metavar="<map | flatten | filter>", default="map")
    parser.add_argument("-a", "--all", default=False, action='store_true')
    parser.add_argument("--ifrom", type=int, metavar="from 2^<from>", default=0)
    parser.add_argument("--ito", type=int, metavar="to 2^<to>", default=10)
    #     parser.add_argument("--name", type=str, metavar="File name")

    args = parser.parse_args()
    min_val = args.min
    max_val = args.max
    size = args.size
    gen_type = args.type
    is_all = args.all

    if is_all and size > 0:
        print("Using all option, ignoring size")
    elif not is_all and size == 0:
        print("Please specify size or provide --all --from --to")

    if is_all:
        for i in range(args.ifrom, args.ito):
            do_gen(gen_type, min_val, max_val, 2 ** i)
    else:
        do_gen(gen_type, min_val, max_val, size)

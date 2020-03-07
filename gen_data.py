import numpy as np
from argparse import ArgumentParser
from os import listdir, mkdir


data_dir = listdir("data")
if __name__ == "__main__":
    parser = ArgumentParser(description="Generate Data")
    parser.add_argument("--min", type=int, metavar="min value", default=0)
    parser.add_argument("--max", type=int, metavar="max value", default=100)
    parser.add_argument("--size", type=int, metavar="number of output", default=1000)
    parser.add_argument("--type", type=str, metavar="<map | flatten | filter>", default="map")
    #     parser.add_argument("--name", type=str, metavar="File name")

    args = parser.parse_args()
    min_val = args.min
    max_val = args.max
    size = args.size
    gen_type = args.type

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

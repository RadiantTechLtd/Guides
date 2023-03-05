import argparse
import mandy

parser = argparse.ArgumentParser()
parser.add_argument("real", type=float)
parser.add_argument("imag", type=float)
parser.add_argument("max_iters", type=int)
args = parser.parse_args()

iterations = mandy.sample.point(args.real, args.imag, args.max_iters)

print(f"({args.real}, {args.imag}) -> {iterations}")

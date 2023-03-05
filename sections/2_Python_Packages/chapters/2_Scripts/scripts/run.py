import argparse
import mandy

parser = argparse.ArgumentParser()
parser.add_argument("real", type=float)
parser.add_argument("imag", type=float)
args = parser.parse_args()

iterations = mandy.sample.point(args.real, args.imag)

print(f"({args.real}, {args.imag}) -> {iterations}")

"""
Code for shuffling the user inputed dataset 
"""

import argparse
from sklearn.utils import shuffle
import pandas as pd

def add_args(parser):
    """
    Create parser for command line utility.

    :meta private:
    """
    parser.add_argument(
        "--data", help="Dataset to be shuffled", required=True
    )
    parser.add_argument("--seed", help="Choose random seed for shuffling", default=None)
    parser.add_argument("--o", help="Choose where to store the output file", default=None)
    return parser

def main(args):
    """
    Shuffle data stated by user 
    """
    data = args.data
    seed = args.seed
    output_path = args.o
    split_data = data.split("/")

    # Check that the file is a tsv-file
    if not data.endswith('.tsv'):
        return "Please use a tsv-file"
    df = pd.read_table(data,header=None)
    df = shuffle(df, random_state=seed)
    if output_path is None:
        output_path = "shuffled_"+split_data[-1]
    df.to_csv(path_or_buf=output_path, sep='\t', header=False, index=False)
    
if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    add_args(parser)
    main(parser.parse_args())

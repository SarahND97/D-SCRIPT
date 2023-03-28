"""
Code for shuffling the user inputed dataset 
"""

import argparse
#from sklearn.utils import shuffle
import random
import pandas as pd
from Bio import SeqIO
import copy
import numpy as np

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
    parser.add_argument("--command", help="Choose where to store the output file", default="shuffle-data-tsv")
    return parser

def main(args):
    """
    Shuffle data stated by user 
    """
    data = args.data
    seed = (int(args.seed) if args.seed is not None else None) 
    output_path = args.o
    command = args.command
    split_data = data.split("/")

    # Check that the file is a tsv-file
    if not data.endswith('.tsv') and not data.endswith('.fasta'):
        return "Please input  a tsv- or fasta-file"

    if command=="shuffle-data-tsv":
        df = pd.read_table(data,header=None)
        df_1_copy = copy.deepcopy(df[1])
        random.Random(seed).shuffle(df_1_copy)
        if output_path is None:
            output_path = "shuffled_"+split_data[-1]
        df1=pd.DataFrame({"0": df[0], "1": df_1_copy, "2": np.zeros(len(df_1_copy))})
        df2=pd.DataFrame({"0": df[0], "1": df[1], "2": df[2]})
        df = pd.concat([df1,df2])  
        df.to_csv(path_or_buf=output_path, sep='\t', header=False, index=False)
    else:
        if data.endswith('.tsv'):
            df = pd.read_table(data,header=None)
            df_copy = df.copy()
            random.Random(seed).shuffle(df_copy)
            if output_path is None:
                output_path = "shuffled_"+split_data[-1]
            df_copy.to_csv(path_or_buf=output_path, sep='\t', header=False, index=False)
        else: 
            with open(data) as fasta_file: 
                df = []
                for seq_record in SeqIO.parse(fasta_file, 'fasta'):  
                    df.append(seq_record)
                random.Random(seed).shuffle(df)
                if output_path is None:
                    output_path = "shuffled_"+split_data[-1]
                SeqIO.write(df, output_path, "fasta")

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    add_args(parser)
    main(parser.parse_args())


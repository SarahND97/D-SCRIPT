"""
Code for shuffling the user inputed dataset 
"""

import argparse
from sklearn.utils import shuffle
import pandas as pd
from Bio import SeqIO

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
    seed = (int(args.seed) if args.seed is not None else None) # a if condition else b
    output_path = args.o
    split_data = data.split("/")

    # Check that the file is a tsv-file
    if not data.endswith('.tsv') and not data.endswith('.fasta'):
        return "Please input  a tsv- or fasta-file"
    
    if data.endswith('.tsv'):
        df = pd.read_table(data,header=None)
        df_copy = df.copy()
        df = shuffle(df_copy, random_state=seed)
        if output_path is None:
            output_path = "shuffled_"+split_data[-1]
        df.to_csv(path_or_buf=output_path, sep='\t', header=False, index=False)
    else: 
        with open(data) as fasta_file: 
            df = []
            for seq_record in SeqIO.parse(fasta_file, 'fasta'):  
                df.append(seq_record)
            df = shuffle(df, random_state=seed)
            if output_path is None:
                output_path = "shuffled_"+split_data[-1]
            SeqIO.write(df, output_path, "fasta")

                    
    
if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    add_args(parser)
    main(parser.parse_args())


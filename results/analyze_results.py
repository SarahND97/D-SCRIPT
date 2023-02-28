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
    parser.add_argument("--data", help="Dataset to be shuffled", required=True)
    parser.add_argument("--command", help="Choose where to store the output file", default="split")
    return parser

def main(args):
    """
    Shuffle data stated by user 
    """
    data = args.data
    command = args.command
    split_data = data.split("/")
    # Check that the file is a tsv-file
    if not data.endswith('.tsv'):
        return "Please input  a tsv-file"

    df = pd.read_table(data,header=None)
    df_copy = copy.deepcopy(df)

    # TODO: add column to .tsv-file that is 1 if result is above 0.5 and lower if lower than 0.5
    if command=="add threshold":  
        df_copy[4] = np.where(df_copy[3]<0.5, 0, 1)
        
        if output_path is None:
            output_path = "threshold_"+split_data[-1]
        df_copy.to_csv(path_or_buf=output_path, sep='\t', header=False, index=False)
    else: 
        grouped = df_copy.groupby(df[2])
        df1 = grouped.get_group(0.0)
        df2 = grouped.get_group(1.0)
        df1.to_csv(path_or_buf="negative_samples_" + split_data[-1], sep='\t', header=False, index=False)
        df2.to_csv(path_or_buf="positive_samples_" + split_data[-1], sep='\t', header=False, index=False)
        
    
if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    add_args(parser)
    main(parser.parse_args())


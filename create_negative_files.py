"""
Code for shuffling the user inputed dataset 
"""

import argparse
import os
import pandas as pd
from Bio import SeqIO
import copy
import numpy as np

def add_args(parser):
    """
    Create parser for command line utility.

    :meta private:
    """
    parser.add_argument("--datadir", help="Directory to fasta-files", required=True)
    parser.add_argument("--pairs", help="The shuffled pairs", required=True)
    parser.add_argument("--o", help="Choose where to store the output files", default=None)
    return parser

def main(args):
    """
    Shuffle data stated by user 
    """
    datadir = args.datadir
    shuffled_pairs = args.pairs
    outdir = args.o
    
    # split_data = data.split("/")

    # Check that the file is a tsv-file
    if not shuffled_pairs.endswith('.tsv'):
        return "Please input  a tsv-file"

    df = pd.read_table(shuffled_pairs,header=None)
    df_1_copy = copy.deepcopy(df)
    files = os.listdir(datadir)
    # : lists all fasta-files in dir
    i = 0
    for index, row in df_1_copy.iterrows():
        output = ""
        outfile = ""
        if (row[0][:-2]+".fasta") in files:
            with open(datadir+row[0][:-2]+".fasta") as f: contents = f.readlines()
            if contents[0][1:-1] == (row[0] or row[1]):
                output = output+contents[0]+contents[1]
            
        if (row[1][:-2]+".fasta") in files:
            with open(datadir+row[1][:-2]+".fasta") as f: contents = f.readlines()
            if contents[2][1:-1] == row[1]:
                output = output+contents[2]+contents[3]
        
        outfile = outdir + str(row[0][:-2]) + "_" + str(row[1][:-2]) + ".fasta"
        f = open(outfile, "a")
        f.write(output)
        f.close()
        # SeqIO.write(output, outfile, "fasta")
        

      
    
    
    
    
    
    # if output_path is None:
    #     output_path = "shuffled_"+split_data[-1]
    # df1=pd.DataFrame({"0": df[0], "1": df_1_copy, "2": np.zeros(len(df_1_copy))})
    # df2=pd.DataFrame({"0": df[0], "1": df[1], "2": df[2]})
    # df = pd.concat([df1,df2])  
    # df.to_csv(path_or_buf=output_path, sep='\t', header=False, index=False)
    
    
    
    # 

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    add_args(parser)
    main(parser.parse_args())

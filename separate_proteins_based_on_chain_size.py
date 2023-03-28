"""
Code for separating larger protein chains
"""
import argparse
import os
import pandas as pd

def add_args(parser):
    """
    Create parser for command line utility.

    :meta private:
    """
    parser.add_argument("--fasta_dir", help="Directory for fasta-files of file that contains names of fasta-files", required=True)
    # parser.add_argument("--pdb_dir", help="Directory for structures", required=True)
    parser.add_argument("--outfile", help="Name of outfile", required=True)
    return parser

def split_file(file_):
    if "/" in file_:
        file_=file_.split("/")[-1]
    return file_.split(".")[0]

def main(args):
    """
    Shuffle data stated by user 
    """
    f_dir = args.fasta_dir
    out = args.outfile

    f_files = [f for f in os.listdir(f_dir) if os.path.isfile(os.path.join(f_dir, f))]
    f_names = list(map(split_file,f_files))
    
    # One file anything smaller than 500
    f_small = open(out+"_small_chains.txt", "w")
    # One file for between 500-1000
    f_medium = open(out+"_medium_chains.txt", "w")
    # One file for larger than 1000
    f_large = open(out+"_large_chains.txt", "w")
    for i in range(len(f_files)):#f_file in f_files:
        fasta_file = open(f_dir+f_files[i],"r")
        lines = fasta_file.readlines()
        if len(lines[1])<650:
            f_small.write(f_names[i]+"\n")
        elif 650<len(lines[1])<1000:
            f_medium.write(f_names[i]+"\n")
        elif 1000<len(lines[1])<3000:
            f_large.write(f_names[i]+"\n")
    f_small.close()
    f_medium.close()
    f_large.close()

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    add_args(parser)
    main(parser.parse_args())


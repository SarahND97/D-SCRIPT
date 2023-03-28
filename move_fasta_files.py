"""
Code for separating into homomers and heteromers
"""
import argparse
import os
import pandas as pd
import shutil

def add_args(parser):
    """
    Create parser for command line utility.

    :meta private:
    """
    parser.add_argument("--fasta_dir", help="Directory for fasta-files of file that contains names of fasta-files", required=True)
    parser.add_argument("--missing_list", help="File that specifies hte files that should be moved", required=True)
    # parser.add_argument("--heteromer_list", help="File that lists heteromers present", required=True)
    parser.add_argument("--dest", help="Directory for structures", required=True)
    return parser

def split_file(file_):
    if "/" in file_:
        file_=file_.split("/")[-1]
    return file_.split(".")[0]

def get_file_content(list_):
    # Check whether f_dir is a txt-file or csv-file
    if list_.endswith('.txt'):
        f = open(list_,"r")
        lines = f.readlines()
        f_files = [f.strip("\n") for f in lines]
        # f_files = [f.strip("B") for f in f_files]
        f.close()
    elif list_.endswith('.csv'):
        df = pd.read_table(list_,header=None)
        f_files = list(df[0])
    return f_files

def main(args):
    f_dir = args.fasta_dir
    dest = args.dest
    mi_list = args.missing_list
    
    # get all fasta-files in the given directory
    f_files = [f for f in os.listdir(f_dir) if os.path.isfile(os.path.join(f_dir, f))]
    f_names = list(map(split_file,f_files))
    # Get the missing files
    mi_files = get_file_content(mi_list)
    
    # Separate homomers and heteromers into two separate directories
    for i in range(len(mi_files)):
        shutil.move(f_dir+"/"+mi_files[i]+".fasta", dest+"/"+mi_files[i]+".fasta")
        
if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    add_args(parser)
    main(parser.parse_args())


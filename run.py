from buildlib import run

if run(False) == None:
    print("Running RostOS failed!")
    exit(-1)
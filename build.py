#!/usr/bin/env python
from buildlib import build

if build() == None:
    print("Failed to compile RostOS!")
    exit(-1)

print("RostOS successfully compiled!")

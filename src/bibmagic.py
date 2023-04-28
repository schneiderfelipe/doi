#!/usr/bin/env python

import sys
import bibtexparser as btp
from bibtexparser.bparser import BibTexParser
from bibtexparser.customization import homogenize_latex_encoding

with open(sys.argv[1]) as bib_file:
    parser = BibTexParser()
    # parser.customization = homogenize_latex_encoding
    bib_db = btp.load(bib_file, parser=parser)

if len(sys.argv) > 2:
    with open(sys.argv[2], 'w') as bib_file:
        btp.dump(bib_db, bib_file)
else:
    print(btp.dumps(bib_db))

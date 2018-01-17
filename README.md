# doi2bib

Convert [Digital Object Identifier (DOI)](https://www.doi.org) to
[BibTeX](http://www.bibtex.org) using
[crosscite.org](https://citation.crosscite.org/).
DOIs are received and the corresponding BibTeX entries are output.

Inspired by <https://gist.github.com/mlund/4609288> and
<http://www.doi2bib.org/>.

**October 16, 2017**: <http://www.doi2bib.org/> seems not to be working
any more.

## Usage

    $ doi2bib 10.1021/la203078w 10.1103/PhysRev.140.A1133

    @article{Cabaleiro_Lago_2012,
     title={The Effect of Nanoparticles on Amyloid Aggregation Depends on the Protein Stability and Intrinsic Aggregation Rate},
     volume={28},
     ISSN={1520-5827},
     url={http://dx.doi.org/10.1021/la203078w},
     DOI={10.1021/la203078w},
     number={3},
     journal={Langmuir},
     publisher={American Chemical Society (ACS)},
     author={Cabaleiro-Lago,
     C. and Szczepankiewicz,
     O. and Linse,
     S.},
     year={2012},
     month={Jan},
     pages={1852–1857}
    }

    @article{Kohn_1965,
     title={Self-Consistent Equations Including Exchange and Correlation Effects},
     volume={140},
     ISSN={0031-899X},
     url={http://dx.doi.org/10.1103/PhysRev.140.A1133},
     DOI={10.1103/physrev.140.a1133},
     number={4A},
     journal={Physical Review},
     publisher={American Physical Society (APS)},
     author={Kohn,
     W. and Sham,
     L. J.},
     year={1965},
     month={Nov},
     pages={A1133–A1138}
    }

## Installation

After downloading the tarball and using `cd` to go where the files are, simply do a `sudo make install` and everything will be installed.

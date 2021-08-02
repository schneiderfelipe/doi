[![DOI](https://zenodo.org/badge/67431697.svg)](https://zenodo.org/badge/latestdoi/67431697)

# doi2bib and friends

This repository provides scripts for converting from either [Digital Object Identifier (DOI)](https://www.doi.org) or [Uniform Resource Locator (URL)](https://en.wikipedia.org/wiki/URL) to [BibTeX](http://www.bibtex.org), or from DOI to plain text.

<p align="center">
  <img alt="ASCII video" src="cast.svg">
</p>

## doi2bib

Convert [Digital Object Identifier (DOI)](https://www.doi.org) to
[BibTeX](http://www.bibtex.org) using
[crosscite.org](https://citation.crosscite.org/).
DOIs are received and the corresponding BibTeX entries are output.

Inspired by <https://gist.github.com/mlund/4609288> and
<http://www.doi2bib.org/>.

**October 16, 2017**: <http://www.doi2bib.org/> seems not to be working
any more.

## doi2text

Convert [Digital Object Identifier (DOI)](https://www.doi.org) to plain text
using [crosscite.org](https://citation.crosscite.org/). DOIs are received and
the corresponding text entries are output. In theory, all the styles
available at <https://github.com/citation-style-language/styles> can be used.

## url2bib

Convert [Uniform Resource Locator (URL)](https://en.wikipedia.org/wiki/URL) to
[BibTeX](http://www.bibtex.org) using pure
[Bash](https://en.wikipedia.org/wiki/Bash_(Unix_shell)).
URLs are received and the corresponding BibTeX entries are output.

(This is made to be used exactly as one would use doi2bib.)

## Installation

After downloading the tarball and using `cd` to go where the files are, simply do a `sudo make install` and everything will be installed.

## Usage

    $ doi2bib 10.1021/acs.jpcc.8b06244 10.1021/acscatal.9b00210                                
    @article{Schneider_2018,
      title={How Do Secondary Phosphine Oxides Interact with Silver Nanoclusters? Insights from Computation},
      volume={122},
      ISSN={1932-7455},
      url={http://dx.doi.org/10.1021/acs.jpcc.8b06244},
      DOI={10.1021/acs.jpcc.8b06244},
      number={37},
      journal={The Journal of Physical Chemistry C},
      publisher={American Chemical Society (ACS)},
      author={Schneider, Felipe S. S. and Segala, Maximiliano and Caramori, Giovanni F. and da Silva, Eder Henrique and Parreira, Renato L. T. and Schrekker, Henri S. and van Leeuwen, Piet W. N. M.},
      year={2018},
      month={Aug},
      pages={21449–21461}
    }

    @article{Coelho_2019,
      title={Mechanism of Palladium(II)-Mediated Uncaging Reactions of Propargylic Substrates},
      volume={9},
      ISSN={2155-5435},
      url={http://dx.doi.org/10.1021/acscatal.9b00210},
      DOI={10.1021/acscatal.9b00210},
      number={5},
      journal={ACS Catalysis},
      publisher={American Chemical Society (ACS)},
      author={Coelho, Sara E. and Schneider, Felipe S. S. and de Oliveira, Daniela C. and Tripodi, Guilherme L. and Eberlin, Marcos N. and Caramori, Giovanni F. and de Souza, Bernardo and Domingos, Josiel B.},
      year={2019},
      month={Mar},
      pages={3792–3799}
    }

    $ url2bib google.com http://schneiderfelipe.xyz/ https://github.com/schneiderfelipe/doi2bib
    @misc{google,
      title = {Google},
      howpublished = {\url{google.com}},
      note = {Accessed: 2020-01-27}
    }

    @misc{felipe-s-s-schneider,
      title = {Felipe S. S. Schneider},
      howpublished = {\url{http://schneiderfelipe.xyz/}},
      note = {Accessed: 2020-01-27}
    }

    @misc{github-schneiderfelipe-doi2bib-convert-dois-and-urls-to-bibtex,
      title = {GitHub - schneiderfelipe/doi2bib: Convert DOIs and URLs to BibTeX},
      howpublished = {\url{https://github.com/schneiderfelipe/doi2bib}},
      note = {Accessed: 2020-01-27}
    }

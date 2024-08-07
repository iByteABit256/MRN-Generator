# MRN-Generator

<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
  </ol>
</details>

<!-- ABOUT THE PROJECT -->
## About The Project

This is a command line utility to generate MRNs conforming to [ISO 6346](https://en.wikipedia.org/wiki/ISO_6346)

There is also a [GUI version](https://github.com/iByteABit256/MRN-Generator-UI) available.

<!-- GETTING STARTED -->
## Getting Started

### Prerequisites

To build the project locally you will need to [Install Rust](https://www.rust-lang.org/tools/install)

### Installation

```cargo build``` for a development build

```cargo build --release``` for a release build

<!-- USAGE EXAMPLES -->
## Usage

mrn-generator [OPTIONS] --country-code <COUNTRY_CODE>

Options:
- -c, --country-code <COUNTRY_CODE>              Country code of MRN
- -n, --number-of-mrns <NUMBER_OF_MRNS>          Number of MRNs to generate [default: 1]
- -p, --procedure-category <PROCEDURE_CATEGORY>  Procedure category
- -C, --combined \<COMBINED\>                      Combined procedure category
- -o, --declaration-office <DECLARATION_OFFICE>  Customs office of declaration
- -h, --help                                     Print help
- -V, --version                                  Print version
  
### Examples
```mrn-generator -c DK``` to generate an MRN with Denmark as a country code

```mrn-generator -c DK -o 004700``` to generate an MRN with Denmark as a country code and 004700
as the declaration office

```mrn-generator -c NL -n 20``` to generate 20 MRNs with Netherlands as a country code

```mrn-generator -c NL -n 20 -p B1``` to generate 20 MRNs with Netherlands as a country code
and B1 procedure category

```mrn-generator -c NL -n 20 -p B1 -C A``` to generate 20 MRNs with Netherlands as a country code
and B1 procedure category combined with A* procedure category

<!-- LICENSE -->
## License

Distributed under the MIT License. See `LICENSE.txt` for more information.

<!-- CONTACT -->
## Contact

Pavlos Smith - paulsmith4561+at+gmail.com

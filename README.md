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

<!-- GETTING STARTED -->
## Getting Started

### Prerequisites

To build the project locally you will need to [Install Rust](https://www.rust-lang.org/tools/install)

### Installation

```cargo build``` for a development build

```cargo build --release``` for a release build

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- USAGE EXAMPLES -->
## Usage

mrn-generator [OPTIONS] --country-code <COUNTRY_CODE>

Options:
- -c, --country-code <COUNTRY_CODE>      Country code of MRN
- -n, --number-of-mrns <NUMBER_OF_MRNS>  Number of MRNs to generate [default: 1]
- -h, --help                             Print help
- -V, --version                          Print version
  
### Examples
```mrn-generator -c DK``` to generate an MRN with Denmark as a country code

```mrn-generator -c NL -n 20``` to generate 20 MRNs with Netherlands as a country code

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- LICENSE -->
## License

Distributed under the MIT License. See `LICENSE.txt` for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- CONTACT -->
## Contact

Pavlos Smith - paulsmith4561@gmail.com.com

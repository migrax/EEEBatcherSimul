HystEEE
=======

[![Build Status](https://travis-ci.org/migrax/EEEBuncherSimul.svg?branch=master)](https://travis-ci.org/migrax/EEEBuncherSimul)

A Simulator the Buncher Algorithm for Actual EEE Interfaces 

## Overview

This is an implementation of the buncher algorithm designed for taking advantage of the
usual configuration parameters of EEE selected by comercial vendors.

## USAGE:
    buncher [OPTIONS]

### FLAGS:
        --help       Prints help information
    -V, --version    Prints version information

### OPTIONS:
    -c, --capacity <capacity>    Sets the output link capacity in Gb/s [default: 10]
    -d, --delay <delay>          Delay for the first packet in the burst in ns. [default: 0]

## Legal

Copyright ⓒ 2018 Miguel Rodríguez Pérez <miguel@det.uvigo.gal>.

This simulator is licensed under the GNU General Public License, version 3 (GPL-3.0). For information see LICENSE

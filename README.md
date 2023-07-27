# `osmchangesets2csv`

Convert OpenStreetMap Changeset files to CSV files


## Getting data

Changeset file can be downloaded at: [https://planet.openstreetmap.org/planet/changesets-latest.osm.bz2](https://planet.openstreetmap.org/planet/changesets-latest.osm.bz2)

Download a changeset dump (~6 GiB)
```sh
$ aria2c --seed-time 0 https://planet.openstreetmap.org/planet/changesets-latest.osm.bz2.torrent
```

Parse & read it:


## Installation

	cargo install osmchangesets2csv

## Usage

	osmchangesets2csv

### Column formats

## Todo

This software isn't finished, here's what I'd like to add. Feel free to send a patch

* Write to compressed output (Gzip & Bzip2). Also in a separate thread too
* Read (& uncompress) input file in a separate thread. This multiprocessing
  should make things faster.
* Query the OSM.org servers for current status, and update the CSV. e.g. close
  exisiting changesets, and add new one. The Dump is only once a week.


## Examples of it being used

* Your project here!


## Misc

Copyright 2022, GNU Affero General Public Licence (AGPL) v3 or later.
Source code is on [Github](https://github.com/amandasaurus/osmchangeset2csv).

The output file should be viewed as a Derived Database of the OpenStreetMap database, and hence under the [ODbL 1.0](https://opendatacommons.org/licenses/odbl/) licence, the same as the [OpenStreetMap copyright](https://www.openstreetmap.org/copyright)

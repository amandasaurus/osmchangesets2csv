# `osmchangesets2csv`

Convert OpenStreetMap Changeset files to CSV files


## Getting data

Changeset file can be downloaded at: [https://planet.openstreetmap.org/planet/changesets-latest.osm.bz2](https://planet.openstreetmap.org/planet/changesets-latest.osm.bz2)

Download a changeset dump (~6 GiB as of July 2023)
```sh
$ aria2c --seed-time 0 https://planet.openstreetmap.org/planet/changesets-latest.osm.bz2.torrent
```

## Installation

	cargo install osmchangesets2csv

## Usage

	osmchangesets2csv -i ~/osm-data/changesets-230717.osm.bz2 -o /dev/stdout

As of July 2023, this takes about 30 minutes to run on my desktop computer.


### Sample output

changeset_id  |  created               |  closed                |  uid     |  user         |  open   |  num_changes  |  comments_count  |  created_by          |  comment                            |  source  |  imagery_used
--------------|------------------------|------------------------|----------|---------------|---------|---------------|------------------|----------------------|-------------------------------------|----------|--------------
1000006       |  2009-04-28T07:29:57Z  |  2009-04-28T08:29:57Z  |  33757   |  Minh Nguyen  |  false  |  35           |  0               |  Potlatch 0.11       |                                     |          |
1000007       |  2009-04-28T07:30:00Z  |  2009-04-28T07:30:02Z  |  105730  |  Wetzi        |  false  |  8            |  0               |  JOSM                |  meine falsche Eintragung entfernt  |          |
1000008       |  2009-04-28T07:30:01Z  |  2009-04-28T08:30:01Z  |  9250    |  j3m          |  false  |  0            |  0               |  Potlatch 0.11       |                                     |          |
1000009       |  2009-04-28T07:30:07Z  |  2009-04-28T07:30:09Z  |  35074   |  mikeltxo     |  false  |  8            |  0               |  JOSM/1.5 (1546 en)  |  Eibar                              |          |
1000010       |  2009-04-28T07:30:07Z  |  2009-04-28T07:30:19Z  |  28145   |  amillar      |  false  |  26           |  0               |  JOSM/1.5 (1561 en)  |  tiger cleanup                      |          |
1000011       |  2009-04-28T07:30:21Z  |  2009-04-28T07:30:22Z  |  7008    |  elsevilla    |  false  |  2            |  0               |  JOSM                |  modif-4tcinturo                    |          |
1000012       |  2009-04-28T07:30:22Z  |  2009-04-28T08:30:22Z  |  45347   |  eriosw       |  false  |  14           |  0               |  Potlatch 0.11       |                                     |          |
1000013       |  2009-04-28T07:30:24Z  |  2009-04-28T08:30:24Z  |  33640   |  kaakeli      |  false  |  0            |  0               |  Potlatch 0.11       |                                     |          |
1000014       |  2009-04-28T07:30:33Z  |  2009-04-28T07:30:38Z  |  24748   |  mabapla      |  false  |  25           |  0               |  JOSM/1.5 (1561 de)  |  Feldwege südl. Mittelstadt         |          |
1000015       |  2009-04-28T07:30:37Z  |  2009-04-28T08:30:37Z  |  14022   |  ressu        |  false  |  0            |  0               |  Potlatch 0.11       |                                     |          |

### Column formats

Control column output with `-c`/`--columns`.

TBC

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

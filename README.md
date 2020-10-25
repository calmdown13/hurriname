![build-linux action status](https://github.com/calmdown13/hurriname/workflows/Continuous%20Integration/badge.svg)

# Hurriname
"What was different about v11.4.16 again?", dicussing different software versions can be a real of pain. If you're lucky the developers might remember the particularities of each version, however, meetings are still going to be a jumbled mess of numbers. Inspired by hurricane naming conventions, hurriname is a simple command line tool to randomly generate human names for versioning.

## Usage
To generate a random name simply invoke hurriname:
```
$ hurriname
tamantha
```
To generate a capitlized name:
```
$ hurriname --capitalize
Navana
```
To generate a name that starts with a particular letter:
```
$ hurriname --letter c
callum
```
To generate a name that starts with the next letter in the alphabet (loops back to `a` if `z`):
```
$ hurriname --previous-letter c
dathol
```
To generate a (predominantly) female name:
```
$ hurriname --sex female
sharrone
```
To generate a (predominantly) male name:
```
$ hurriname --sex male
meric
```
To customise the returned name format you can pass a string format, using the `name` keyword:
```
$ hurriname --format hurricane-{name}
hurricane-melita
```
If you need uniqueness to be guaranteed, you can use the `uuid` keyword to insert a uuidv4:
```
$ hurriname --format hurricane-{name}-{uuid}
hurricane-blyss-cc5f67d9-a81f-4a8d-96cf-c7c4c8987ff0
```
If you need the date to be recorded, you can use the `date` keyword to insert the date:
```
$ hurriname --format hurricane-{name}-{date}
hurricane-annel-20201025
```
If you need the date and time to be recorded, you can use the `datetime` keyword to insert the date and time:
```
$ hurriname --format hurricane-{name}-{datetime}
hurricane-anyea-2020102515455
```

## Names
The names used in hurriname are parsed from the US Social Security Administration's Baby Names [dataset](https://catalog.data.gov/dataset/baby-names-from-social-security-card-applications-national-level-data). In total hurriname contains 99444 unique names, 62670 female names and 36774 male names (the US population must be more creative when naming their daughters). Names were judged to be female if more females had that name than males, and vice versa.

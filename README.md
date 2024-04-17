# 1️⃣🐝🏎️ The One Billion Row Challenge

The One Billion Row Challenge (1BRC) is a fun exploration of how far modern languages can be pushed for aggregating one billion rows from a text file.
Grab all your (virtual) threads, reach out to SIMD, optimize your GC, or pull any other trick, and create the fastest implementation for solving this task!

The text file contains temperature values for a range of weather stations.
Each row is one measurement in the format `<string: station name>;<double: measurement>`, with the measurement value having exactly one fractional digit.
The following shows ten rows as an example:

```
Hamburg;12.0
Bulawayo;8.9
Palembang;38.8
St. John's;15.2
Cracow;12.6
Bridgetown;26.9
Istanbul;6.2
Roseau;34.4
Conakry;31.2
Istanbul;23.0
```

The task is to write a program which reads the file, calculates the min, mean, and max temperature value per weather station, and emits the results on stdout like this
(i.e. sorted alphabetically by station name, and the result values per station in the format `<min>/<mean>/<max>`, rounded to one fractional digit):

```
{Abha=-23.0/18.0/59.2, Abidjan=-16.2/26.0/67.3, Abéché=-10.0/29.4/69.0, Accra=-10.1/26.4/66.4, Addis Ababa=-23.7/16.0/67.0, Adelaide=-27.8/17.3/58.5, ...}
```

# Results

The file `data/measurements.txt`, containing 1 billion rows, is not included as the size of the size is >12GB.
A sample file is provided for convenience in the repo but results found here were run against `data/measurements.txt` file.

To generate a real file, head over to the initial repo: https://github.com/gunnarmorling/1brc

| Try # | Time     | Human readable time |
|-------|----------|---------------------|
| 1     | 172363ms | ~2m52s              |

For reference score obtained through community solutions to get a time baseline on my hardware (reading only no printing).

| Try # | Time     | Human readable time | Name                   |
|-------|----------|---------------------|------------------------|
| 1     | 31459ms  | ~31s                | Butch78 w/ polars      |
| 2     | 121978ms | ~2m02s              | aminediro w/ bufreader |




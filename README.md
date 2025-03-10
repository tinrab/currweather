# currweather

A CLI tool to display the current weather information in your terminal.

It uses [ipify.org](https://www.ipify.org/) to get the IP, [ip-api.com](https://ip-api.com) to get geoip, and the [Open-Meteo API](https://open-meteo.com) for weather data.

## Installation

Install `currweather` from GitHub:

```sh
cargo install --git https://github.com/tinrab/currweather
```

## Usage

```text
Usage: currweather [OPTIONS]

Options:
  -t, --temperature           Display only the temperature
  -u, --humidity              Display only the relative humidity
  -a, --apparent-temperature  Display only the apparent temperature
  -n, --nightorday            Show `0` for night and `1` for day
  -p, --precipitation         Display only the precipitation
  -r, --rain                  Display only the rain
  -s, --showers               Display only the showers
  -w, --snowfall              Display only the snowfall
  -h, --help                  Print help
  -V, --version               Print version
```

Run the `currweather` command in your terminal:

```bash
$ currweather
Location: Ljubljana, Slovenia
Time: 2025-03-10T03:30 (Night)
Temperature: 9.6 °C
Relative Humidity: 77 %
Apparent Temperature: 8.1
Precipitation: 0 mm
Rain: 0 mm
Showers: 0 mm
Snowfall: 0 cm
```

You can also display only a specific value:

```bash
$ currweather --temperature
10.1
```

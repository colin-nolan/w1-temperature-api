# 1-Wire Temperature API

A Rust-based API server to provide temperature readings from a w1 device, such as a DS18B20 sensor.

## Usage

### Service

```
Usage: w1-temperature-api [OPTIONS] <DEVICE_ID>

Arguments:
  <DEVICE_ID>  ID of the w1 device to supply temperature for

Options:
  -v, --verbose...             Increase logging verbosity
  -q, --quiet...               Decrease logging verbosity
  -i, --interface <INTERFACE>  Interface to run server on [default: 127.0.0.1]
  -p, --port <PORT>            Port to run server on [default: 8080]
  -h, --help                   Print help
  -V, --version                Print version
```

### CLI Tool

```
Usage: w1-temperature [OPTIONS] <DEVICE_ID>

Arguments:
  <DEVICE_ID>  ID of the w1 device to supply temperature for

Options:
  -v, --verbose...  Increase logging verbosity
  -q, --quiet...    Decrease logging verbosity
  -h, --help        Print help
  -V, --version     Print version
```

## Legal
AGPL v3.0 (contact for other licencing). Copyright 2024 Colin Nolan.

This work is in no way related to any company that I work for.

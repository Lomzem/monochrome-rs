# monochrome-rs

A simple command line tool to monochrome images to a certain color.

## Usage

See for full list of options:
```bash
monochrome-rs --help
```

The tool supports both `HSL` and `HEX` input

### HSL
Note: Light will automatically use the input image's lightness value
```bash
monochrome-rs -i <INPUT> hsl --hue <HUE> --saturation <SATURATION>
```

### HEX
```bash
monochrome-rs -i <INPUT> hex <HEX>
```

Optionally, you could also specify a saturation value:
```bash
monochrome-rs -i <INPUT> hex <HEX> --saturation <SATURATION>
```

## Examples

##### Base Image:
![Base Itachi](./assets/itachi.png)

##### RGB
With this input color: ![](./assets/9ccfd8_dot.svg)

```bash
monochrome-rs -i itachi.png hex '#9ccfd8'
```

![Output Itachi #9ccfd8](./assets/itachi_9ccfd8.png)

##### HSL
With this input hue: ![](./assets/267_20_dot.svg)
```bash
monochrome-rs -i itachi.png hsl --hue 267 --saturation 20
```

![Output Itachi ](./assets/itachi_267_57.png)

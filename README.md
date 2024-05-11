# honeytree-calc
Munchlax honey tree calculator for command-line.

## Usage

To use the program interactively, execute it as a command like this:
```
honeytree-calc
```

To use the program with command line arguments, execute it as a command like this:
```
honeytree-calc-cli
```

After providing the trainer ID and secret ID, the program will output the Munchlax tree locations.

### Interactive example output

```
DPPt Honey Tree Calculator
This program calculates the Munchlax honey trees based on the trainer's ID an SID.

Input your trainer ID (number between 0 and 65535): 12345
Input your trainer secret ID (number between 0 and 65535): 54321

Munchlax honey trees for TID 12345 and SID 54321 can be found in the following locations:
        Route 210 north
        Route 206
        Route 221
        Route 210 south
```

### CLI example output

```
> honeytree-calc-cli 12345 54321
Munchlax honey trees for TID 12345 and SID 54321 can be found in the following locations:
        Route 210 north
        Route 206
        Route 221
        Route 210 south
```

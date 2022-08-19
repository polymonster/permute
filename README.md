# permute

Generates permutation combinations for all inputs.

## Usage

```shell
cargo build
./permute input_file.toml
```

## Defining Inputs

json is used define inputs which permutations can be generated from:

```json
"channel_map": {
    "v2-v2": [
        [ {"name": "x", "index": 0}, {"name": "y", "index": 1} ], 
        [ {"name": "x", "index": 0}, {"name": "y", "index": 1} ]
    ],

    "v3-v2": [
        [ {"name": "x", "index": 0}, {"name": "y", "index": 1}, {"name": "z", "index": 2} ], 
        [ {"name": "x", "index": 0}, {"name": "y", "index": 1}, {"name": "z", "index": 2} ]
    ]
},
```

Supply an array of map of channels, each element is called an option, you can then specify outputs which use an entry in the channel map:

```json
"outputs": [
    {
        "channels": "v2-v2",
        "format_string": "Swizzle<T, 2, %d, %i[0], %i[1]> %n[0]%n[1];", 
        "ignore_duplicates": false,
        "include_least_index": 0
    }
]
```

`format_string` allows you specify a string which will be formatted on output:

`%i` replaces with option index and `%n` replaces with the option name, the array `%i[1]` syntax is to select which channel the option comes from. `%d` will be replaced with 0 if none of the indices in the result are duplicated, 1 if there are duplicates in the result.

`ignore_duplicates` ensure each element inside the permutation is unique.  
`include_least_index` allows you to select permuatation combination containing element that is this value or greater.

## Examples
### Generate all permutations of vector swizzles 

To perform shader-style swizzles with c++ requires some templates and leg-work.. here are all the generated c++ templated declarations of a vec4 to a vec2 swizzle.

```
Swizzle<T, 0, 0> xx;
Swizzle<T, 0, 1> xy;
Swizzle<T, 0, 2> xz;
Swizzle<T, 0, 3> xw;
Swizzle<T, 1, 0> yx;
Swizzle<T, 1, 1> yy;
Swizzle<T, 1, 2> yz;
Swizzle<T, 1, 3> yw;
Swizzle<T, 2, 0> zx;
Swizzle<T, 2, 1> zy;
Swizzle<T, 2, 2> zz;
Swizzle<T, 2, 3> zw;
Swizzle<T, 3, 0> wx;
Swizzle<T, 3, 1> wy;
Swizzle<T, 3, 2> wz;
Swizzle<T, 3, 3> ww;
```

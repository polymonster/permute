# permute

Generates permutation combinations for all inputs.

## Usage

```shell
cargo build
./permute <filename list>
```

## Generate all permutations of vector swizzles 

To perform shader-style swizzles with c++ requires some templates and leg-work.. here are all the generated swizzlws of a vec4 to a vec2.

```shell
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

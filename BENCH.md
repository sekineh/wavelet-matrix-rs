
running 6 tests
test rsdic_simple::tests::rsdic_sanity ... ignored
test wavelet_matrix::tests::example ... ignored
test wavelet_matrix::tests::layers_4 ... ignored
test wavelet_matrix::tests::layers_64 ... ignored
test wavelet_matrix::tests::layers_7 ... ignored
test wavelet_matrix::tests::wavelet_matrix_sanity ... ignored

test result: ok. 0 passed; 0 failed; 6 ignored; 0 measured; 0 filtered out

# Overall Performance

## 16-bit Values

### N = 10000000

```
    WaveletMatrix::new(), N = 10000000, 16-bit values: 4841399578 ns   (R²=1.977, 8 iterations in 6 samples)
               .lookup(), N = 10000000, 16-bit values:       1769 ns   (R²=0.991, 575638 iterations in 114 samples)
                   vec[], N = 10000000, 16-bit values:          5 ns   (R²=0.994, 159340873 iterations in 173 samples)
                 .rank(), N = 10000000, 16-bit values:       3130 ns   (R²=0.997, 357422 iterations in 109 samples)
               .select(), N = 10000000, 16-bit values:      36551 ns   (R²=0.988, 27250 iterations in 82 samples)
                .count(), N = 10000000, 16-bit values:       3147 ns   (R²=0.997, 357422 iterations in 109 samples)
         .count_prefix(), N = 10000000, 16-bit values:         43 ns   (R²=0.995, 19574375 iterations in 151 samples)
        .search().next(), N = 10000000, 16-bit values:      42369 ns   (R²=0.996, 24771 iterations in 81 samples)
 .search_prefix().next(), N = 10000000, 16-bit values:      25353 ns   (R²=0.994, 43895 iterations in 87 samples)
```

### N = 1000000

```
    WaveletMatrix::new(), N = 1000000, 16-bit values:  472959782 ns   (R²=0.126, 10 iterations in 7 samples)
               .lookup(), N = 1000000, 16-bit values:       1470 ns   (R²=0.994, 696525 iterations in 116 samples)
                   vec[], N = 1000000, 16-bit values:          5 ns   (R²=0.998, 175274961 iterations in 174 samples)
                 .rank(), N = 1000000, 16-bit values:       3165 ns   (R²=0.996, 324928 iterations in 108 samples)
               .select(), N = 1000000, 16-bit values:      33044 ns   (R²=0.995, 29976 iterations in 83 samples)
                .count(), N = 1000000, 16-bit values:       3163 ns   (R²=0.997, 324928 iterations in 108 samples)
         .count_prefix(), N = 1000000, 16-bit values:         53 ns   (R²=0.997, 16177167 iterations in 149 samples)
        .search().next(), N = 1000000, 16-bit values:      37674 ns   (R²=0.994, 27250 iterations in 82 samples)
 .search_prefix().next(), N = 1000000, 16-bit values:      16486 ns   (R²=0.999, 64273 iterations in 91 samples)
```

### N = 100000

```
    WaveletMatrix::new(), N = 100000, 16-bit values:   40373242 ns   (R²=0.994, 23 iterations in 12 samples)
               .lookup(), N = 100000, 16-bit values:       1314 ns   (R²=0.997, 766179 iterations in 117 samples)
                   vec[], N = 100000, 16-bit values:          5 ns   (R²=0.998, 175274961 iterations in 174 samples)
                 .rank(), N = 100000, 16-bit values:       3170 ns   (R²=0.997, 324928 iterations in 108 samples)
               .select(), N = 100000, 16-bit values:      25763 ns   (R²=0.999, 39903 iterations in 86 samples)
                .count(), N = 100000, 16-bit values:       3069 ns   (R²=0.993, 324928 iterations in 108 samples)
         .count_prefix(), N = 100000, 16-bit values:         40 ns   (R²=0.997, 21531814 iterations in 152 samples)
        .search().next(), N = 100000, 16-bit values:      29411 ns   (R²=0.998, 36274 iterations in 85 samples)
 .search_prefix().next(), N = 100000, 16-bit values:      17130 ns   (R²=0.997, 58429 iterations in 90 samples)
```

### N = 10000

```
    WaveletMatrix::new(), N = 10000, 16-bit values:    4173016 ns   (R²=0.983, 243 iterations in 33 samples)
               .lookup(), N = 10000, 16-bit values:       1923 ns   (R²=0.996, 523306 iterations in 113 samples)
                   vec[], N = 10000, 16-bit values:          5 ns   (R²=0.997, 159340873 iterations in 173 samples)
                 .rank(), N = 10000, 16-bit values:       3167 ns   (R²=0.990, 324928 iterations in 108 samples)
               .select(), N = 10000, 16-bit values:      24799 ns   (R²=0.992, 39903 iterations in 86 samples)
                .count(), N = 10000, 16-bit values:       2925 ns   (R²=0.997, 357422 iterations in 109 samples)
         .count_prefix(), N = 10000, 16-bit values:         42 ns   (R²=0.997, 21531814 iterations in 152 samples)
        .search().next(), N = 10000, 16-bit values:      21725 ns   (R²=0.996, 48286 iterations in 88 samples)
 .search_prefix().next(), N = 10000, 16-bit values:      13176 ns   (R²=0.997, 77774 iterations in 93 samples)
```

### N = 1000

```
    WaveletMatrix::new(), N = 1000, 16-bit values:     349686 ns   (R²=0.996, 3032 iterations in 59 samples)
               .lookup(), N = 1000, 16-bit values:       1747 ns   (R²=0.999, 633203 iterations in 115 samples)
                   vec[], N = 1000, 16-bit values:          5 ns   (R²=0.992, 159340873 iterations in 173 samples)
                 .rank(), N = 1000, 16-bit values:       2884 ns   (R²=0.997, 357422 iterations in 109 samples)
               .select(), N = 1000, 16-bit values:      15137 ns   (R²=0.954, 70702 iterations in 92 samples)
                .count(), N = 1000, 16-bit values:       3929 ns   (R²=0.917, 295388 iterations in 107 samples)
         .count_prefix(), N = 1000, 16-bit values:         85 ns   (R²=0.944, 11049219 iterations in 145 samples)
        .search().next(), N = 1000, 16-bit values:      19673 ns   (R²=0.982, 48286 iterations in 88 samples)
 .search_prefix().next(), N = 1000, 16-bit values:      12865 ns   (R²=0.986, 85553 iterations in 94 samples)
```

## 32-bit Values

### N = 10000000

```
    WaveletMatrix::new(), N = 10000000, 32-bit values: 10840990320 ns   (R²=14.193, 8 iterations in 6 samples)
               .lookup(), N = 10000000, 32-bit values:       3094 ns   (R²=0.998, 357422 iterations in 109 samples)
                   vec[], N = 10000000, 32-bit values:          5 ns   (R²=0.997, 159340873 iterations in 173 samples)
                 .rank(), N = 10000000, 32-bit values:       6218 ns   (R²=0.998, 166733 iterations in 101 samples)
               .select(), N = 10000000, 32-bit values:      69984 ns   (R²=0.998, 15376 iterations in 76 samples)
                .count(), N = 10000000, 32-bit values:       7650 ns   (R²=0.994, 137793 iterations in 99 samples)
         .count_prefix(), N = 10000000, 32-bit values:       3770 ns   (R²=0.995, 268533 iterations in 106 samples)
        .search().next(), N = 10000000, 32-bit values:      79127 ns   (R²=0.994, 12705 iterations in 74 samples)
 .search_prefix().next(), N = 10000000, 32-bit values:      39942 ns   (R²=0.995, 24771 iterations in 81 samples)
```

### N = 1000000

```
    WaveletMatrix::new(), N = 1000000, 32-bit values: 1018673667 ns   (R²=0.118, 10 iterations in 7 samples)
               .lookup(), N = 1000000, 32-bit values:       4521 ns   (R²=0.989, 244120 iterations in 105 samples)
                   vec[], N = 1000000, 32-bit values:          8 ns   (R²=0.996, 119715154 iterations in 170 samples)
                 .rank(), N = 1000000, 32-bit values:       8991 ns   (R²=0.991, 113876 iterations in 97 samples)
               .select(), N = 1000000, 32-bit values:      67672 ns   (R²=0.994, 15376 iterations in 76 samples)
                .count(), N = 1000000, 32-bit values:       7053 ns   (R²=0.988, 137793 iterations in 99 samples)
         .count_prefix(), N = 1000000, 32-bit values:       3192 ns   (R²=0.996, 324928 iterations in 108 samples)
        .search().next(), N = 1000000, 32-bit values:      57304 ns   (R²=0.996, 18608 iterations in 78 samples)
 .search_prefix().next(), N = 1000000, 32-bit values:      30147 ns   (R²=0.994, 32975 iterations in 84 samples)
```

### N = 100000

```
    WaveletMatrix::new(), N = 100000, 32-bit values:   78463488 ns   (R²=0.993, 44 iterations in 17 samples)
               .lookup(), N = 100000, 32-bit values:       2834 ns   (R²=0.998, 357422 iterations in 109 samples)
                   vec[], N = 100000, 32-bit values:          6 ns   (R²=0.997, 159340873 iterations in 173 samples)
                 .rank(), N = 100000, 32-bit values:       6723 ns   (R²=0.997, 166733 iterations in 101 samples)
               .select(), N = 100000, 32-bit values:      48867 ns   (R²=0.998, 20470 iterations in 79 samples)
                .count(), N = 100000, 32-bit values:       5320 ns   (R²=0.996, 183408 iterations in 102 samples)
         .count_prefix(), N = 100000, 32-bit values:       2689 ns   (R²=0.998, 393165 iterations in 110 samples)
        .search().next(), N = 100000, 32-bit values:      66288 ns   (R²=0.996, 16915 iterations in 77 samples)
 .search_prefix().next(), N = 100000, 32-bit values:      32412 ns   (R²=0.996, 32975 iterations in 84 samples)
```

### N = 10000

```
    WaveletMatrix::new(), N = 10000, 32-bit values:    6310768 ns   (R²=0.993, 180 iterations in 30 samples)
               .lookup(), N = 10000, 32-bit values:       2869 ns   (R²=0.999, 357422 iterations in 109 samples)
                   vec[], N = 10000, 32-bit values:          5 ns   (R²=0.991, 159340873 iterations in 173 samples)
                 .rank(), N = 10000, 32-bit values:       6141 ns   (R²=0.997, 166733 iterations in 101 samples)
               .select(), N = 10000, 32-bit values:      35275 ns   (R²=0.998, 29976 iterations in 83 samples)
                .count(), N = 10000, 32-bit values:       7602 ns   (R²=0.987, 137793 iterations in 99 samples)
         .count_prefix(), N = 10000, 32-bit values:       4388 ns   (R²=0.983, 244120 iterations in 105 samples)
        .search().next(), N = 10000, 32-bit values:      69889 ns   (R²=0.988, 15376 iterations in 76 samples)
 .search_prefix().next(), N = 10000, 32-bit values:      30333 ns   (R²=0.988, 32975 iterations in 84 samples)
```

### N = 1000

```
    WaveletMatrix::new(), N = 1000, 32-bit values:     883040 ns   (R²=0.991, 1161 iterations in 49 samples)
               .lookup(), N = 1000, 32-bit values:       3376 ns   (R²=0.985, 295388 iterations in 107 samples)
                   vec[], N = 1000, 32-bit values:          5 ns   (R²=0.994, 159340873 iterations in 173 samples)
                 .rank(), N = 1000, 32-bit values:       5222 ns   (R²=0.997, 201750 iterations in 103 samples)
               .select(), N = 1000, 32-bit values:      24367 ns   (R²=0.997, 43895 iterations in 87 samples)
                .count(), N = 1000, 32-bit values:       5802 ns   (R²=0.997, 183408 iterations in 102 samples)
         .count_prefix(), N = 1000, 32-bit values:       3746 ns   (R²=0.991, 295388 iterations in 107 samples)
        .search().next(), N = 1000, 32-bit values:      33743 ns   (R²=0.992, 29976 iterations in 83 samples)
 .search_prefix().next(), N = 1000, 32-bit values:      17350 ns   (R²=0.995, 58429 iterations in 90 samples)
```

## 64-bit Values

### N = 10000000

```
    WaveletMatrix::new(), N = 10000000, 64-bit values: 25639948434 ns   (R²=68.467, 8 iterations in 6 samples)
               .lookup(), N = 10000000, 64-bit values:       7951 ns   (R²=0.998, 137793 iterations in 99 samples)
                   vec[], N = 10000000, 64-bit values:          6 ns   (R²=0.998, 159340873 iterations in 173 samples)
                 .rank(), N = 10000000, 64-bit values:      13149 ns   (R²=0.997, 77774 iterations in 93 samples)
               .select(), N = 10000000, 64-bit values:     172539 ns   (R²=0.997, 5919 iterations in 66 samples)
                .count(), N = 10000000, 64-bit values:      15900 ns   (R²=0.997, 64273 iterations in 91 samples)
         .count_prefix(), N = 10000000, 64-bit values:       9760 ns   (R²=0.997, 103522 iterations in 96 samples)
        .search().next(), N = 10000000, 64-bit values:     149714 ns   (R²=0.997, 7165 iterations in 68 samples)
 .search_prefix().next(), N = 10000000, 64-bit values:      76714 ns   (R²=0.997, 13977 iterations in 75 samples)
```

### N = 1000000

```
    WaveletMatrix::new(), N = 1000000, 64-bit values: 1823369224 ns   (R²=0.728, 12 iterations in 8 samples)
               .lookup(), N = 1000000, 64-bit values:       7911 ns   (R²=0.997, 137793 iterations in 99 samples)
                   vec[], N = 1000000, 64-bit values:          7 ns   (R²=0.994, 131686670 iterations in 171 samples)
                 .rank(), N = 1000000, 64-bit values:      17579 ns   (R²=0.996, 64273 iterations in 91 samples)
               .select(), N = 1000000, 64-bit values:     124430 ns   (R²=0.993, 8673 iterations in 70 samples)
                .count(), N = 1000000, 64-bit values:      13352 ns   (R²=0.998, 77774 iterations in 93 samples)
         .count_prefix(), N = 1000000, 64-bit values:       9372 ns   (R²=0.987, 113876 iterations in 97 samples)
        .search().next(), N = 1000000, 64-bit values:     109510 ns   (R²=0.993, 9542 iterations in 71 samples)
 .search_prefix().next(), N = 1000000, 64-bit values:      67121 ns   (R²=0.993, 15376 iterations in 76 samples)
```

### N = 100000

```
    WaveletMatrix::new(), N = 100000, 64-bit values:  178879315 ns   (R²=0.161, 34 iterations in 15 samples)
               .lookup(), N = 100000, 64-bit values:       5980 ns   (R²=0.999, 166733 iterations in 101 samples)
                   vec[], N = 100000, 64-bit values:          6 ns   (R²=0.996, 159340873 iterations in 173 samples)
                 .rank(), N = 100000, 64-bit values:      14792 ns   (R²=0.998, 70702 iterations in 92 samples)
               .select(), N = 100000, 64-bit values:      89046 ns   (R²=0.992, 12705 iterations in 74 samples)
                .count(), N = 100000, 64-bit values:      15292 ns   (R²=0.994, 70702 iterations in 92 samples)
         .count_prefix(), N = 100000, 64-bit values:      10849 ns   (R²=0.995, 94110 iterations in 95 samples)
        .search().next(), N = 100000, 64-bit values:     107483 ns   (R²=0.990, 9542 iterations in 71 samples)
 .search_prefix().next(), N = 100000, 64-bit values:      57545 ns   (R²=0.989, 16915 iterations in 77 samples)
```

### N = 10000

```
    WaveletMatrix::new(), N = 10000, 64-bit values:   10906175 ns   (R²=0.992, 87 iterations in 23 samples)
               .lookup(), N = 10000, 64-bit values:       5504 ns   (R²=0.999, 201750 iterations in 103 samples)
                   vec[], N = 10000, 64-bit values:          5 ns   (R²=0.997, 159340873 iterations in 173 samples)
                 .rank(), N = 10000, 64-bit values:      12097 ns   (R²=0.998, 85553 iterations in 94 samples)
               .select(), N = 10000, 64-bit values:      68467 ns   (R²=0.997, 15376 iterations in 76 samples)
                .count(), N = 10000, 64-bit values:      12371 ns   (R²=0.996, 85553 iterations in 94 samples)
         .count_prefix(), N = 10000, 64-bit values:       9340 ns   (R²=0.998, 113876 iterations in 97 samples)
        .search().next(), N = 10000, 64-bit values:      78876 ns   (R²=0.997, 12705 iterations in 74 samples)
 .search_prefix().next(), N = 10000, 64-bit values:      43988 ns   (R²=0.998, 24771 iterations in 81 samples)
```

### N = 1000

```
    WaveletMatrix::new(), N = 1000, 64-bit values:    1702787 ns   (R²=0.992, 650 iterations in 43 samples)
               .lookup(), N = 1000, 64-bit values:       6476 ns   (R²=0.995, 151574 iterations in 100 samples)
                   vec[], N = 1000, 64-bit values:          5 ns   (R²=0.994, 159340873 iterations in 173 samples)
                 .rank(), N = 1000, 64-bit values:      11239 ns   (R²=0.998, 94110 iterations in 95 samples)
               .select(), N = 1000, 64-bit values:      59259 ns   (R²=0.996, 18608 iterations in 78 samples)
                .count(), N = 1000, 64-bit values:      11306 ns   (R²=0.998, 94110 iterations in 95 samples)
         .count_prefix(), N = 1000, 64-bit values:       9928 ns   (R²=0.997, 103522 iterations in 96 samples)
        .search().next(), N = 1000, 64-bit values:      63026 ns   (R²=0.996, 16915 iterations in 77 samples)
 .search_prefix().next(), N = 1000, 64-bit values:      30232 ns   (R²=0.997, 32975 iterations in 84 samples)
```

# Statistical Performance

## Uniform distribution

For uniform distribution, `.sum()` produce good results using smaller number of `k`.

```
              actual sum, N = 10000, 0 <= val < 256: 1270180
  .sum_expreriment1(k=5), N = 10000, 0 <= val < 256: (1277024, 10000), error = 0.5388212694263805%,       2829 ns   (R²=0.998, 357422 iterations in 109 samples)
  .sum_expreriment2(k=5), N = 10000, 0 <= val < 256: (1277024, 10000), error = 0.5388212694263805%,       3117 ns   (R²=0.996, 324928 iterations in 108 samples)
  .sum_expreriment3(k=5), N = 10000, 0 <= val < 256: (997600..1546449, 10000), error = 0.14517627422884943%,       2918 ns   (R²=0.998, 357422 iterations in 109 samples)
```

```
              actual sum, N = 10000, 0 <= val < 257: 1290840
  .sum_expreriment1(k=5), N = 10000, 0 <= val < 257: (1301408, 10000), error = 0.8186917046264447%,       2214 ns   (R²=0.996, 475732 iterations in 112 samples)
  .sum_expreriment2(k=5), N = 10000, 0 <= val < 257: (1301408, 10000), error = 0.8186917046264447%,       2569 ns   (R²=0.998, 393165 iterations in 110 samples)
  .sum_expreriment3(k=5), N = 10000, 0 <= val < 257: (976704..1616113, 10000), error = 0.4313470298410337%,       2843 ns   (R²=0.995, 393165 iterations in 110 samples)
```

```
              actual sum, N = 10000, 256 <= val < 1024: 6358671
  .sum_expreriment1(k=5), N = 10000, 256 <= val < 1024: (6372992, 10000), error = 0.22522001845983225%,       4062 ns   (R²=0.990, 268533 iterations in 106 samples)
  .sum_expreriment2(k=5), N = 10000, 256 <= val < 1024: (6372992, 10000), error = 0.22522001845983225%,       4801 ns   (R²=0.995, 221926 iterations in 104 samples)
  .sum_expreriment3(k=5), N = 10000, 256 <= val < 1024: (5523840..7212145, 10000), error = 0.14658723497410073%,       4873 ns   (R²=0.995, 201750 iterations in 103 samples)
```

```
              actual sum, N = 10000, 255 <= val < 1024: 6418359
  .sum_expreriment1(k=5), N = 10000, 255 <= val < 1024: (6004928, 10000), error = -6.441381667806366%,       3874 ns   (R²=0.987, 268533 iterations in 106 samples)
  .sum_expreriment2(k=5), N = 10000, 255 <= val < 1024: (6421760, 10000), error = 0.052988622169623106%,       3783 ns   (R²=0.992, 295388 iterations in 107 samples)
  .sum_expreriment3(k=5), N = 10000, 255 <= val < 1024: (5357312..7476209, 10000), error = -0.02491290998213095%,       3612 ns   (R²=0.990, 268533 iterations in 106 samples)
```

```
              actual sum, N = 10000, 256 <= val < 1025: 6424812
  .sum_expreriment1(k=5), N = 10000, 256 <= val < 1025: (6018688, 10000), error = -6.321181071134845%,       2857 ns   (R²=0.995, 357422 iterations in 109 samples)
  .sum_expreriment2(k=5), N = 10000, 256 <= val < 1025: (6437120, 10000), error = 0.19156980780138003%,       3483 ns   (R²=0.997, 295388 iterations in 107 samples)
  .sum_expreriment3(k=5), N = 10000, 256 <= val < 1025: (5369472..7494769, 10000), error = 0.11374651896429032%,       3357 ns   (R²=0.994, 295388 iterations in 107 samples)
```

```
              actual sum, N = 10000, 400 <= val < 2000: 11916587
  .sum_expreriment1(k=5), N = 10000, 400 <= val < 2000: (11850368, 10000), error = -0.5556876310305963%,       2956 ns   (R²=0.995, 357422 iterations in 109 samples)
  .sum_expreriment2(k=5), N = 10000, 400 <= val < 2000: (11850368, 10000), error = -0.5556876310305963%,       2903 ns   (R²=0.997, 357422 iterations in 109 samples)
  .sum_expreriment3(k=5), N = 10000, 400 <= val < 2000: (9706240..13984497, 10000), error = -0.5976459534932276%,       2690 ns   (R²=0.997, 393165 iterations in 110 samples)
```

### Some value ranges require greater k to achieve error < 1%

#### Bad (k=5)

```
              actual sum, N = 10000, 300 <= val < 1500: 8970873
  .sum_expreriment1(k=5), N = 10000, 300 <= val < 1500: (8746496, 10000), error = -2.501172405405806%,       3206 ns   (R²=0.998, 324928 iterations in 108 samples)
  .sum_expreriment2(k=5), N = 10000, 300 <= val < 1500: (8746496, 10000), error = -2.501172405405806%,       3651 ns   (R²=0.997, 295388 iterations in 107 samples)
  .sum_expreriment3(k=5), N = 10000, 300 <= val < 1500: (7237376..10245617, 10000), error = -2.556908341027679%,       3461 ns   (R²=0.994, 295388 iterations in 107 samples)
```

#### Good (k=10)

```
              actual sum, N = 10000, 300 <= val < 1500: 8974215
 .sum_expreriment1(k=10), N = 10000, 300 <= val < 1500: (8972032, 10000), error = -0.024325247389325975%,       6758 ns   (R²=0.997, 151574 iterations in 100 samples)
 .sum_expreriment2(k=10), N = 10000, 300 <= val < 1500: (8972032, 10000), error = -0.024325247389325975%,       7439 ns   (R²=0.996, 137793 iterations in 99 samples)
 .sum_expreriment3(k=10), N = 10000, 300 <= val < 1500: (8332032..9602033, 10000), error = -0.0800404269342778%,       7987 ns   (R²=0.996, 125265 iterations in 98 samples)
```


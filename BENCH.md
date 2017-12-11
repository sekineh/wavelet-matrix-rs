
running 6 tests
test rsdic_simple::tests::rsdic_sanity ... ignored
test wavelet_matrix::tests::example ... ignored
test wavelet_matrix::tests::layers_4 ... ignored
test wavelet_matrix::tests::layers_64 ... ignored
test wavelet_matrix::tests::layers_7 ... ignored
test wavelet_matrix::tests::wavelet_matrix_sanity ... ignored

test result: ok. 0 passed; 0 failed; 6 ignored; 0 measured; 0 filtered out

(wait for cpu cooldown)         36 ns   (R²=0.993, 23684996 iterations in 153 samples)
# Overall Performance

## 16-bit Values

(wait for cpu cooldown)         35 ns   (R²=1.000, 26053497 iterations in 154 samples)
### N = 10000000

```
    WaveletMatrix::new(), N = 10000000, 16-bit values: 3510841997 ns   (only 1 iteration)
           .lookup(rand), N = 10000000, 16-bit values:       1137 ns   (R²=1.000, 927079 iterations in 119 samples)
               vec[rand], N = 10000000, 16-bit values:         36 ns   (R²=0.998, 23684996 iterations in 153 samples)
               rand only, N = 10000000, 16-bit values:         36 ns   (R²=0.999, 23684996 iterations in 153 samples)
                 .rank(), N = 10000000, 16-bit values:       2030 ns   (R²=1.000, 523306 iterations in 113 samples)
               .select(), N = 10000000, 16-bit values:      18185 ns   (R²=1.000, 58429 iterations in 90 samples)
                .count(), N = 10000000, 16-bit values:       2048 ns   (R²=1.000, 523306 iterations in 113 samples)
         .count_prefix(), N = 10000000, 16-bit values:       1043 ns   (R²=0.999, 1019788 iterations in 120 samples)
        .search().next(), N = 10000000, 16-bit values:      30513 ns   (R²=0.998, 36274 iterations in 85 samples)
 .search_prefix().next(), N = 10000000, 16-bit values:      14169 ns   (R²=0.999, 77774 iterations in 93 samples)
               .median(), N = 10000000, 16-bit values:       1759 ns   (R²=1.000, 575638 iterations in 114 samples)
             .quantile(), N = 10000000, 16-bit values:       1961 ns   (R²=1.000, 523306 iterations in 113 samples)
          .sum_ex3(k=16), N = 10000000, 16-bit values:      11308 ns   (R²=0.998, 94110 iterations in 95 samples)
         .sum_ex3(k=256), N = 10000000, 16-bit values:     167921 ns   (R²=1.000, 6512 iterations in 67 samples)
```

(wait for cpu cooldown)         34 ns   (R²=1.000, 26053497 iterations in 154 samples)
### N = 1000000

```
    WaveletMatrix::new(), N = 1000000, 16-bit values:  305381503 ns   (only 1 iteration)
           .lookup(rand), N = 1000000, 16-bit values:        976 ns   (R²=1.000, 1019788 iterations in 120 samples)
               vec[rand], N = 1000000, 16-bit values:         35 ns   (R²=1.000, 23684996 iterations in 153 samples)
               rand only, N = 1000000, 16-bit values:         34 ns   (R²=0.999, 26053497 iterations in 154 samples)
                 .rank(), N = 1000000, 16-bit values:       2076 ns   (R²=0.998, 523306 iterations in 113 samples)
               .select(), N = 1000000, 16-bit values:      15267 ns   (R²=0.999, 70702 iterations in 92 samples)
                .count(), N = 1000000, 16-bit values:       2042 ns   (R²=1.000, 523306 iterations in 113 samples)
         .count_prefix(), N = 1000000, 16-bit values:       1183 ns   (R²=0.999, 927079 iterations in 119 samples)
        .search().next(), N = 1000000, 16-bit values:      24656 ns   (R²=1.000, 43895 iterations in 87 samples)
 .search_prefix().next(), N = 1000000, 16-bit values:      12430 ns   (R²=1.000, 85553 iterations in 94 samples)
               .median(), N = 1000000, 16-bit values:       1751 ns   (R²=1.000, 575638 iterations in 114 samples)
             .quantile(), N = 1000000, 16-bit values:       1962 ns   (R²=1.000, 523306 iterations in 113 samples)
          .sum_ex3(k=16), N = 1000000, 16-bit values:       9956 ns   (R²=0.999, 103522 iterations in 96 samples)
         .sum_ex3(k=256), N = 1000000, 16-bit values:     185058 ns   (R²=0.999, 5919 iterations in 66 samples)
```

(wait for cpu cooldown)         41 ns   (R²=0.999, 21531814 iterations in 152 samples)
(wait for cpu cooldown)         44 ns   (R²=0.998, 19574375 iterations in 151 samples)
(wait for cpu cooldown)         48 ns   (R²=0.996, 19574375 iterations in 151 samples)
(wait for cpu cooldown)         52 ns   (R²=0.997, 16177167 iterations in 149 samples)
(wait for cpu cooldown)         43 ns   (R²=0.998, 19574375 iterations in 151 samples)
(wait for cpu cooldown)         39 ns   (R²=0.999, 21531814 iterations in 152 samples)
(wait for cpu cooldown)         35 ns   (R²=0.999, 26053497 iterations in 154 samples)
### N = 100000

```
    WaveletMatrix::new(), N = 100000, 16-bit values:   32220257 ns   (only 1 iteration)
           .lookup(rand), N = 100000, 16-bit values:       1118 ns   (R²=0.999, 927079 iterations in 119 samples)
               vec[rand], N = 100000, 16-bit values:         39 ns   (R²=0.999, 21531814 iterations in 152 samples)
               rand only, N = 100000, 16-bit values:         41 ns   (R²=0.999, 21531814 iterations in 152 samples)
                 .rank(), N = 100000, 16-bit values:       2401 ns   (R²=0.998, 432483 iterations in 111 samples)
               .select(), N = 100000, 16-bit values:      19207 ns   (R²=0.992, 58429 iterations in 90 samples)
                .count(), N = 100000, 16-bit values:       3183 ns   (R²=0.997, 324928 iterations in 108 samples)
         .count_prefix(), N = 100000, 16-bit values:       1630 ns   (R²=0.997, 633203 iterations in 115 samples)
        .search().next(), N = 100000, 16-bit values:      27999 ns   (R²=0.993, 36274 iterations in 85 samples)
 .search_prefix().next(), N = 100000, 16-bit values:      13294 ns   (R²=0.998, 77774 iterations in 93 samples)
               .median(), N = 100000, 16-bit values:       3346 ns   (R²=0.999, 324928 iterations in 108 samples)
             .quantile(), N = 100000, 16-bit values:       3412 ns   (R²=0.995, 324928 iterations in 108 samples)
          .sum_ex3(k=16), N = 100000, 16-bit values:      16260 ns   (R²=0.996, 64273 iterations in 91 samples)
         .sum_ex3(k=256), N = 100000, 16-bit values:     261073 ns   (R²=0.996, 4444 iterations in 63 samples)
```

(wait for cpu cooldown)         47 ns   (R²=0.997, 17794885 iterations in 150 samples)
(wait for cpu cooldown)         41 ns   (R²=0.997, 21531814 iterations in 152 samples)
(wait for cpu cooldown)         37 ns   (R²=0.999, 23684996 iterations in 153 samples)
### N = 10000

```
    WaveletMatrix::new(), N = 10000, 16-bit values:    2920752 ns   (only 1 iteration)
           .lookup(rand), N = 10000, 16-bit values:       1280 ns   (R²=0.999, 842798 iterations in 118 samples)
               vec[rand], N = 10000, 16-bit values:         41 ns   (R²=1.000, 21531814 iterations in 152 samples)
               rand only, N = 10000, 16-bit values:         39 ns   (R²=1.000, 23684996 iterations in 153 samples)
                 .rank(), N = 10000, 16-bit values:       2195 ns   (R²=1.000, 475732 iterations in 112 samples)
               .select(), N = 10000, 16-bit values:      13640 ns   (R²=0.999, 77774 iterations in 93 samples)
                .count(), N = 10000, 16-bit values:       2622 ns   (R²=0.994, 393165 iterations in 110 samples)
         .count_prefix(), N = 10000, 16-bit values:       1459 ns   (R²=0.998, 696525 iterations in 116 samples)
        .search().next(), N = 10000, 16-bit values:      27321 ns   (R²=0.998, 39903 iterations in 86 samples)
 .search_prefix().next(), N = 10000, 16-bit values:      16802 ns   (R²=0.997, 64273 iterations in 91 samples)
               .median(), N = 10000, 16-bit values:       4725 ns   (R²=0.996, 221926 iterations in 104 samples)
             .quantile(), N = 10000, 16-bit values:       3719 ns   (R²=0.996, 268533 iterations in 106 samples)
          .sum_ex3(k=16), N = 10000, 16-bit values:      18902 ns   (R²=0.997, 53116 iterations in 89 samples)
         .sum_ex3(k=256), N = 10000, 16-bit values:     226184 ns   (R²=0.993, 4444 iterations in 63 samples)
```

(wait for cpu cooldown)         45 ns   (R²=0.998, 19574375 iterations in 151 samples)
(wait for cpu cooldown)         40 ns   (R²=0.998, 21531814 iterations in 152 samples)
(wait for cpu cooldown)         36 ns   (R²=0.997, 23684996 iterations in 153 samples)
### N = 1000

```
    WaveletMatrix::new(), N = 1000, 16-bit values:     268824 ns   (only 1 iteration)
           .lookup(rand), N = 1000, 16-bit values:       1254 ns   (R²=0.999, 842798 iterations in 118 samples)
               vec[rand], N = 1000, 16-bit values:         37 ns   (R²=0.999, 23684996 iterations in 153 samples)
               rand only, N = 1000, 16-bit values:         37 ns   (R²=0.999, 23684996 iterations in 153 samples)
                 .rank(), N = 1000, 16-bit values:       2034 ns   (R²=0.998, 523306 iterations in 113 samples)
               .select(), N = 1000, 16-bit values:       9293 ns   (R²=0.999, 113876 iterations in 97 samples)
                .count(), N = 1000, 16-bit values:       2651 ns   (R²=0.991, 393165 iterations in 110 samples)
         .count_prefix(), N = 1000, 16-bit values:       1222 ns   (R²=0.996, 842798 iterations in 118 samples)
        .search().next(), N = 1000, 16-bit values:      15538 ns   (R²=0.998, 64273 iterations in 91 samples)
 .search_prefix().next(), N = 1000, 16-bit values:       9528 ns   (R²=0.998, 113876 iterations in 97 samples)
               .median(), N = 1000, 16-bit values:       3097 ns   (R²=0.996, 357422 iterations in 109 samples)
             .quantile(), N = 1000, 16-bit values:       2972 ns   (R²=0.997, 324928 iterations in 108 samples)
          .sum_ex3(k=16), N = 1000, 16-bit values:      12955 ns   (R²=0.999, 77774 iterations in 93 samples)
         .sum_ex3(k=256), N = 1000, 16-bit values:      12235 ns   (R²=0.999, 85553 iterations in 94 samples)
```

## 32-bit Values

(wait for cpu cooldown)         47 ns   (R²=0.999, 19574375 iterations in 151 samples)
(wait for cpu cooldown)         43 ns   (R²=1.000, 19574375 iterations in 151 samples)
(wait for cpu cooldown)         39 ns   (R²=0.998, 21531814 iterations in 152 samples)
(wait for cpu cooldown)         35 ns   (R²=1.000, 23684996 iterations in 153 samples)
### N = 10000000

```
    WaveletMatrix::new(), N = 10000000, 32-bit values: 9938511817 ns   (only 1 iteration)
           .lookup(rand), N = 10000000, 32-bit values:       4751 ns   (R²=0.988, 221926 iterations in 104 samples)
               vec[rand], N = 10000000, 32-bit values:         92 ns   (R²=0.994, 10044744 iterations in 144 samples)
               rand only, N = 10000000, 32-bit values:         93 ns   (R²=0.998, 10044744 iterations in 144 samples)
                 .rank(), N = 10000000, 32-bit values:      11371 ns   (R²=0.996, 94110 iterations in 95 samples)
               .select(), N = 10000000, 32-bit values:     100473 ns   (R²=0.997, 10498 iterations in 72 samples)
                .count(), N = 10000000, 32-bit values:      10496 ns   (R²=0.994, 94110 iterations in 95 samples)
         .count_prefix(), N = 10000000, 32-bit values:       4174 ns   (R²=0.998, 244120 iterations in 105 samples)
        .search().next(), N = 10000000, 32-bit values:      99729 ns   (R²=0.995, 10498 iterations in 72 samples)
 .search_prefix().next(), N = 10000000, 32-bit values:      50853 ns   (R²=0.989, 20470 iterations in 79 samples)
               .median(), N = 10000000, 32-bit values:       7482 ns   (R²=0.993, 137793 iterations in 99 samples)
             .quantile(), N = 10000000, 32-bit values:       5432 ns   (R²=0.994, 183408 iterations in 102 samples)
          .sum_ex3(k=16), N = 10000000, 32-bit values:      12808 ns   (R²=0.996, 77774 iterations in 93 samples)
         .sum_ex3(k=256), N = 10000000, 32-bit values:     203976 ns   (R²=0.999, 5380 iterations in 65 samples)
```

(wait for cpu cooldown)         43 ns   (R²=0.999, 19574375 iterations in 151 samples)
(wait for cpu cooldown)         40 ns   (R²=0.996, 21531814 iterations in 152 samples)
(wait for cpu cooldown)         37 ns   (R²=0.998, 23684996 iterations in 153 samples)
### N = 1000000

```
    WaveletMatrix::new(), N = 1000000, 32-bit values:  635308542 ns   (only 1 iteration)
           .lookup(rand), N = 1000000, 32-bit values:       2486 ns   (R²=0.997, 432483 iterations in 111 samples)
               vec[rand], N = 1000000, 32-bit values:         42 ns   (R²=1.000, 21531814 iterations in 152 samples)
               rand only, N = 1000000, 32-bit values:         38 ns   (R²=0.999, 23684996 iterations in 153 samples)
                 .rank(), N = 1000000, 32-bit values:       4156 ns   (R²=0.999, 244120 iterations in 105 samples)
               .select(), N = 1000000, 32-bit values:      34345 ns   (R²=0.999, 29976 iterations in 83 samples)
                .count(), N = 1000000, 32-bit values:       5072 ns   (R²=0.999, 201750 iterations in 103 samples)
         .count_prefix(), N = 1000000, 32-bit values:       2894 ns   (R²=0.979, 393165 iterations in 110 samples)
        .search().next(), N = 1000000, 32-bit values:      61463 ns   (R²=0.991, 16915 iterations in 77 samples)
 .search_prefix().next(), N = 1000000, 32-bit values:      41156 ns   (R²=0.998, 27250 iterations in 82 samples)
               .median(), N = 1000000, 32-bit values:       6810 ns   (R²=0.993, 151574 iterations in 100 samples)
             .quantile(), N = 1000000, 32-bit values:       7610 ns   (R²=0.997, 137793 iterations in 99 samples)
          .sum_ex3(k=16), N = 1000000, 32-bit values:      22107 ns   (R²=0.994, 48286 iterations in 88 samples)
         .sum_ex3(k=256), N = 1000000, 32-bit values:     338922 ns   (R²=0.998, 3336 iterations in 60 samples)
```

(wait for cpu cooldown)         70 ns   (R²=0.997, 12154142 iterations in 146 samples)
(wait for cpu cooldown)         60 ns   (R²=0.989, 14706514 iterations in 148 samples)
(wait for cpu cooldown)         48 ns   (R²=0.996, 17794885 iterations in 150 samples)
(wait for cpu cooldown)         41 ns   (R²=0.999, 21531814 iterations in 152 samples)
(wait for cpu cooldown)         37 ns   (R²=0.999, 23684996 iterations in 153 samples)
### N = 100000

```
    WaveletMatrix::new(), N = 100000, 32-bit values:   65378835 ns   (only 1 iteration)
           .lookup(rand), N = 100000, 32-bit values:       2397 ns   (R²=0.998, 432483 iterations in 111 samples)
               vec[rand], N = 100000, 32-bit values:         47 ns   (R²=0.973, 19574375 iterations in 151 samples)
               rand only, N = 100000, 32-bit values:         45 ns   (R²=0.989, 19574375 iterations in 151 samples)
                 .rank(), N = 100000, 32-bit values:       7857 ns   (R²=0.827, 151574 iterations in 100 samples)
               .select(), N = 100000, 32-bit values:      46319 ns   (R²=0.983, 22518 iterations in 80 samples)
                .count(), N = 100000, 32-bit values:       8969 ns   (R²=0.957, 113876 iterations in 97 samples)
         .count_prefix(), N = 100000, 32-bit values:       4556 ns   (R²=0.983, 244120 iterations in 105 samples)
        .search().next(), N = 100000, 32-bit values:      85936 ns   (R²=0.952, 12705 iterations in 74 samples)
 .search_prefix().next(), N = 100000, 32-bit values:      47275 ns   (R²=0.956, 22518 iterations in 80 samples)
               .median(), N = 100000, 32-bit values:      12231 ns   (R²=0.972, 85553 iterations in 94 samples)
             .quantile(), N = 100000, 32-bit values:      11131 ns   (R²=0.987, 94110 iterations in 95 samples)
          .sum_ex3(k=16), N = 100000, 32-bit values:      22330 ns   (R²=0.981, 43895 iterations in 87 samples)
         .sum_ex3(k=256), N = 100000, 32-bit values:     336925 ns   (R²=0.997, 3032 iterations in 59 samples)
```

(wait for cpu cooldown)         65 ns   (R²=0.988, 13369557 iterations in 147 samples)
(wait for cpu cooldown)         54 ns   (R²=0.979, 16177167 iterations in 149 samples)
(wait for cpu cooldown)         50 ns   (R²=0.963, 17794885 iterations in 150 samples)
(wait for cpu cooldown)         49 ns   (R²=0.986, 17794885 iterations in 150 samples)
(wait for cpu cooldown)         69 ns   (R²=0.923, 13369557 iterations in 147 samples)
(wait for cpu cooldown)         65 ns   (R²=0.993, 13369557 iterations in 147 samples)
(wait for cpu cooldown)         74 ns   (R²=0.977, 12154142 iterations in 146 samples)
(wait for cpu cooldown)         70 ns   (R²=0.994, 12154142 iterations in 146 samples)
(wait for cpu cooldown)         62 ns   (R²=0.970, 13369557 iterations in 147 samples)
(wait for cpu cooldown)         62 ns   (R²=0.978, 14706514 iterations in 148 samples)
(wait for cpu cooldown)         56 ns   (R²=0.973, 16177167 iterations in 149 samples)
(wait for cpu cooldown)         51 ns   (R²=0.982, 17794885 iterations in 150 samples)
(wait for cpu cooldown)         62 ns   (R²=0.976, 14706514 iterations in 148 samples)
(wait for cpu cooldown)         48 ns   (R²=0.995, 17794885 iterations in 150 samples)
(wait for cpu cooldown)         37 ns   (R²=0.999, 23684996 iterations in 153 samples)
### N = 10000

```
    WaveletMatrix::new(), N = 10000, 32-bit values:    5712027 ns   (only 1 iteration)
           .lookup(rand), N = 10000, 32-bit values:       2356 ns   (R²=0.993, 475732 iterations in 112 samples)
               vec[rand], N = 10000, 32-bit values:         37 ns   (R²=0.999, 23684996 iterations in 153 samples)
               rand only, N = 10000, 32-bit values:         38 ns   (R²=0.999, 23684996 iterations in 153 samples)
                 .rank(), N = 10000, 32-bit values:       5144 ns   (R²=0.999, 201750 iterations in 103 samples)
               .select(), N = 10000, 32-bit values:      28453 ns   (R²=0.999, 36274 iterations in 85 samples)
                .count(), N = 10000, 32-bit values:       5665 ns   (R²=0.999, 183408 iterations in 102 samples)
         .count_prefix(), N = 10000, 32-bit values:       2621 ns   (R²=0.999, 393165 iterations in 110 samples)
        .search().next(), N = 10000, 32-bit values:      39364 ns   (R²=0.999, 27250 iterations in 82 samples)
 .search_prefix().next(), N = 10000, 32-bit values:      26038 ns   (R²=0.998, 39903 iterations in 86 samples)
               .median(), N = 10000, 32-bit values:       5682 ns   (R²=0.999, 183408 iterations in 102 samples)
             .quantile(), N = 10000, 32-bit values:       5629 ns   (R²=0.999, 183408 iterations in 102 samples)
          .sum_ex3(k=16), N = 10000, 32-bit values:      12652 ns   (R²=0.999, 85553 iterations in 94 samples)
         .sum_ex3(k=256), N = 10000, 32-bit values:     260334 ns   (R²=0.999, 4039 iterations in 62 samples)
```

(wait for cpu cooldown)         47 ns   (R²=0.999, 17794885 iterations in 150 samples)
(wait for cpu cooldown)         41 ns   (R²=0.996, 21531814 iterations in 152 samples)
(wait for cpu cooldown)         41 ns   (R²=1.000, 21531814 iterations in 152 samples)
(wait for cpu cooldown)         37 ns   (R²=1.000, 23684996 iterations in 153 samples)
### N = 1000

```
    WaveletMatrix::new(), N = 1000, 32-bit values:     527385 ns   (only 1 iteration)
           .lookup(rand), N = 1000, 32-bit values:       2105 ns   (R²=1.000, 475732 iterations in 112 samples)
               vec[rand], N = 1000, 32-bit values:         37 ns   (R²=1.000, 23684996 iterations in 153 samples)
               rand only, N = 1000, 32-bit values:         39 ns   (R²=0.998, 23684996 iterations in 153 samples)
                 .rank(), N = 1000, 32-bit values:       4547 ns   (R²=0.998, 221926 iterations in 104 samples)
               .select(), N = 1000, 32-bit values:      20598 ns   (R²=0.998, 53116 iterations in 89 samples)
                .count(), N = 1000, 32-bit values:       5683 ns   (R²=0.994, 201750 iterations in 103 samples)
         .count_prefix(), N = 1000, 32-bit values:       3328 ns   (R²=0.997, 324928 iterations in 108 samples)
        .search().next(), N = 1000, 32-bit values:      37168 ns   (R²=0.998, 27250 iterations in 82 samples)
 .search_prefix().next(), N = 1000, 32-bit values:      19652 ns   (R²=0.998, 53116 iterations in 89 samples)
               .median(), N = 1000, 32-bit values:       7864 ns   (R²=0.995, 137793 iterations in 99 samples)
             .quantile(), N = 1000, 32-bit values:       6422 ns   (R²=0.998, 151574 iterations in 100 samples)
          .sum_ex3(k=16), N = 1000, 32-bit values:      13009 ns   (R²=0.993, 77774 iterations in 93 samples)
         .sum_ex3(k=256), N = 1000, 32-bit values:    2740110 ns   (R²=0.992, 399 iterations in 38 samples)
```

## 64-bit Values

(wait for cpu cooldown)         48 ns   (R²=0.995, 17794885 iterations in 150 samples)
(wait for cpu cooldown)         42 ns   (R²=0.998, 21531814 iterations in 152 samples)
(wait for cpu cooldown)         37 ns   (R²=0.999, 23684996 iterations in 153 samples)
### N = 10000000

```
    WaveletMatrix::new(), N = 10000000, 64-bit values: 21775794423 ns   (only 1 iteration)
           .lookup(rand), N = 10000000, 64-bit values:       6487 ns   (R²=0.993, 151574 iterations in 100 samples)
               vec[rand], N = 10000000, 64-bit values:         53 ns   (R²=0.995, 17794885 iterations in 150 samples)
               rand only, N = 10000000, 64-bit values:         56 ns   (R²=0.998, 16177167 iterations in 149 samples)
                 .rank(), N = 10000000, 64-bit values:      11885 ns   (R²=0.996, 85553 iterations in 94 samples)
               .select(), N = 10000000, 64-bit values:     110014 ns   (R²=0.999, 9542 iterations in 71 samples)
                .count(), N = 10000000, 64-bit values:      11559 ns   (R²=0.998, 94110 iterations in 95 samples)
         .count_prefix(), N = 10000000, 64-bit values:       4850 ns   (R²=0.999, 221926 iterations in 104 samples)
        .search().next(), N = 10000000, 64-bit values:     129216 ns   (R²=0.999, 7883 iterations in 69 samples)
 .search_prefix().next(), N = 10000000, 64-bit values:      71950 ns   (R²=1.000, 15376 iterations in 76 samples)
               .median(), N = 10000000, 64-bit values:      12851 ns   (R²=0.999, 77774 iterations in 93 samples)
             .quantile(), N = 10000000, 64-bit values:      14493 ns   (R²=0.999, 77774 iterations in 93 samples)
          .sum_ex3(k=16), N = 10000000, 64-bit values:      17355 ns   (R²=0.999, 58429 iterations in 90 samples)
         .sum_ex3(k=256), N = 10000000, 64-bit values:     229036 ns   (R²=0.992, 4444 iterations in 63 samples)
```

(wait for cpu cooldown)         53 ns   (R²=0.998, 16177167 iterations in 149 samples)
(wait for cpu cooldown)         43 ns   (R²=0.996, 19574375 iterations in 151 samples)
(wait for cpu cooldown)         39 ns   (R²=0.999, 23684996 iterations in 153 samples)
(wait for cpu cooldown)         38 ns   (R²=0.999, 23684996 iterations in 153 samples)
### N = 1000000

```
    WaveletMatrix::new(), N = 1000000, 64-bit values: 1294090478 ns   (only 1 iteration)
           .lookup(rand), N = 1000000, 64-bit values:       5294 ns   (R²=0.999, 201750 iterations in 103 samples)
               vec[rand], N = 1000000, 64-bit values:         44 ns   (R²=0.999, 21531814 iterations in 152 samples)
               rand only, N = 1000000, 64-bit values:         46 ns   (R²=0.999, 19574375 iterations in 151 samples)
                 .rank(), N = 1000000, 64-bit values:      12742 ns   (R²=0.989, 85553 iterations in 94 samples)
               .select(), N = 1000000, 64-bit values:      88120 ns   (R²=0.993, 11549 iterations in 73 samples)
                .count(), N = 1000000, 64-bit values:      14500 ns   (R²=0.987, 77774 iterations in 93 samples)
         .count_prefix(), N = 1000000, 64-bit values:       7896 ns   (R²=0.978, 137793 iterations in 99 samples)
        .search().next(), N = 1000000, 64-bit values:     168622 ns   (R²=0.957, 5919 iterations in 66 samples)
 .search_prefix().next(), N = 1000000, 64-bit values:      85461 ns   (R²=0.976, 11549 iterations in 73 samples)
               .median(), N = 1000000, 64-bit values:      16546 ns   (R²=0.988, 64273 iterations in 91 samples)
             .quantile(), N = 1000000, 64-bit values:      17011 ns   (R²=0.994, 58429 iterations in 90 samples)
          .sum_ex3(k=16), N = 1000000, 64-bit values:      27346 ns   (R²=0.993, 39903 iterations in 86 samples)
         .sum_ex3(k=256), N = 1000000, 64-bit values:     444714 ns   (R²=0.984, 2274 iterations in 56 samples)
```

(wait for cpu cooldown)         69 ns   (R²=0.995, 12154142 iterations in 146 samples)
(wait for cpu cooldown)         58 ns   (R²=0.996, 14706514 iterations in 148 samples)
(wait for cpu cooldown)         46 ns   (R²=0.999, 19574375 iterations in 151 samples)
(wait for cpu cooldown)         41 ns   (R²=0.997, 21531814 iterations in 152 samples)
(wait for cpu cooldown)         37 ns   (R²=0.999, 23684996 iterations in 153 samples)
### N = 100000

```
    WaveletMatrix::new(), N = 100000, 64-bit values:  130884780 ns   (only 1 iteration)
           .lookup(rand), N = 100000, 64-bit values:       5822 ns   (R²=0.998, 183408 iterations in 102 samples)
               vec[rand], N = 100000, 64-bit values:         42 ns   (R²=0.999, 21531814 iterations in 152 samples)
               rand only, N = 100000, 64-bit values:         42 ns   (R²=0.999, 21531814 iterations in 152 samples)
                 .rank(), N = 100000, 64-bit values:      10599 ns   (R²=0.999, 103522 iterations in 96 samples)
               .select(), N = 100000, 64-bit values:      73711 ns   (R²=0.997, 15376 iterations in 76 samples)
                .count(), N = 100000, 64-bit values:      10818 ns   (R²=0.995, 94110 iterations in 95 samples)
         .count_prefix(), N = 100000, 64-bit values:       5135 ns   (R²=0.999, 201750 iterations in 103 samples)
        .search().next(), N = 100000, 64-bit values:      83950 ns   (R²=0.999, 12705 iterations in 74 samples)
 .search_prefix().next(), N = 100000, 64-bit values:      43350 ns   (R²=0.999, 24771 iterations in 81 samples)
               .median(), N = 100000, 64-bit values:       9224 ns   (R²=0.999, 113876 iterations in 97 samples)
             .quantile(), N = 100000, 64-bit values:      11059 ns   (R²=0.999, 94110 iterations in 95 samples)
          .sum_ex3(k=16), N = 100000, 64-bit values:      14883 ns   (R²=0.999, 70702 iterations in 92 samples)
         .sum_ex3(k=256), N = 100000, 64-bit values:     238800 ns   (R²=0.995, 4444 iterations in 63 samples)
```

(wait for cpu cooldown)         46 ns   (R²=0.998, 19574375 iterations in 151 samples)
(wait for cpu cooldown)         41 ns   (R²=0.999, 21531814 iterations in 152 samples)
(wait for cpu cooldown)         40 ns   (R²=0.999, 21531814 iterations in 152 samples)
(wait for cpu cooldown)         36 ns   (R²=0.999, 23684996 iterations in 153 samples)
### N = 10000

```
    WaveletMatrix::new(), N = 10000, 64-bit values:    9469255 ns   (only 1 iteration)
           .lookup(rand), N = 10000, 64-bit values:       4358 ns   (R²=0.999, 244120 iterations in 105 samples)
               vec[rand], N = 10000, 64-bit values:         37 ns   (R²=0.999, 23684996 iterations in 153 samples)
               rand only, N = 10000, 64-bit values:         39 ns   (R²=0.999, 21531814 iterations in 152 samples)
                 .rank(), N = 10000, 64-bit values:      10924 ns   (R²=0.998, 94110 iterations in 95 samples)
               .select(), N = 10000, 64-bit values:      58799 ns   (R²=0.998, 18608 iterations in 78 samples)
                .count(), N = 10000, 64-bit values:      13416 ns   (R²=0.993, 77774 iterations in 93 samples)
         .count_prefix(), N = 10000, 64-bit values:       6357 ns   (R²=0.994, 151574 iterations in 100 samples)
        .search().next(), N = 10000, 64-bit values:      76980 ns   (R²=0.999, 13977 iterations in 75 samples)
 .search_prefix().next(), N = 10000, 64-bit values:      46677 ns   (R²=0.993, 24771 iterations in 81 samples)
               .median(), N = 10000, 64-bit values:      13935 ns   (R²=0.998, 77774 iterations in 93 samples)
             .quantile(), N = 10000, 64-bit values:      15399 ns   (R²=0.998, 70702 iterations in 92 samples)
          .sum_ex3(k=16), N = 10000, 64-bit values:      18069 ns   (R²=0.999, 58429 iterations in 90 samples)
         .sum_ex3(k=256), N = 10000, 64-bit values:    3722666 ns   (R²=0.997, 269 iterations in 34 samples)
```

(wait for cpu cooldown)         47 ns   (R²=0.998, 17794885 iterations in 150 samples)
(wait for cpu cooldown)         41 ns   (R²=0.998, 21531814 iterations in 152 samples)
(wait for cpu cooldown)         37 ns   (R²=1.000, 23684996 iterations in 153 samples)
### N = 1000

```
    WaveletMatrix::new(), N = 1000, 64-bit values:    1060692 ns   (only 1 iteration)
           .lookup(rand), N = 1000, 64-bit values:       4911 ns   (R²=1.000, 221926 iterations in 104 samples)
               vec[rand], N = 1000, 64-bit values:         40 ns   (R²=0.999, 21531814 iterations in 152 samples)
               rand only, N = 1000, 64-bit values:         44 ns   (R²=0.998, 19574375 iterations in 151 samples)
                 .rank(), N = 1000, 64-bit values:      11840 ns   (R²=0.999, 85553 iterations in 94 samples)
               .select(), N = 1000, 64-bit values:      52253 ns   (R²=0.998, 20470 iterations in 79 samples)
                .count(), N = 1000, 64-bit values:      11861 ns   (R²=0.999, 85553 iterations in 94 samples)
         .count_prefix(), N = 1000, 64-bit values:       5601 ns   (R²=0.999, 183408 iterations in 102 samples)
        .search().next(), N = 1000, 64-bit values:      51594 ns   (R²=1.000, 20470 iterations in 79 samples)
 .search_prefix().next(), N = 1000, 64-bit values:      28908 ns   (R²=0.999, 36274 iterations in 85 samples)
               .median(), N = 1000, 64-bit values:      10678 ns   (R²=0.999, 103522 iterations in 96 samples)
             .quantile(), N = 1000, 64-bit values:      10802 ns   (R²=0.999, 103522 iterations in 96 samples)
          .sum_ex3(k=16), N = 1000, 64-bit values:      12389 ns   (R²=0.994, 85553 iterations in 94 samples)
         .sum_ex3(k=256), N = 1000, 64-bit values:    7375632 ns   (R²=0.999, 147 iterations in 28 samples)
```


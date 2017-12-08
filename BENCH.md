
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
    WaveletMatrix::new(), N = 10000000, 16-bit values: 4189892395 ns   (only 1 iteration)
           .lookup(rand), N = 10000000, 16-bit values:       1128 ns   (R²=0.997, 927079 iterations in 119 samples)
               vec[rand], N = 10000000, 16-bit values:         37 ns   (R²=0.999, 23684996 iterations in 153 samples)
               rand only, N = 10000000, 16-bit values:         36 ns   (R²=0.998, 23684996 iterations in 153 samples)
                 .rank(), N = 10000000, 16-bit values:       2899 ns   (R²=0.984, 357422 iterations in 109 samples)
               .select(), N = 10000000, 16-bit values:      22299 ns   (R²=0.988, 48286 iterations in 88 samples)
                .count(), N = 10000000, 16-bit values:       2241 ns   (R²=0.984, 432483 iterations in 111 samples)
         .count_prefix(), N = 10000000, 16-bit values:       1200 ns   (R²=0.998, 842798 iterations in 118 samples)
        .search().next(), N = 10000000, 16-bit values:      38630 ns   (R²=0.997, 27250 iterations in 82 samples)
 .search_prefix().next(), N = 10000000, 16-bit values:      17320 ns   (R²=0.998, 58429 iterations in 90 samples)
               .median(), N = 10000000, 16-bit values:       2469 ns   (R²=0.997, 432483 iterations in 111 samples)
             .quantile(), N = 10000000, 16-bit values:       2839 ns   (R²=0.996, 393165 iterations in 110 samples)
          .sum_ex3(k=16), N = 10000000, 16-bit values:      16933 ns   (R²=0.996, 58429 iterations in 90 samples)
         .sum_ex3(k=256), N = 10000000, 16-bit values:     221905 ns   (R²=0.998, 4890 iterations in 64 samples)
```

### N = 1000000

```
    WaveletMatrix::new(), N = 1000000, 16-bit values:  398694410 ns   (only 1 iteration)
           .lookup(rand), N = 1000000, 16-bit values:       1708 ns   (R²=0.997, 633203 iterations in 115 samples)
               vec[rand], N = 1000000, 16-bit values:         48 ns   (R²=0.999, 17794885 iterations in 150 samples)
               rand only, N = 1000000, 16-bit values:         42 ns   (R²=0.999, 21531814 iterations in 152 samples)
                 .rank(), N = 1000000, 16-bit values:       3114 ns   (R²=0.998, 324928 iterations in 108 samples)
               .select(), N = 1000000, 16-bit values:      22443 ns   (R²=0.998, 48286 iterations in 88 samples)
                .count(), N = 1000000, 16-bit values:       2710 ns   (R²=0.998, 393165 iterations in 110 samples)
         .count_prefix(), N = 1000000, 16-bit values:       1232 ns   (R²=0.996, 842798 iterations in 118 samples)
        .search().next(), N = 1000000, 16-bit values:      29283 ns   (R²=0.995, 36274 iterations in 85 samples)
 .search_prefix().next(), N = 1000000, 16-bit values:      16299 ns   (R²=0.996, 64273 iterations in 91 samples)
               .median(), N = 1000000, 16-bit values:       2372 ns   (R²=0.997, 432483 iterations in 111 samples)
             .quantile(), N = 1000000, 16-bit values:       2587 ns   (R²=0.994, 393165 iterations in 110 samples)
          .sum_ex3(k=16), N = 1000000, 16-bit values:      12498 ns   (R²=0.989, 94110 iterations in 95 samples)
         .sum_ex3(k=256), N = 1000000, 16-bit values:     195067 ns   (R²=0.994, 5380 iterations in 65 samples)
```

### N = 100000

```
    WaveletMatrix::new(), N = 100000, 16-bit values:   31948543 ns   (only 1 iteration)
           .lookup(rand), N = 100000, 16-bit values:       1047 ns   (R²=0.997, 1019788 iterations in 120 samples)
               vec[rand], N = 100000, 16-bit values:         39 ns   (R²=0.998, 23684996 iterations in 153 samples)
               rand only, N = 100000, 16-bit values:         36 ns   (R²=0.999, 23684996 iterations in 153 samples)
                 .rank(), N = 100000, 16-bit values:       2114 ns   (R²=0.999, 523306 iterations in 113 samples)
               .select(), N = 100000, 16-bit values:      16969 ns   (R²=0.996, 64273 iterations in 91 samples)
                .count(), N = 100000, 16-bit values:       2686 ns   (R²=0.997, 393165 iterations in 110 samples)
         .count_prefix(), N = 100000, 16-bit values:       1263 ns   (R²=0.999, 842798 iterations in 118 samples)
        .search().next(), N = 100000, 16-bit values:      27532 ns   (R²=0.998, 36274 iterations in 85 samples)
 .search_prefix().next(), N = 100000, 16-bit values:      12331 ns   (R²=0.997, 85553 iterations in 94 samples)
               .median(), N = 100000, 16-bit values:       2778 ns   (R²=0.999, 393165 iterations in 110 samples)
             .quantile(), N = 100000, 16-bit values:       2855 ns   (R²=0.998, 357422 iterations in 109 samples)
          .sum_ex3(k=16), N = 100000, 16-bit values:      14239 ns   (R²=0.997, 70702 iterations in 92 samples)
         .sum_ex3(k=256), N = 100000, 16-bit values:     254157 ns   (R²=0.997, 4039 iterations in 62 samples)
```

### N = 10000

```
    WaveletMatrix::new(), N = 10000, 16-bit values:    2583231 ns   (only 1 iteration)
           .lookup(rand), N = 10000, 16-bit values:       1283 ns   (R²=0.996, 842798 iterations in 118 samples)
               vec[rand], N = 10000, 16-bit values:         44 ns   (R²=0.999, 19574375 iterations in 151 samples)
               rand only, N = 10000, 16-bit values:         45 ns   (R²=0.998, 19574375 iterations in 151 samples)
                 .rank(), N = 10000, 16-bit values:       2274 ns   (R²=0.990, 432483 iterations in 111 samples)
               .select(), N = 10000, 16-bit values:      14940 ns   (R²=0.959, 77774 iterations in 93 samples)
                .count(), N = 10000, 16-bit values:       2717 ns   (R²=0.998, 393165 iterations in 110 samples)
         .count_prefix(), N = 10000, 16-bit values:       1449 ns   (R²=0.993, 766179 iterations in 117 samples)
        .search().next(), N = 10000, 16-bit values:      22754 ns   (R²=0.996, 43895 iterations in 87 samples)
 .search_prefix().next(), N = 10000, 16-bit values:      12432 ns   (R²=0.999, 85553 iterations in 94 samples)
               .median(), N = 10000, 16-bit values:       3158 ns   (R²=0.998, 324928 iterations in 108 samples)
             .quantile(), N = 10000, 16-bit values:       2897 ns   (R²=0.995, 357422 iterations in 109 samples)
          .sum_ex3(k=16), N = 10000, 16-bit values:      13152 ns   (R²=0.998, 85553 iterations in 94 samples)
         .sum_ex3(k=256), N = 10000, 16-bit values:     306331 ns   (R²=0.960, 4039 iterations in 62 samples)
```

### N = 1000

```
    WaveletMatrix::new(), N = 1000, 16-bit values:     592516 ns   (only 1 iteration)
           .lookup(rand), N = 1000, 16-bit values:       1905 ns   (R²=0.974, 523306 iterations in 113 samples)
               vec[rand], N = 1000, 16-bit values:         44 ns   (R²=0.999, 19574375 iterations in 151 samples)
               rand only, N = 1000, 16-bit values:         45 ns   (R²=0.999, 19574375 iterations in 151 samples)
                 .rank(), N = 1000, 16-bit values:       2694 ns   (R²=0.998, 393165 iterations in 110 samples)
               .select(), N = 1000, 16-bit values:      12018 ns   (R²=0.981, 85553 iterations in 94 samples)
                .count(), N = 1000, 16-bit values:       2222 ns   (R²=0.999, 475732 iterations in 112 samples)
         .count_prefix(), N = 1000, 16-bit values:       1050 ns   (R²=0.999, 1019788 iterations in 120 samples)
        .search().next(), N = 1000, 16-bit values:      16111 ns   (R²=0.995, 64273 iterations in 91 samples)
 .search_prefix().next(), N = 1000, 16-bit values:       8586 ns   (R²=0.997, 125265 iterations in 98 samples)
               .median(), N = 1000, 16-bit values:       2567 ns   (R²=0.997, 432483 iterations in 111 samples)
             .quantile(), N = 1000, 16-bit values:       2937 ns   (R²=0.997, 357422 iterations in 109 samples)
          .sum_ex3(k=16), N = 1000, 16-bit values:      14169 ns   (R²=0.997, 77774 iterations in 93 samples)
         .sum_ex3(k=256), N = 1000, 16-bit values:     178555 ns   (R²=0.999, 5919 iterations in 66 samples)
```

## 32-bit Values

### N = 10000000

```
    WaveletMatrix::new(), N = 10000000, 32-bit values: 9683594781 ns   (only 1 iteration)
           .lookup(rand), N = 10000000, 32-bit values:       3912 ns   (R²=0.997, 268533 iterations in 106 samples)
               vec[rand], N = 10000000, 32-bit values:         53 ns   (R²=0.990, 16177167 iterations in 149 samples)
               rand only, N = 10000000, 32-bit values:         48 ns   (R²=0.994, 17794885 iterations in 150 samples)
                 .rank(), N = 10000000, 32-bit values:       6224 ns   (R²=0.982, 166733 iterations in 101 samples)
               .select(), N = 10000000, 32-bit values:      50810 ns   (R²=0.994, 20470 iterations in 79 samples)
                .count(), N = 10000000, 32-bit values:       5150 ns   (R²=0.997, 201750 iterations in 103 samples)
         .count_prefix(), N = 10000000, 32-bit values:       2849 ns   (R²=0.990, 393165 iterations in 110 samples)
        .search().next(), N = 10000000, 32-bit values:      77220 ns   (R²=0.975, 13977 iterations in 75 samples)
 .search_prefix().next(), N = 10000000, 32-bit values:      39357 ns   (R²=0.996, 27250 iterations in 82 samples)
               .median(), N = 10000000, 32-bit values:       7590 ns   (R²=0.979, 137793 iterations in 99 samples)
             .quantile(), N = 10000000, 32-bit values:       8940 ns   (R²=0.990, 125265 iterations in 98 samples)
          .sum_ex3(k=16), N = 10000000, 32-bit values:      19629 ns   (R²=0.985, 53116 iterations in 89 samples)
         .sum_ex3(k=256), N = 10000000, 32-bit values:     361257 ns   (R²=0.984, 3032 iterations in 59 samples)
```

### N = 1000000

```
    WaveletMatrix::new(), N = 1000000, 32-bit values: 1297178222 ns   (only 1 iteration)
           .lookup(rand), N = 1000000, 32-bit values:       5194 ns   (R²=0.988, 201750 iterations in 103 samples)
               vec[rand], N = 1000000, 32-bit values:         66 ns   (R²=0.994, 13369557 iterations in 147 samples)
               rand only, N = 1000000, 32-bit values:         56 ns   (R²=0.991, 14706514 iterations in 148 samples)
                 .rank(), N = 1000000, 32-bit values:       9057 ns   (R²=0.984, 125265 iterations in 98 samples)
               .select(), N = 1000000, 32-bit values:      56110 ns   (R²=0.960, 16915 iterations in 77 samples)
                .count(), N = 1000000, 32-bit values:       8948 ns   (R²=0.977, 125265 iterations in 98 samples)
         .count_prefix(), N = 1000000, 32-bit values:       3769 ns   (R²=0.979, 268533 iterations in 106 samples)
        .search().next(), N = 1000000, 32-bit values:      81957 ns   (R²=0.971, 12705 iterations in 74 samples)
 .search_prefix().next(), N = 1000000, 32-bit values:      55704 ns   (R²=0.985, 18608 iterations in 78 samples)
               .median(), N = 1000000, 32-bit values:       7130 ns   (R²=0.985, 137793 iterations in 99 samples)
             .quantile(), N = 1000000, 32-bit values:       7197 ns   (R²=0.972, 137793 iterations in 99 samples)
          .sum_ex3(k=16), N = 1000000, 32-bit values:      20465 ns   (R²=0.982, 53116 iterations in 89 samples)
         .sum_ex3(k=256), N = 1000000, 32-bit values:     294078 ns   (R²=0.985, 3336 iterations in 60 samples)
```

### N = 100000

```
    WaveletMatrix::new(), N = 100000, 32-bit values:   99090066 ns   (only 1 iteration)
           .lookup(rand), N = 100000, 32-bit values:       4235 ns   (R²=0.991, 244120 iterations in 105 samples)
               vec[rand], N = 100000, 32-bit values:         54 ns   (R²=0.995, 16177167 iterations in 149 samples)
               rand only, N = 100000, 32-bit values:         60 ns   (R²=0.997, 14706514 iterations in 148 samples)
                 .rank(), N = 100000, 32-bit values:       6331 ns   (R²=0.988, 151574 iterations in 100 samples)
               .select(), N = 100000, 32-bit values:      55415 ns   (R²=0.984, 20470 iterations in 79 samples)
                .count(), N = 100000, 32-bit values:       7506 ns   (R²=0.986, 137793 iterations in 99 samples)
         .count_prefix(), N = 100000, 32-bit values:       3808 ns   (R²=0.990, 268533 iterations in 106 samples)
        .search().next(), N = 100000, 32-bit values:      88381 ns   (R²=0.969, 12705 iterations in 74 samples)
 .search_prefix().next(), N = 100000, 32-bit values:      45532 ns   (R²=0.990, 22518 iterations in 80 samples)
               .median(), N = 100000, 32-bit values:       7059 ns   (R²=0.992, 151574 iterations in 100 samples)
             .quantile(), N = 100000, 32-bit values:       8338 ns   (R²=0.982, 125265 iterations in 98 samples)
          .sum_ex3(k=16), N = 100000, 32-bit values:      19161 ns   (R²=0.972, 58429 iterations in 90 samples)
         .sum_ex3(k=256), N = 100000, 32-bit values:     314762 ns   (R²=0.981, 3336 iterations in 60 samples)
```

### N = 10000

```
    WaveletMatrix::new(), N = 10000, 32-bit values:    9875018 ns   (only 1 iteration)
           .lookup(rand), N = 10000, 32-bit values:       4251 ns   (R²=0.970, 268533 iterations in 106 samples)
               vec[rand], N = 10000, 32-bit values:         66 ns   (R²=0.991, 13369557 iterations in 147 samples)
               rand only, N = 10000, 32-bit values:         63 ns   (R²=0.996, 13369557 iterations in 147 samples)
                 .rank(), N = 10000, 32-bit values:       7737 ns   (R²=0.971, 125265 iterations in 98 samples)
               .select(), N = 10000, 32-bit values:      41869 ns   (R²=0.974, 24771 iterations in 81 samples)
                .count(), N = 10000, 32-bit values:       8742 ns   (R²=0.991, 125265 iterations in 98 samples)
         .count_prefix(), N = 10000, 32-bit values:       3489 ns   (R²=0.988, 295388 iterations in 107 samples)
        .search().next(), N = 10000, 32-bit values:      41736 ns   (R²=0.975, 22518 iterations in 80 samples)
 .search_prefix().next(), N = 10000, 32-bit values:      27898 ns   (R²=0.992, 39903 iterations in 86 samples)
               .median(), N = 10000, 32-bit values:       7748 ns   (R²=0.985, 137793 iterations in 99 samples)
             .quantile(), N = 10000, 32-bit values:       9640 ns   (R²=0.987, 113876 iterations in 97 samples)
          .sum_ex3(k=16), N = 10000, 32-bit values:       5683 ns   (R²=0.981, 166733 iterations in 101 samples)
         .sum_ex3(k=256), N = 10000, 32-bit values:       5556 ns   (R²=0.983, 183408 iterations in 102 samples)
```

### N = 1000

```
    WaveletMatrix::new(), N = 1000, 32-bit values:     833708 ns   (only 1 iteration)
           .lookup(rand), N = 1000, 32-bit values:       3043 ns   (R²=0.993, 324928 iterations in 108 samples)
               vec[rand], N = 1000, 32-bit values:         46 ns   (R²=0.999, 17794885 iterations in 150 samples)
               rand only, N = 1000, 32-bit values:         42 ns   (R²=0.999, 19574375 iterations in 151 samples)
                 .rank(), N = 1000, 32-bit values:       5370 ns   (R²=0.998, 201750 iterations in 103 samples)
               .select(), N = 1000, 32-bit values:      23769 ns   (R²=0.997, 43895 iterations in 87 samples)
                .count(), N = 1000, 32-bit values:       5112 ns   (R²=0.997, 201750 iterations in 103 samples)
         .count_prefix(), N = 1000, 32-bit values:       2698 ns   (R²=0.997, 393165 iterations in 110 samples)
        .search().next(), N = 1000, 32-bit values:      29916 ns   (R²=0.997, 36274 iterations in 85 samples)
 .search_prefix().next(), N = 1000, 32-bit values:      13585 ns   (R²=0.998, 77774 iterations in 93 samples)
               .median(), N = 1000, 32-bit values:       4400 ns   (R²=0.999, 244120 iterations in 105 samples)
             .quantile(), N = 1000, 32-bit values:       5405 ns   (R²=0.998, 201750 iterations in 103 samples)
          .sum_ex3(k=16), N = 1000, 32-bit values:     145264 ns   (R²=0.991, 7165 iterations in 68 samples)
         .sum_ex3(k=256), N = 1000, 32-bit values:     156347 ns   (R²=0.990, 7165 iterations in 68 samples)
```

## 64-bit Values

### N = 10000000

```
    WaveletMatrix::new(), N = 10000000, 64-bit values: 19846239688 ns   (only 1 iteration)
           .lookup(rand), N = 10000000, 64-bit values:       7196 ns   (R²=0.996, 151574 iterations in 100 samples)
               vec[rand], N = 10000000, 64-bit values:         49 ns   (R²=0.999, 17794885 iterations in 150 samples)
               rand only, N = 10000000, 64-bit values:         43 ns   (R²=0.999, 19574375 iterations in 151 samples)
                 .rank(), N = 10000000, 64-bit values:      10686 ns   (R²=0.991, 94110 iterations in 95 samples)
               .select(), N = 10000000, 64-bit values:     102963 ns   (R²=0.993, 10498 iterations in 72 samples)
                .count(), N = 10000000, 64-bit values:      11772 ns   (R²=0.996, 94110 iterations in 95 samples)
         .count_prefix(), N = 10000000, 64-bit values:       6687 ns   (R²=0.990, 166733 iterations in 101 samples)
        .search().next(), N = 10000000, 64-bit values:     176047 ns   (R²=0.994, 6512 iterations in 67 samples)
 .search_prefix().next(), N = 10000000, 64-bit values:      82417 ns   (R²=0.987, 11549 iterations in 73 samples)
               .median(), N = 10000000, 64-bit values:      12754 ns   (R²=0.996, 77774 iterations in 93 samples)
             .quantile(), N = 10000000, 64-bit values:      12980 ns   (R²=0.998, 85553 iterations in 94 samples)
          .sum_ex3(k=16), N = 10000000, 64-bit values:      17893 ns   (R²=0.996, 64273 iterations in 91 samples)
         .sum_ex3(k=256), N = 10000000, 64-bit values:     266141 ns   (R²=0.991, 3671 iterations in 61 samples)
```

### N = 1000000

```
    WaveletMatrix::new(), N = 1000000, 64-bit values: 1845349136 ns   (only 1 iteration)
           .lookup(rand), N = 1000000, 64-bit values:       6038 ns   (R²=0.993, 166733 iterations in 101 samples)
               vec[rand], N = 1000000, 64-bit values:         45 ns   (R²=0.999, 19574375 iterations in 151 samples)
               rand only, N = 1000000, 64-bit values:         44 ns   (R²=0.999, 19574375 iterations in 151 samples)
                 .rank(), N = 1000000, 64-bit values:      10630 ns   (R²=0.997, 94110 iterations in 95 samples)
               .select(), N = 1000000, 64-bit values:      85821 ns   (R²=0.998, 12705 iterations in 74 samples)
                .count(), N = 1000000, 64-bit values:      10650 ns   (R²=0.998, 94110 iterations in 95 samples)
         .count_prefix(), N = 1000000, 64-bit values:       4653 ns   (R²=0.998, 221926 iterations in 104 samples)
        .search().next(), N = 1000000, 64-bit values:      98370 ns   (R²=0.998, 10498 iterations in 72 samples)
 .search_prefix().next(), N = 1000000, 64-bit values:      59253 ns   (R²=0.998, 18608 iterations in 78 samples)
               .median(), N = 1000000, 64-bit values:      12629 ns   (R²=0.998, 85553 iterations in 94 samples)
             .quantile(), N = 1000000, 64-bit values:      10873 ns   (R²=0.998, 94110 iterations in 95 samples)
          .sum_ex3(k=16), N = 1000000, 64-bit values:      12500 ns   (R²=0.999, 85553 iterations in 94 samples)
         .sum_ex3(k=256), N = 1000000, 64-bit values:     227481 ns   (R²=0.995, 4444 iterations in 63 samples)
```

### N = 100000

```
    WaveletMatrix::new(), N = 100000, 64-bit values:  158752690 ns   (only 1 iteration)
           .lookup(rand), N = 100000, 64-bit values:       5890 ns   (R²=0.998, 183408 iterations in 102 samples)
               vec[rand], N = 100000, 64-bit values:         45 ns   (R²=0.998, 19574375 iterations in 151 samples)
               rand only, N = 100000, 64-bit values:         40 ns   (R²=0.996, 21531814 iterations in 152 samples)
                 .rank(), N = 100000, 64-bit values:       9992 ns   (R²=0.994, 103522 iterations in 96 samples)
               .select(), N = 100000, 64-bit values:      70993 ns   (R²=0.998, 15376 iterations in 76 samples)
                .count(), N = 100000, 64-bit values:      11303 ns   (R²=0.995, 94110 iterations in 95 samples)
         .count_prefix(), N = 100000, 64-bit values:       6929 ns   (R²=0.988, 166733 iterations in 101 samples)
        .search().next(), N = 100000, 64-bit values:     101831 ns   (R²=0.986, 9542 iterations in 71 samples)
 .search_prefix().next(), N = 100000, 64-bit values:      60071 ns   (R²=0.990, 18608 iterations in 78 samples)
               .median(), N = 100000, 64-bit values:      12959 ns   (R²=0.991, 77774 iterations in 93 samples)
             .quantile(), N = 100000, 64-bit values:      11288 ns   (R²=0.996, 94110 iterations in 95 samples)
          .sum_ex3(k=16), N = 100000, 64-bit values:      14893 ns   (R²=0.998, 70702 iterations in 92 samples)
         .sum_ex3(k=256), N = 100000, 64-bit values:     211594 ns   (R²=0.997, 4890 iterations in 64 samples)
```

### N = 10000

```
    WaveletMatrix::new(), N = 10000, 64-bit values:    9833175 ns   (only 1 iteration)
           .lookup(rand), N = 10000, 64-bit values:       5199 ns   (R²=0.998, 201750 iterations in 103 samples)
               vec[rand], N = 10000, 64-bit values:         38 ns   (R²=0.998, 21531814 iterations in 152 samples)
               rand only, N = 10000, 64-bit values:         36 ns   (R²=0.999, 23684996 iterations in 153 samples)
                 .rank(), N = 10000, 64-bit values:      10056 ns   (R²=0.998, 103522 iterations in 96 samples)
               .select(), N = 10000, 64-bit values:      57690 ns   (R²=0.996, 18608 iterations in 78 samples)
                .count(), N = 10000, 64-bit values:      12047 ns   (R²=0.995, 85553 iterations in 94 samples)
         .count_prefix(), N = 10000, 64-bit values:       5579 ns   (R²=0.999, 183408 iterations in 102 samples)
        .search().next(), N = 10000, 64-bit values:      76471 ns   (R²=0.999, 13977 iterations in 75 samples)
 .search_prefix().next(), N = 10000, 64-bit values:      36860 ns   (R²=0.998, 27250 iterations in 82 samples)
               .median(), N = 10000, 64-bit values:      11578 ns   (R²=0.999, 94110 iterations in 95 samples)
             .quantile(), N = 10000, 64-bit values:      10552 ns   (R²=0.999, 94110 iterations in 95 samples)
          .sum_ex3(k=16), N = 10000, 64-bit values:      13576 ns   (R²=0.994, 77774 iterations in 93 samples)
         .sum_ex3(k=256), N = 10000, 64-bit values:    5953486 ns   (R²=0.993, 163 iterations in 29 samples)
```

### N = 1000

```
    WaveletMatrix::new(), N = 1000, 64-bit values:    1263588 ns   (only 1 iteration)
           .lookup(rand), N = 1000, 64-bit values:       6177 ns   (R²=0.994, 166733 iterations in 101 samples)
               vec[rand], N = 1000, 64-bit values:         49 ns   (R²=0.998, 17794885 iterations in 150 samples)
               rand only, N = 1000, 64-bit values:         45 ns   (R²=0.998, 19574375 iterations in 151 samples)
                 .rank(), N = 1000, 64-bit values:      10095 ns   (R²=0.998, 103522 iterations in 96 samples)
               .select(), N = 1000, 64-bit values:      43006 ns   (R²=0.997, 24771 iterations in 81 samples)
                .count(), N = 1000, 64-bit values:      11271 ns   (R²=0.996, 94110 iterations in 95 samples)
         .count_prefix(), N = 1000, 64-bit values:       5704 ns   (R²=0.998, 183408 iterations in 102 samples)
        .search().next(), N = 1000, 64-bit values:      68832 ns   (R²=0.982, 15376 iterations in 76 samples)
 .search_prefix().next(), N = 1000, 64-bit values:      35351 ns   (R²=0.987, 27250 iterations in 82 samples)
               .median(), N = 1000, 64-bit values:      14463 ns   (R²=0.992, 77774 iterations in 93 samples)
             .quantile(), N = 1000, 64-bit values:      12976 ns   (R²=0.987, 77774 iterations in 93 samples)
          .sum_ex3(k=16), N = 1000, 64-bit values:     221754 ns   (R²=0.997, 4444 iterations in 63 samples)
         .sum_ex3(k=256), N = 1000, 64-bit values:     207646 ns   (R²=0.997, 4890 iterations in 64 samples)
```



running 6 tests
test rsdic_simple::tests::rsdic_sanity ... ignored
test wavelet_matrix::tests::example ... ignored
test wavelet_matrix::tests::layers_4 ... ignored
test wavelet_matrix::tests::layers_64 ... ignored
test wavelet_matrix::tests::layers_7 ... ignored
test wavelet_matrix::tests::wavelet_matrix_sanity ... ignored

test result: ok. 0 passed; 0 failed; 6 ignored; 0 measured; 0 filtered out

(wait for cpu cooldown)         35 ns   (R²=0.993, 23684996 iterations in 153 samples)
# Overall Performance

## 16-bit Values

(wait for cpu cooldown)         35 ns   (R²=0.999, 26053497 iterations in 154 samples)
### N = 10000000

```
    WaveletMatrix::new(), N = 10000000, 16-bit values: 2422463466 ns   (only 1 iteration)
           .lookup(rand), N = 10000000, 16-bit values:       1063 ns   (R²=1.000, 1019788 iterations in 120 samples)
               vec[rand], N = 10000000, 16-bit values:         35 ns   (R²=0.998, 26053497 iterations in 154 samples)
               rand only, N = 10000000, 16-bit values:         35 ns   (R²=1.000, 23684996 iterations in 153 samples)
                 .rank(), N = 10000000, 16-bit values:       2097 ns   (R²=1.000, 475732 iterations in 112 samples)
               .select(), N = 10000000, 16-bit values:      18212 ns   (R²=0.997, 58429 iterations in 90 samples)
                .count(), N = 10000000, 16-bit values:       2111 ns   (R²=0.997, 475732 iterations in 112 samples)
         .count_prefix(), N = 10000000, 16-bit values:       1267 ns   (R²=1.000, 842798 iterations in 118 samples)
        .search().next(), N = 10000000, 16-bit values:      28932 ns   (R²=0.999, 36274 iterations in 85 samples)
 .search_prefix().next(), N = 10000000, 16-bit values:      14376 ns   (R²=1.000, 70702 iterations in 92 samples)
               .median(), N = 10000000, 16-bit values:       1818 ns   (R²=0.999, 575638 iterations in 114 samples)
             .quantile(), N = 10000000, 16-bit values:       2159 ns   (R²=1.000, 475732 iterations in 112 samples)
          .sum_ex3(k=16), N = 10000000, 16-bit values:      11658 ns   (R²=0.999, 94110 iterations in 95 samples)
         .sum_ex3(k=256), N = 10000000, 16-bit values:     190504 ns   (R²=0.999, 5380 iterations in 65 samples)
```

(wait for cpu cooldown)         35 ns   (R²=0.999, 26053497 iterations in 154 samples)
### N = 1000000

```
    WaveletMatrix::new(), N = 1000000, 16-bit values:  242212477 ns   (only 1 iteration)
           .lookup(rand), N = 1000000, 16-bit values:       1116 ns   (R²=1.000, 927079 iterations in 119 samples)
               vec[rand], N = 1000000, 16-bit values:         35 ns   (R²=0.999, 26053497 iterations in 154 samples)
               rand only, N = 1000000, 16-bit values:         34 ns   (R²=0.999, 26053497 iterations in 154 samples)
                 .rank(), N = 1000000, 16-bit values:       1898 ns   (R²=1.000, 575638 iterations in 114 samples)
               .select(), N = 1000000, 16-bit values:      14985 ns   (R²=1.000, 70702 iterations in 92 samples)
                .count(), N = 1000000, 16-bit values:       2009 ns   (R²=0.997, 523306 iterations in 113 samples)
         .count_prefix(), N = 1000000, 16-bit values:       1016 ns   (R²=1.000, 1019788 iterations in 120 samples)
        .search().next(), N = 1000000, 16-bit values:      24420 ns   (R²=0.997, 43895 iterations in 87 samples)
 .search_prefix().next(), N = 1000000, 16-bit values:      11656 ns   (R²=0.999, 94110 iterations in 95 samples)
               .median(), N = 1000000, 16-bit values:       1976 ns   (R²=1.000, 523306 iterations in 113 samples)
             .quantile(), N = 1000000, 16-bit values:       2335 ns   (R²=0.999, 475732 iterations in 112 samples)
          .sum_ex3(k=16), N = 1000000, 16-bit values:      17745 ns   (R²=0.999, 58429 iterations in 90 samples)
         .sum_ex3(k=256), N = 1000000, 16-bit values:     233457 ns   (R²=0.999, 4444 iterations in 63 samples)
```

(wait for cpu cooldown)         46 ns   (R²=0.999, 19574375 iterations in 151 samples)
(wait for cpu cooldown)         65 ns   (R²=0.981, 14706514 iterations in 148 samples)
(wait for cpu cooldown)         64 ns   (R²=0.999, 14706514 iterations in 148 samples)
(wait for cpu cooldown)         59 ns   (R²=0.999, 14706514 iterations in 148 samples)
(wait for cpu cooldown)         47 ns   (R²=0.998, 17794885 iterations in 150 samples)
(wait for cpu cooldown)         41 ns   (R²=0.999, 21531814 iterations in 152 samples)
(wait for cpu cooldown)         36 ns   (R²=0.999, 23684996 iterations in 153 samples)
### N = 100000

```
    WaveletMatrix::new(), N = 100000, 16-bit values:   20662382 ns   (only 1 iteration)
           .lookup(rand), N = 100000, 16-bit values:       1008 ns   (R²=0.999, 1019788 iterations in 120 samples)
               vec[rand], N = 100000, 16-bit values:         41 ns   (R²=0.999, 21531814 iterations in 152 samples)
               rand only, N = 100000, 16-bit values:         39 ns   (R²=0.992, 21531814 iterations in 152 samples)
                 .rank(), N = 100000, 16-bit values:       2227 ns   (R²=0.999, 475732 iterations in 112 samples)
               .select(), N = 100000, 16-bit values:      16739 ns   (R²=1.000, 64273 iterations in 91 samples)
                .count(), N = 100000, 16-bit values:       3089 ns   (R²=0.994, 357422 iterations in 109 samples)
         .count_prefix(), N = 100000, 16-bit values:       1858 ns   (R²=0.999, 575638 iterations in 114 samples)
        .search().next(), N = 100000, 16-bit values:      32485 ns   (R²=0.991, 29976 iterations in 83 samples)
 .search_prefix().next(), N = 100000, 16-bit values:      13525 ns   (R²=0.998, 77774 iterations in 93 samples)
               .median(), N = 100000, 16-bit values:       2872 ns   (R²=0.999, 357422 iterations in 109 samples)
             .quantile(), N = 100000, 16-bit values:       3123 ns   (R²=0.993, 357422 iterations in 109 samples)
          .sum_ex3(k=16), N = 100000, 16-bit values:      20031 ns   (R²=0.996, 53116 iterations in 89 samples)
         .sum_ex3(k=256), N = 100000, 16-bit values:     295160 ns   (R²=0.996, 3336 iterations in 60 samples)
```

(wait for cpu cooldown)         49 ns   (R²=0.992, 17794885 iterations in 150 samples)
(wait for cpu cooldown)         41 ns   (R²=0.998, 21531814 iterations in 152 samples)
(wait for cpu cooldown)         40 ns   (R²=0.999, 21531814 iterations in 152 samples)
(wait for cpu cooldown)         37 ns   (R²=0.999, 23684996 iterations in 153 samples)
### N = 10000

```
    WaveletMatrix::new(), N = 10000, 16-bit values:    2792459 ns   (only 1 iteration)
           .lookup(rand), N = 10000, 16-bit values:       1078 ns   (R²=0.999, 1019788 iterations in 120 samples)
               vec[rand], N = 10000, 16-bit values:         41 ns   (R²=0.999, 21531814 iterations in 152 samples)
               rand only, N = 10000, 16-bit values:         39 ns   (R²=0.998, 21531814 iterations in 152 samples)
                 .rank(), N = 10000, 16-bit values:       2510 ns   (R²=0.997, 432483 iterations in 111 samples)
               .select(), N = 10000, 16-bit values:      13858 ns   (R²=0.999, 77774 iterations in 93 samples)
                .count(), N = 10000, 16-bit values:       3280 ns   (R²=0.999, 324928 iterations in 108 samples)
         .count_prefix(), N = 10000, 16-bit values:       2151 ns   (R²=0.719, 475732 iterations in 112 samples)
        .search().next(), N = 10000, 16-bit values:      29571 ns   (R²=0.965, 36274 iterations in 85 samples)
 .search_prefix().next(), N = 10000, 16-bit values:      14274 ns   (R²=0.992, 70702 iterations in 92 samples)
               .median(), N = 10000, 16-bit values:       2995 ns   (R²=0.997, 357422 iterations in 109 samples)
             .quantile(), N = 10000, 16-bit values:       3511 ns   (R²=0.986, 295388 iterations in 107 samples)
          .sum_ex3(k=16), N = 10000, 16-bit values:      77965 ns   (R²=0.995, 13977 iterations in 75 samples)
         .sum_ex3(k=256), N = 10000, 16-bit values:      87757 ns   (R²=0.999, 11549 iterations in 73 samples)
```

(wait for cpu cooldown)         55 ns   (R²=0.999, 16177167 iterations in 149 samples)
(wait for cpu cooldown)         43 ns   (R²=1.000, 19574375 iterations in 151 samples)
(wait for cpu cooldown)         38 ns   (R²=1.000, 23684996 iterations in 153 samples)
### N = 1000

```
    WaveletMatrix::new(), N = 1000, 16-bit values:     842395 ns   (only 1 iteration)
           .lookup(rand), N = 1000, 16-bit values:       1260 ns   (R²=0.999, 842798 iterations in 118 samples)
               vec[rand], N = 1000, 16-bit values:         43 ns   (R²=0.999, 19574375 iterations in 151 samples)
               rand only, N = 1000, 16-bit values:         46 ns   (R²=0.999, 19574375 iterations in 151 samples)
                 .rank(), N = 1000, 16-bit values:       2571 ns   (R²=0.999, 393165 iterations in 110 samples)
               .select(), N = 1000, 16-bit values:      10608 ns   (R²=0.998, 94110 iterations in 95 samples)
                .count(), N = 1000, 16-bit values:       2659 ns   (R²=0.998, 393165 iterations in 110 samples)
         .count_prefix(), N = 1000, 16-bit values:       1310 ns   (R²=0.999, 842798 iterations in 118 samples)
        .search().next(), N = 1000, 16-bit values:      15906 ns   (R²=0.998, 70702 iterations in 92 samples)
 .search_prefix().next(), N = 1000, 16-bit values:       8877 ns   (R²=0.998, 113876 iterations in 97 samples)
               .median(), N = 1000, 16-bit values:       2508 ns   (R²=0.997, 393165 iterations in 110 samples)
             .quantile(), N = 1000, 16-bit values:       2508 ns   (R²=0.998, 393165 iterations in 110 samples)
          .sum_ex3(k=16), N = 1000, 16-bit values:      49858 ns   (R²=0.999, 20470 iterations in 79 samples)
         .sum_ex3(k=256), N = 1000, 16-bit values:      80274 ns   (R²=0.998, 12705 iterations in 74 samples)
```

## 32-bit Values

(wait for cpu cooldown)         45 ns   (R²=0.998, 19574375 iterations in 151 samples)
(wait for cpu cooldown)         45 ns   (R²=0.997, 19574375 iterations in 151 samples)
(wait for cpu cooldown)         39 ns   (R²=0.997, 21531814 iterations in 152 samples)
(wait for cpu cooldown)         38 ns   (R²=0.999, 23684996 iterations in 153 samples)
### N = 10000000

```
    WaveletMatrix::new(), N = 10000000, 32-bit values: 6780819192 ns   (only 1 iteration)
           .lookup(rand), N = 10000000, 32-bit values:       3828 ns   (R²=0.996, 268533 iterations in 106 samples)
               vec[rand], N = 10000000, 32-bit values:         64 ns   (R²=0.998, 13369557 iterations in 147 samples)
               rand only, N = 10000000, 32-bit values:         61 ns   (R²=0.998, 14706514 iterations in 148 samples)
                 .rank(), N = 10000000, 32-bit values:       6334 ns   (R²=0.998, 166733 iterations in 101 samples)
               .select(), N = 10000000, 32-bit values:      51794 ns   (R²=0.987, 18608 iterations in 78 samples)
                .count(), N = 10000000, 32-bit values:       6471 ns   (R²=0.996, 166733 iterations in 101 samples)
         .count_prefix(), N = 10000000, 32-bit values:       3403 ns   (R²=0.999, 324928 iterations in 108 samples)
        .search().next(), N = 10000000, 32-bit values:     106054 ns   (R²=0.989, 10498 iterations in 72 samples)
 .search_prefix().next(), N = 10000000, 32-bit values:      49680 ns   (R²=0.994, 20470 iterations in 79 samples)
               .median(), N = 10000000, 32-bit values:       7241 ns   (R²=0.999, 137793 iterations in 99 samples)
             .quantile(), N = 10000000, 32-bit values:       7698 ns   (R²=0.999, 137793 iterations in 99 samples)
          .sum_ex3(k=16), N = 10000000, 32-bit values:      19739 ns   (R²=0.999, 53116 iterations in 89 samples)
         .sum_ex3(k=256), N = 10000000, 32-bit values:     254592 ns   (R²=0.999, 4039 iterations in 62 samples)
```

(wait for cpu cooldown)         43 ns   (R²=0.999, 19574375 iterations in 151 samples)
(wait for cpu cooldown)         40 ns   (R²=0.998, 23684996 iterations in 153 samples)
(wait for cpu cooldown)         35 ns   (R²=1.000, 26053497 iterations in 154 samples)
### N = 1000000

```
    WaveletMatrix::new(), N = 1000000, 32-bit values:  501694857 ns   (only 1 iteration)
           .lookup(rand), N = 1000000, 32-bit values:       2474 ns   (R²=0.999, 432483 iterations in 111 samples)
               vec[rand], N = 1000000, 32-bit values:         42 ns   (R²=0.999, 21531814 iterations in 152 samples)
               rand only, N = 1000000, 32-bit values:         43 ns   (R²=0.998, 21531814 iterations in 152 samples)
                 .rank(), N = 1000000, 32-bit values:       5754 ns   (R²=1.000, 183408 iterations in 102 samples)
               .select(), N = 1000000, 32-bit values:      48547 ns   (R²=0.995, 22518 iterations in 80 samples)
                .count(), N = 1000000, 32-bit values:       7693 ns   (R²=0.999, 137793 iterations in 99 samples)
         .count_prefix(), N = 1000000, 32-bit values:       3262 ns   (R²=0.999, 324928 iterations in 108 samples)
        .search().next(), N = 1000000, 32-bit values:      77283 ns   (R²=0.999, 13977 iterations in 75 samples)
 .search_prefix().next(), N = 1000000, 32-bit values:      39556 ns   (R²=0.999, 27250 iterations in 82 samples)
               .median(), N = 1000000, 32-bit values:       4379 ns   (R²=0.998, 244120 iterations in 105 samples)
             .quantile(), N = 1000000, 32-bit values:       5318 ns   (R²=0.998, 201750 iterations in 103 samples)
          .sum_ex3(k=16), N = 1000000, 32-bit values:      17420 ns   (R²=0.995, 58429 iterations in 90 samples)
         .sum_ex3(k=256), N = 1000000, 32-bit values:     274019 ns   (R²=0.979, 3671 iterations in 61 samples)
```

(wait for cpu cooldown)         46 ns   (R²=0.998, 19574375 iterations in 151 samples)
(wait for cpu cooldown)         40 ns   (R²=0.999, 21531814 iterations in 152 samples)
(wait for cpu cooldown)         37 ns   (R²=0.999, 23684996 iterations in 153 samples)
### N = 100000

```
    WaveletMatrix::new(), N = 100000, 32-bit values:   43051793 ns   (only 1 iteration)
           .lookup(rand), N = 100000, 32-bit values:       2496 ns   (R²=0.999, 432483 iterations in 111 samples)
               vec[rand], N = 100000, 32-bit values:         39 ns   (R²=0.999, 21531814 iterations in 152 samples)
               rand only, N = 100000, 32-bit values:         39 ns   (R²=0.998, 21531814 iterations in 152 samples)
                 .rank(), N = 100000, 32-bit values:       6278 ns   (R²=0.991, 183408 iterations in 102 samples)
               .select(), N = 100000, 32-bit values:      39614 ns   (R²=0.985, 27250 iterations in 82 samples)
                .count(), N = 100000, 32-bit values:       8110 ns   (R²=0.986, 137793 iterations in 99 samples)
         .count_prefix(), N = 100000, 32-bit values:       3919 ns   (R²=1.000, 268533 iterations in 106 samples)
        .search().next(), N = 100000, 32-bit values:      61694 ns   (R²=0.998, 16915 iterations in 77 samples)
 .search_prefix().next(), N = 100000, 32-bit values:      36574 ns   (R²=1.000, 29976 iterations in 83 samples)
               .median(), N = 100000, 32-bit values:       6885 ns   (R²=0.993, 151574 iterations in 100 samples)
             .quantile(), N = 100000, 32-bit values:       7197 ns   (R²=0.999, 151574 iterations in 100 samples)
          .sum_ex3(k=16), N = 100000, 32-bit values:      20865 ns   (R²=0.999, 53116 iterations in 89 samples)
         .sum_ex3(k=256), N = 100000, 32-bit values:    2156109 ns   (R²=0.992, 485 iterations in 40 samples)
```

(wait for cpu cooldown)         54 ns   (R²=0.999, 16177167 iterations in 149 samples)
(wait for cpu cooldown)         44 ns   (R²=1.000, 19574375 iterations in 151 samples)
(wait for cpu cooldown)         39 ns   (R²=0.999, 21531814 iterations in 152 samples)
(wait for cpu cooldown)         35 ns   (R²=1.000, 26053497 iterations in 154 samples)
### N = 10000

```
    WaveletMatrix::new(), N = 10000, 32-bit values:    5118718 ns   (only 1 iteration)
           .lookup(rand), N = 10000, 32-bit values:       2213 ns   (R²=0.999, 475732 iterations in 112 samples)
               vec[rand], N = 10000, 32-bit values:         39 ns   (R²=0.999, 23684996 iterations in 153 samples)
               rand only, N = 10000, 32-bit values:         42 ns   (R²=0.999, 21531814 iterations in 152 samples)
                 .rank(), N = 10000, 32-bit values:       4965 ns   (R²=0.997, 221926 iterations in 104 samples)
               .select(), N = 10000, 32-bit values:      29130 ns   (R²=0.996, 36274 iterations in 85 samples)
                .count(), N = 10000, 32-bit values:       5298 ns   (R²=0.999, 201750 iterations in 103 samples)
         .count_prefix(), N = 10000, 32-bit values:       2701 ns   (R²=0.997, 393165 iterations in 110 samples)
        .search().next(), N = 10000, 32-bit values:      40330 ns   (R²=0.998, 27250 iterations in 82 samples)
 .search_prefix().next(), N = 10000, 32-bit values:      20891 ns   (R²=0.998, 48286 iterations in 88 samples)
               .median(), N = 10000, 32-bit values:       6142 ns   (R²=0.998, 166733 iterations in 101 samples)
             .quantile(), N = 10000, 32-bit values:       4687 ns   (R²=0.998, 221926 iterations in 104 samples)
          .sum_ex3(k=16), N = 10000, 32-bit values:      15742 ns   (R²=0.993, 64273 iterations in 91 samples)
         .sum_ex3(k=256), N = 10000, 32-bit values:     253367 ns   (R²=0.999, 4039 iterations in 62 samples)
```

(wait for cpu cooldown)         46 ns   (R²=0.999, 19574375 iterations in 151 samples)
(wait for cpu cooldown)         47 ns   (R²=0.997, 17794885 iterations in 150 samples)
(wait for cpu cooldown)         41 ns   (R²=0.999, 21531814 iterations in 152 samples)
(wait for cpu cooldown)         37 ns   (R²=0.999, 23684996 iterations in 153 samples)
### N = 1000

```
    WaveletMatrix::new(), N = 1000, 32-bit values:     889765 ns   (only 1 iteration)
           .lookup(rand), N = 1000, 32-bit values:       2319 ns   (R²=0.999, 475732 iterations in 112 samples)
               vec[rand], N = 1000, 32-bit values:         41 ns   (R²=1.000, 21531814 iterations in 152 samples)
               rand only, N = 1000, 32-bit values:         44 ns   (R²=1.000, 19574375 iterations in 151 samples)
                 .rank(), N = 1000, 32-bit values:       4827 ns   (R²=0.998, 221926 iterations in 104 samples)
               .select(), N = 1000, 32-bit values:      20606 ns   (R²=1.000, 53116 iterations in 89 samples)
                .count(), N = 1000, 32-bit values:       5718 ns   (R²=0.999, 183408 iterations in 102 samples)
         .count_prefix(), N = 1000, 32-bit values:       3587 ns   (R²=0.999, 295388 iterations in 107 samples)
        .search().next(), N = 1000, 32-bit values:      37268 ns   (R²=0.999, 27250 iterations in 82 samples)
 .search_prefix().next(), N = 1000, 32-bit values:      18920 ns   (R²=0.996, 58429 iterations in 90 samples)
               .median(), N = 1000, 32-bit values:       6959 ns   (R²=0.998, 151574 iterations in 100 samples)
             .quantile(), N = 1000, 32-bit values:       5686 ns   (R²=0.993, 183408 iterations in 102 samples)
          .sum_ex3(k=16), N = 1000, 32-bit values:       3137 ns   (R²=0.998, 324928 iterations in 108 samples)
         .sum_ex3(k=256), N = 1000, 32-bit values:       3099 ns   (R²=0.998, 357422 iterations in 109 samples)
```

## 64-bit Values

(wait for cpu cooldown)         51 ns   (R²=0.993, 17794885 iterations in 150 samples)
(wait for cpu cooldown)         44 ns   (R²=0.992, 19574375 iterations in 151 samples)
(wait for cpu cooldown)         40 ns   (R²=0.998, 21531814 iterations in 152 samples)
(wait for cpu cooldown)         35 ns   (R²=0.998, 23684996 iterations in 153 samples)
### N = 10000000

```
    WaveletMatrix::new(), N = 10000000, 64-bit values: 15017809526 ns   (only 1 iteration)
           .lookup(rand), N = 10000000, 64-bit values:       9611 ns   (R²=0.989, 113876 iterations in 97 samples)
               vec[rand], N = 10000000, 64-bit values:         67 ns   (R²=0.986, 13369557 iterations in 147 samples)
               rand only, N = 10000000, 64-bit values:         61 ns   (R²=0.994, 14706514 iterations in 148 samples)
                 .rank(), N = 10000000, 64-bit values:      13114 ns   (R²=0.996, 77774 iterations in 93 samples)
               .select(), N = 10000000, 64-bit values:     114260 ns   (R²=0.998, 8673 iterations in 70 samples)
                .count(), N = 10000000, 64-bit values:      12870 ns   (R²=0.998, 85553 iterations in 94 samples)
         .count_prefix(), N = 10000000, 64-bit values:       6276 ns   (R²=0.993, 166733 iterations in 101 samples)
        .search().next(), N = 10000000, 64-bit values:     171009 ns   (R²=0.999, 6512 iterations in 67 samples)
 .search_prefix().next(), N = 10000000, 64-bit values:      82047 ns   (R²=0.977, 12705 iterations in 74 samples)
               .median(), N = 10000000, 64-bit values:      14098 ns   (R²=0.999, 77774 iterations in 93 samples)
             .quantile(), N = 10000000, 64-bit values:      11580 ns   (R²=0.999, 94110 iterations in 95 samples)
          .sum_ex3(k=16), N = 10000000, 64-bit values:      17828 ns   (R²=0.999, 58429 iterations in 90 samples)
         .sum_ex3(k=256), N = 10000000, 64-bit values:     336287 ns   (R²=0.998, 3032 iterations in 59 samples)
```

(wait for cpu cooldown)         57 ns   (R²=0.998, 14706514 iterations in 148 samples)
(wait for cpu cooldown)         45 ns   (R²=0.999, 19574375 iterations in 151 samples)
(wait for cpu cooldown)         40 ns   (R²=0.999, 21531814 iterations in 152 samples)
(wait for cpu cooldown)         36 ns   (R²=0.999, 23684996 iterations in 153 samples)
### N = 1000000

```
    WaveletMatrix::new(), N = 1000000, 64-bit values:  960448451 ns   (only 1 iteration)
           .lookup(rand), N = 1000000, 64-bit values:       4448 ns   (R²=0.999, 244120 iterations in 105 samples)
               vec[rand], N = 1000000, 64-bit values:         40 ns   (R²=0.999, 21531814 iterations in 152 samples)
               rand only, N = 1000000, 64-bit values:         43 ns   (R²=0.995, 21531814 iterations in 152 samples)
                 .rank(), N = 1000000, 64-bit values:      12358 ns   (R²=0.981, 94110 iterations in 95 samples)
               .select(), N = 1000000, 64-bit values:     102993 ns   (R²=0.972, 10498 iterations in 72 samples)
                .count(), N = 1000000, 64-bit values:      19613 ns   (R²=0.772, 70702 iterations in 92 samples)
         .count_prefix(), N = 1000000, 64-bit values:       9307 ns   (R²=0.964, 125265 iterations in 98 samples)
        .search().next(), N = 1000000, 64-bit values:     177159 ns   (R²=0.990, 5919 iterations in 66 samples)
 .search_prefix().next(), N = 1000000, 64-bit values:      95867 ns   (R²=0.983, 10498 iterations in 72 samples)
               .median(), N = 1000000, 64-bit values:      15416 ns   (R²=0.980, 64273 iterations in 91 samples)
             .quantile(), N = 1000000, 64-bit values:      18310 ns   (R²=0.982, 58429 iterations in 90 samples)
          .sum_ex3(k=16), N = 1000000, 64-bit values:      19418 ns   (R²=0.988, 53116 iterations in 89 samples)
         .sum_ex3(k=256), N = 1000000, 64-bit values:     455642 ns   (R²=0.890, 2503 iterations in 57 samples)
```

(wait for cpu cooldown)         73 ns   (R²=0.977, 12154142 iterations in 146 samples)
(wait for cpu cooldown)         75 ns   (R²=0.973, 11049219 iterations in 145 samples)
(wait for cpu cooldown)        110 ns   (R²=0.931, 8301439 iterations in 142 samples)
(wait for cpu cooldown)         69 ns   (R²=0.972, 12154142 iterations in 146 samples)
(wait for cpu cooldown)         61 ns   (R²=0.990, 14706514 iterations in 148 samples)
(wait for cpu cooldown)         49 ns   (R²=0.980, 19574375 iterations in 151 samples)
(wait for cpu cooldown)         50 ns   (R²=0.979, 17794885 iterations in 150 samples)
(wait for cpu cooldown)         55 ns   (R²=0.991, 16177167 iterations in 149 samples)
(wait for cpu cooldown)         66 ns   (R²=0.982, 13369557 iterations in 147 samples)
(wait for cpu cooldown)         71 ns   (R²=0.997, 13369557 iterations in 147 samples)
(wait for cpu cooldown)         95 ns   (R²=0.990, 9131584 iterations in 143 samples)
(wait for cpu cooldown)         74 ns   (R²=0.929, 12154142 iterations in 146 samples)
(wait for cpu cooldown)         51 ns   (R²=0.973, 17794885 iterations in 150 samples)
(wait for cpu cooldown)         48 ns   (R²=0.989, 17794885 iterations in 150 samples)
(wait for cpu cooldown)         36 ns   (R²=0.999, 23684996 iterations in 153 samples)
### N = 100000

```
    WaveletMatrix::new(), N = 100000, 64-bit values:   81038949 ns   (only 1 iteration)
           .lookup(rand), N = 100000, 64-bit values:       4104 ns   (R²=0.999, 268533 iterations in 106 samples)
               vec[rand], N = 100000, 64-bit values:         38 ns   (R²=0.998, 23684996 iterations in 153 samples)
               rand only, N = 100000, 64-bit values:         39 ns   (R²=0.998, 21531814 iterations in 152 samples)
                 .rank(), N = 100000, 64-bit values:      10449 ns   (R²=0.999, 103522 iterations in 96 samples)
               .select(), N = 100000, 64-bit values:      75474 ns   (R²=0.995, 13977 iterations in 75 samples)
                .count(), N = 100000, 64-bit values:      10789 ns   (R²=0.996, 94110 iterations in 95 samples)
         .count_prefix(), N = 100000, 64-bit values:       5783 ns   (R²=0.999, 183408 iterations in 102 samples)
        .search().next(), N = 100000, 64-bit values:      92142 ns   (R²=1.000, 11549 iterations in 73 samples)
 .search_prefix().next(), N = 100000, 64-bit values:      57188 ns   (R²=0.996, 18608 iterations in 78 samples)
               .median(), N = 100000, 64-bit values:      14245 ns   (R²=0.999, 70702 iterations in 92 samples)
             .quantile(), N = 100000, 64-bit values:      14903 ns   (R²=0.998, 70702 iterations in 92 samples)
          .sum_ex3(k=16), N = 100000, 64-bit values:      21465 ns   (R²=0.998, 48286 iterations in 88 samples)
         .sum_ex3(k=256), N = 100000, 64-bit values:     510123 ns   (R²=0.997, 2066 iterations in 55 samples)
```

(wait for cpu cooldown)         65 ns   (R²=0.998, 13369557 iterations in 147 samples)
(wait for cpu cooldown)         55 ns   (R²=0.999, 16177167 iterations in 149 samples)
(wait for cpu cooldown)         44 ns   (R²=0.999, 19574375 iterations in 151 samples)
(wait for cpu cooldown)         39 ns   (R²=1.000, 21531814 iterations in 152 samples)
(wait for cpu cooldown)         35 ns   (R²=0.999, 26053497 iterations in 154 samples)
### N = 10000

```
    WaveletMatrix::new(), N = 10000, 64-bit values:    7968022 ns   (only 1 iteration)
           .lookup(rand), N = 10000, 64-bit values:       4936 ns   (R²=1.000, 221926 iterations in 104 samples)
               vec[rand], N = 10000, 64-bit values:         39 ns   (R²=1.000, 23684996 iterations in 153 samples)
               rand only, N = 10000, 64-bit values:         41 ns   (R²=0.996, 21531814 iterations in 152 samples)
                 .rank(), N = 10000, 64-bit values:      10136 ns   (R²=0.999, 103522 iterations in 96 samples)
               .select(), N = 10000, 64-bit values:      59004 ns   (R²=1.000, 16915 iterations in 77 samples)
                .count(), N = 10000, 64-bit values:       9810 ns   (R²=0.998, 103522 iterations in 96 samples)
         .count_prefix(), N = 10000, 64-bit values:       5098 ns   (R²=1.000, 201750 iterations in 103 samples)
        .search().next(), N = 10000, 64-bit values:      71115 ns   (R²=0.999, 15376 iterations in 76 samples)
 .search_prefix().next(), N = 10000, 64-bit values:      44905 ns   (R²=0.993, 24771 iterations in 81 samples)
               .median(), N = 10000, 64-bit values:      11645 ns   (R²=0.989, 85553 iterations in 94 samples)
             .quantile(), N = 10000, 64-bit values:      11885 ns   (R²=0.998, 85553 iterations in 94 samples)
          .sum_ex3(k=16), N = 10000, 64-bit values:      14683 ns   (R²=0.999, 70702 iterations in 92 samples)
         .sum_ex3(k=256), N = 10000, 64-bit values:    6705044 ns   (R²=0.989, 180 iterations in 30 samples)
```

(wait for cpu cooldown)         49 ns   (R²=0.993, 17794885 iterations in 150 samples)
(wait for cpu cooldown)         46 ns   (R²=0.998, 19574375 iterations in 151 samples)
(wait for cpu cooldown)         41 ns   (R²=0.998, 21531814 iterations in 152 samples)
(wait for cpu cooldown)         38 ns   (R²=0.997, 23684996 iterations in 153 samples)
### N = 1000

```
    WaveletMatrix::new(), N = 1000, 64-bit values:    1228066 ns   (only 1 iteration)
           .lookup(rand), N = 1000, 64-bit values:       4552 ns   (R²=0.999, 244120 iterations in 105 samples)
               vec[rand], N = 1000, 64-bit values:         36 ns   (R²=0.999, 23684996 iterations in 153 samples)
               rand only, N = 1000, 64-bit values:         39 ns   (R²=0.999, 23684996 iterations in 153 samples)
                 .rank(), N = 1000, 64-bit values:       8939 ns   (R²=0.999, 113876 iterations in 97 samples)
               .select(), N = 1000, 64-bit values:      40607 ns   (R²=0.999, 27250 iterations in 82 samples)
                .count(), N = 1000, 64-bit values:      10897 ns   (R²=0.999, 94110 iterations in 95 samples)
         .count_prefix(), N = 1000, 64-bit values:       6475 ns   (R²=0.994, 166733 iterations in 101 samples)
        .search().next(), N = 1000, 64-bit values:      57845 ns   (R²=0.998, 18608 iterations in 78 samples)
 .search_prefix().next(), N = 1000, 64-bit values:      28881 ns   (R²=0.998, 36274 iterations in 85 samples)
               .median(), N = 1000, 64-bit values:      12395 ns   (R²=1.000, 85553 iterations in 94 samples)
             .quantile(), N = 1000, 64-bit values:      13236 ns   (R²=0.998, 77774 iterations in 93 samples)
          .sum_ex3(k=16), N = 1000, 64-bit values:     177136 ns   (R²=0.995, 5919 iterations in 66 samples)
         .sum_ex3(k=256), N = 1000, 64-bit values:     165477 ns   (R²=0.995, 6512 iterations in 67 samples)
```


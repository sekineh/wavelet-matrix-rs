# Overall Performance

## 16-bit Values

(wait for cpu cooldown)         21 ns   (R²=1.000, 41959424 iterations in 159 samples)
### N = 10000000

```
    WaveletMatrix::new(), N = 10000000, 16-bit values: 1541527033 ns   (only 1 iteration)
           .lookup(rand), N = 10000000, 16-bit values:        727 ns   (R²=1.000, 1493077 iterations in 124 samples)
               vec[rand], N = 10000000, 16-bit values:         23 ns   (R²=1.000, 41959424 iterations in 159 samples)
               rand only, N = 10000000, 16-bit values:         23 ns   (R²=1.000, 38144930 iterations in 158 samples)
                 .rank(), N = 10000000, 16-bit values:       1016 ns   (R²=1.000, 1019788 iterations in 120 samples)
               .select(), N = 10000000, 16-bit values:      12195 ns   (R²=1.000, 85553 iterations in 94 samples)
                .count(), N = 10000000, 16-bit values:       1285 ns   (R²=1.000, 842798 iterations in 118 samples)
         .count_prefix(), N = 10000000, 16-bit values:        677 ns   (R²=1.000, 1493077 iterations in 124 samples)
        .search().next(), N = 10000000, 16-bit values:      19794 ns   (R²=1.000, 53116 iterations in 89 samples)
 .search_prefix().next(), N = 10000000, 16-bit values:       8776 ns   (R²=1.000, 125265 iterations in 98 samples)
               .median(), N = 10000000, 16-bit values:       1186 ns   (R²=1.000, 842798 iterations in 118 samples)
             .quantile(), N = 10000000, 16-bit values:       1382 ns   (R²=1.000, 766179 iterations in 117 samples)
          .sum_ex3(k=16), N = 10000000, 16-bit values:       5798 ns   (R²=1.000, 183408 iterations in 102 samples)
         .sum_ex3(k=256), N = 10000000, 16-bit values:     100675 ns   (R²=1.000, 10498 iterations in 72 samples)
```

(wait for cpu cooldown)         21 ns   (R²=1.000, 41959424 iterations in 159 samples)
### N = 1000000

```
    WaveletMatrix::new(), N = 1000000, 16-bit values:  150889715 ns   (only 1 iteration)
           .lookup(rand), N = 1000000, 16-bit values:        673 ns   (R²=1.000, 1493077 iterations in 124 samples)
               vec[rand], N = 1000000, 16-bit values:         22 ns   (R²=1.000, 41959424 iterations in 159 samples)
               rand only, N = 1000000, 16-bit values:         23 ns   (R²=1.000, 38144930 iterations in 158 samples)
                 .rank(), N = 1000000, 16-bit values:       1207 ns   (R²=1.000, 842798 iterations in 118 samples)
               .select(), N = 1000000, 16-bit values:      10122 ns   (R²=1.000, 103522 iterations in 96 samples)
                .count(), N = 1000000, 16-bit values:       1190 ns   (R²=1.000, 842798 iterations in 118 samples)
         .count_prefix(), N = 1000000, 16-bit values:        632 ns   (R²=1.000, 1642386 iterations in 125 samples)
        .search().next(), N = 1000000, 16-bit values:      15676 ns   (R²=1.000, 64273 iterations in 91 samples)
 .search_prefix().next(), N = 1000000, 16-bit values:       7935 ns   (R²=1.000, 137793 iterations in 99 samples)
               .median(), N = 1000000, 16-bit values:       1315 ns   (R²=1.000, 766179 iterations in 117 samples)
             .quantile(), N = 1000000, 16-bit values:       1557 ns   (R²=1.000, 696525 iterations in 116 samples)
          .sum_ex3(k=16), N = 1000000, 16-bit values:       5761 ns   (R²=1.000, 183408 iterations in 102 samples)
         .sum_ex3(k=256), N = 1000000, 16-bit values:     100071 ns   (R²=1.000, 10498 iterations in 72 samples)
```

(wait for cpu cooldown)         21 ns   (R²=1.000, 41959424 iterations in 159 samples)
### N = 100000

```
    WaveletMatrix::new(), N = 100000, 16-bit values:   13611430 ns   (only 1 iteration)
           .lookup(rand), N = 100000, 16-bit values:        615 ns   (R²=1.000, 1642386 iterations in 125 samples)
               vec[rand], N = 100000, 16-bit values:         23 ns   (R²=1.000, 38144930 iterations in 158 samples)
               rand only, N = 100000, 16-bit values:         22 ns   (R²=1.000, 38144930 iterations in 158 samples)
                 .rank(), N = 100000, 16-bit values:       1140 ns   (R²=1.000, 927079 iterations in 119 samples)
               .select(), N = 100000, 16-bit values:       8811 ns   (R²=1.000, 113876 iterations in 97 samples)
                .count(), N = 100000, 16-bit values:       1220 ns   (R²=1.000, 842798 iterations in 118 samples)
         .count_prefix(), N = 100000, 16-bit values:        575 ns   (R²=1.000, 1806626 iterations in 126 samples)
        .search().next(), N = 100000, 16-bit values:      13481 ns   (R²=1.000, 77774 iterations in 93 samples)
 .search_prefix().next(), N = 100000, 16-bit values:       7252 ns   (R²=1.000, 151574 iterations in 100 samples)
               .median(), N = 100000, 16-bit values:       1136 ns   (R²=1.000, 927079 iterations in 119 samples)
             .quantile(), N = 100000, 16-bit values:       1589 ns   (R²=1.000, 633203 iterations in 115 samples)
          .sum_ex3(k=16), N = 100000, 16-bit values:       6548 ns   (R²=1.000, 166733 iterations in 101 samples)
         .sum_ex3(k=256), N = 100000, 16-bit values:     104714 ns   (R²=1.000, 10498 iterations in 72 samples)
```

(wait for cpu cooldown)         21 ns   (R²=1.000, 41959424 iterations in 159 samples)
### N = 10000

```
    WaveletMatrix::new(), N = 10000, 16-bit values:    1377998 ns   (only 1 iteration)
           .lookup(rand), N = 10000, 16-bit values:        695 ns   (R²=1.000, 1493077 iterations in 124 samples)
               vec[rand], N = 10000, 16-bit values:         22 ns   (R²=1.000, 38144930 iterations in 158 samples)
               rand only, N = 10000, 16-bit values:         22 ns   (R²=1.000, 38144930 iterations in 158 samples)
                 .rank(), N = 10000, 16-bit values:       1168 ns   (R²=1.000, 927079 iterations in 119 samples)
               .select(), N = 10000, 16-bit values:       7186 ns   (R²=1.000, 151574 iterations in 100 samples)
                .count(), N = 10000, 16-bit values:       1324 ns   (R²=1.000, 766179 iterations in 117 samples)
         .count_prefix(), N = 10000, 16-bit values:        728 ns   (R²=1.000, 1493077 iterations in 124 samples)
        .search().next(), N = 10000, 16-bit values:      10378 ns   (R²=1.000, 103522 iterations in 96 samples)
 .search_prefix().next(), N = 10000, 16-bit values:       5971 ns   (R²=1.000, 183408 iterations in 102 samples)
               .median(), N = 10000, 16-bit values:       1537 ns   (R²=1.000, 696525 iterations in 116 samples)
             .quantile(), N = 10000, 16-bit values:       1584 ns   (R²=1.000, 633203 iterations in 115 samples)
          .sum_ex3(k=16), N = 10000, 16-bit values:       5474 ns   (R²=1.000, 183408 iterations in 102 samples)
         .sum_ex3(k=256), N = 10000, 16-bit values:      97751 ns   (R²=1.000, 10498 iterations in 72 samples)
```

(wait for cpu cooldown)         21 ns   (R²=1.000, 41959424 iterations in 159 samples)
### N = 1000

```
    WaveletMatrix::new(), N = 1000, 16-bit values:     161732 ns   (only 1 iteration)
           .lookup(rand), N = 1000, 16-bit values:        748 ns   (R²=1.000, 1357342 iterations in 123 samples)
               vec[rand], N = 1000, 16-bit values:         22 ns   (R²=1.000, 38144930 iterations in 158 samples)
               rand only, N = 1000, 16-bit values:         22 ns   (R²=1.000, 38144930 iterations in 158 samples)
                 .rank(), N = 1000, 16-bit values:       1040 ns   (R²=1.000, 1019788 iterations in 120 samples)
               .select(), N = 1000, 16-bit values:       5116 ns   (R²=1.000, 201750 iterations in 103 samples)
                .count(), N = 1000, 16-bit values:       1179 ns   (R²=1.000, 927079 iterations in 119 samples)
         .count_prefix(), N = 1000, 16-bit values:        584 ns   (R²=1.000, 1806626 iterations in 126 samples)
        .search().next(), N = 1000, 16-bit values:       7435 ns   (R²=1.000, 137793 iterations in 99 samples)
 .search_prefix().next(), N = 1000, 16-bit values:       4686 ns   (R²=1.000, 221926 iterations in 104 samples)
               .median(), N = 1000, 16-bit values:       1424 ns   (R²=1.000, 766179 iterations in 117 samples)
             .quantile(), N = 1000, 16-bit values:       1511 ns   (R²=1.000, 696525 iterations in 116 samples)
          .sum_ex3(k=16), N = 1000, 16-bit values:      12352 ns   (R²=1.000, 85553 iterations in 94 samples)
         .sum_ex3(k=256), N = 1000, 16-bit values:      12354 ns   (R²=1.000, 85553 iterations in 94 samples)
```

## 32-bit Values

(wait for cpu cooldown)         21 ns   (R²=1.000, 41959424 iterations in 159 samples)
### N = 10000000

```
    WaveletMatrix::new(), N = 10000000, 32-bit values: 3045843227 ns   (only 1 iteration)
           .lookup(rand), N = 10000000, 32-bit values:       1628 ns   (R²=1.000, 633203 iterations in 115 samples)
               vec[rand], N = 10000000, 32-bit values:         23 ns   (R²=1.000, 41959424 iterations in 159 samples)
               rand only, N = 10000000, 32-bit values:         23 ns   (R²=1.000, 41959424 iterations in 159 samples)
                 .rank(), N = 10000000, 32-bit values:       2642 ns   (R²=1.000, 393165 iterations in 110 samples)
               .select(), N = 10000000, 32-bit values:      26609 ns   (R²=1.000, 39903 iterations in 86 samples)
                .count(), N = 10000000, 32-bit values:       2471 ns   (R²=1.000, 432483 iterations in 111 samples)
         .count_prefix(), N = 10000000, 32-bit values:       1252 ns   (R²=1.000, 842798 iterations in 118 samples)
        .search().next(), N = 10000000, 32-bit values:      36067 ns   (R²=1.000, 29976 iterations in 83 samples)
 .search_prefix().next(), N = 10000000, 32-bit values:      19656 ns   (R²=1.000, 53116 iterations in 89 samples)
               .median(), N = 10000000, 32-bit values:       3057 ns   (R²=1.000, 357422 iterations in 109 samples)
             .quantile(), N = 10000000, 32-bit values:       2903 ns   (R²=1.000, 357422 iterations in 109 samples)
          .sum_ex3(k=16), N = 10000000, 32-bit values:       6131 ns   (R²=1.000, 166733 iterations in 101 samples)
         .sum_ex3(k=256), N = 10000000, 32-bit values:     103495 ns   (R²=1.000, 10498 iterations in 72 samples)
```

(wait for cpu cooldown)         21 ns   (R²=1.000, 41959424 iterations in 159 samples)
### N = 1000000

```
    WaveletMatrix::new(), N = 1000000, 32-bit values:  287209109 ns   (only 1 iteration)
           .lookup(rand), N = 1000000, 32-bit values:       1542 ns   (R²=1.000, 696525 iterations in 116 samples)
               vec[rand], N = 1000000, 32-bit values:         22 ns   (R²=1.000, 41959424 iterations in 159 samples)
               rand only, N = 1000000, 32-bit values:         23 ns   (R²=1.000, 38144930 iterations in 158 samples)
                 .rank(), N = 1000000, 32-bit values:       2369 ns   (R²=1.000, 432483 iterations in 111 samples)
               .select(), N = 1000000, 32-bit values:      21427 ns   (R²=1.000, 48286 iterations in 88 samples)
                .count(), N = 1000000, 32-bit values:       2392 ns   (R²=1.000, 432483 iterations in 111 samples)
         .count_prefix(), N = 1000000, 32-bit values:       1229 ns   (R²=1.000, 842798 iterations in 118 samples)
        .search().next(), N = 1000000, 32-bit values:      29802 ns   (R²=1.000, 36274 iterations in 85 samples)
 .search_prefix().next(), N = 1000000, 32-bit values:      17091 ns   (R²=0.998, 64273 iterations in 91 samples)
               .median(), N = 1000000, 32-bit values:       3213 ns   (R²=1.000, 324928 iterations in 108 samples)
             .quantile(), N = 1000000, 32-bit values:       2785 ns   (R²=1.000, 393165 iterations in 110 samples)
          .sum_ex3(k=16), N = 1000000, 32-bit values:       6255 ns   (R²=1.000, 166733 iterations in 101 samples)
         .sum_ex3(k=256), N = 1000000, 32-bit values:     100300 ns   (R²=1.000, 10498 iterations in 72 samples)
```

(wait for cpu cooldown)         21 ns   (R²=0.999, 41959424 iterations in 159 samples)
### N = 100000

```
    WaveletMatrix::new(), N = 100000, 32-bit values:   27077300 ns   (only 1 iteration)
           .lookup(rand), N = 100000, 32-bit values:       1411 ns   (R²=1.000, 766179 iterations in 117 samples)
               vec[rand], N = 100000, 32-bit values:         22 ns   (R²=1.000, 41959424 iterations in 159 samples)
               rand only, N = 100000, 32-bit values:         22 ns   (R²=1.000, 41959424 iterations in 159 samples)
                 .rank(), N = 100000, 32-bit values:       2372 ns   (R²=1.000, 432483 iterations in 111 samples)
               .select(), N = 100000, 32-bit values:      18017 ns   (R²=1.000, 58429 iterations in 90 samples)
                .count(), N = 100000, 32-bit values:       2389 ns   (R²=1.000, 432483 iterations in 111 samples)
         .count_prefix(), N = 100000, 32-bit values:       1142 ns   (R²=1.000, 927079 iterations in 119 samples)
        .search().next(), N = 100000, 32-bit values:      24222 ns   (R²=1.000, 43895 iterations in 87 samples)
 .search_prefix().next(), N = 100000, 32-bit values:      14190 ns   (R²=1.000, 70702 iterations in 92 samples)
               .median(), N = 100000, 32-bit values:       3209 ns   (R²=1.000, 324928 iterations in 108 samples)
             .quantile(), N = 100000, 32-bit values:       3156 ns   (R²=1.000, 324928 iterations in 108 samples)
          .sum_ex3(k=16), N = 100000, 32-bit values:       7106 ns   (R²=1.000, 151574 iterations in 100 samples)
         .sum_ex3(k=256), N = 100000, 32-bit values:      99252 ns   (R²=1.000, 10498 iterations in 72 samples)
```

(wait for cpu cooldown)         21 ns   (R²=1.000, 41959424 iterations in 159 samples)
### N = 10000

```
    WaveletMatrix::new(), N = 10000, 32-bit values:    2684836 ns   (only 1 iteration)
           .lookup(rand), N = 10000, 32-bit values:       1535 ns   (R²=1.000, 696525 iterations in 116 samples)
               vec[rand], N = 10000, 32-bit values:         22 ns   (R²=1.000, 41959424 iterations in 159 samples)
               rand only, N = 10000, 32-bit values:         22 ns   (R²=1.000, 38144930 iterations in 158 samples)
                 .rank(), N = 10000, 32-bit values:       2149 ns   (R²=1.000, 475732 iterations in 112 samples)
               .select(), N = 10000, 32-bit values:      14316 ns   (R²=1.000, 70702 iterations in 92 samples)
                .count(), N = 10000, 32-bit values:       2408 ns   (R²=1.000, 432483 iterations in 111 samples)
         .count_prefix(), N = 10000, 32-bit values:       1147 ns   (R²=1.000, 927079 iterations in 119 samples)
        .search().next(), N = 10000, 32-bit values:      19389 ns   (R²=1.000, 53116 iterations in 89 samples)
 .search_prefix().next(), N = 10000, 32-bit values:      11058 ns   (R²=1.000, 94110 iterations in 95 samples)
               .median(), N = 10000, 32-bit values:       2811 ns   (R²=1.000, 357422 iterations in 109 samples)
             .quantile(), N = 10000, 32-bit values:       2863 ns   (R²=1.000, 357422 iterations in 109 samples)
          .sum_ex3(k=16), N = 10000, 32-bit values:       5566 ns   (R²=1.000, 183408 iterations in 102 samples)
         .sum_ex3(k=256), N = 10000, 32-bit values:      98515 ns   (R²=1.000, 10498 iterations in 72 samples)
```

(wait for cpu cooldown)         21 ns   (R²=1.000, 41959424 iterations in 159 samples)
### N = 1000

```
    WaveletMatrix::new(), N = 1000, 32-bit values:     294449 ns   (only 1 iteration)
           .lookup(rand), N = 1000, 32-bit values:       1442 ns   (R²=1.000, 696525 iterations in 116 samples)
               vec[rand], N = 1000, 32-bit values:         22 ns   (R²=1.000, 38144930 iterations in 158 samples)
               rand only, N = 1000, 32-bit values:         22 ns   (R²=1.000, 38144930 iterations in 158 samples)
                 .rank(), N = 1000, 32-bit values:       2316 ns   (R²=1.000, 432483 iterations in 111 samples)
               .select(), N = 1000, 32-bit values:      10579 ns   (R²=1.000, 103522 iterations in 96 samples)
                .count(), N = 1000, 32-bit values:       2217 ns   (R²=1.000, 475732 iterations in 112 samples)
         .count_prefix(), N = 1000, 32-bit values:       1117 ns   (R²=1.000, 927079 iterations in 119 samples)
        .search().next(), N = 1000, 32-bit values:      14375 ns   (R²=1.000, 70702 iterations in 92 samples)
 .search_prefix().next(), N = 1000, 32-bit values:       6931 ns   (R²=1.000, 151574 iterations in 100 samples)
               .median(), N = 1000, 32-bit values:       2737 ns   (R²=0.999, 393165 iterations in 110 samples)
             .quantile(), N = 1000, 32-bit values:       2588 ns   (R²=1.000, 393165 iterations in 110 samples)
          .sum_ex3(k=16), N = 1000, 32-bit values:       6541 ns   (R²=1.000, 166733 iterations in 101 samples)
         .sum_ex3(k=256), N = 1000, 32-bit values:     183722 ns   (R²=1.000, 5919 iterations in 66 samples)
```

## 64-bit Values

(wait for cpu cooldown)         21 ns   (R²=1.000, 41959424 iterations in 159 samples)
### N = 10000000

```
    WaveletMatrix::new(), N = 10000000, 64-bit values: 6084333720 ns   (only 1 iteration)
           .lookup(rand), N = 10000000, 64-bit values:       3754 ns   (R²=1.000, 268533 iterations in 106 samples)
               vec[rand], N = 10000000, 64-bit values:         23 ns   (R²=1.000, 41959424 iterations in 159 samples)
               rand only, N = 10000000, 64-bit values:         23 ns   (R²=1.000, 38144930 iterations in 158 samples)
                 .rank(), N = 10000000, 64-bit values:       4992 ns   (R²=0.999, 201750 iterations in 103 samples)
               .select(), N = 10000000, 64-bit values:      59279 ns   (R²=0.999, 16915 iterations in 77 samples)
                .count(), N = 10000000, 64-bit values:       4991 ns   (R²=1.000, 201750 iterations in 103 samples)
         .count_prefix(), N = 10000000, 64-bit values:       2388 ns   (R²=0.999, 432483 iterations in 111 samples)
        .search().next(), N = 10000000, 64-bit values:      74036 ns   (R²=1.000, 13977 iterations in 75 samples)
 .search_prefix().next(), N = 10000000, 64-bit values:      39650 ns   (R²=1.000, 27250 iterations in 82 samples)
               .median(), N = 10000000, 64-bit values:       5911 ns   (R²=1.000, 183408 iterations in 102 samples)
             .quantile(), N = 10000000, 64-bit values:       6113 ns   (R²=1.000, 166733 iterations in 101 samples)
          .sum_ex3(k=16), N = 10000000, 64-bit values:       5183 ns   (R²=1.000, 201750 iterations in 103 samples)
         .sum_ex3(k=256), N = 10000000, 64-bit values:     106905 ns   (R²=1.000, 9542 iterations in 71 samples)
```

(wait for cpu cooldown)         21 ns   (R²=1.000, 41959424 iterations in 159 samples)
### N = 1000000

```
    WaveletMatrix::new(), N = 1000000, 64-bit values:  671108633 ns   (only 1 iteration)
           .lookup(rand), N = 1000000, 64-bit values:       2839 ns   (R²=1.000, 357422 iterations in 109 samples)
               vec[rand], N = 1000000, 64-bit values:         22 ns   (R²=1.000, 38144930 iterations in 158 samples)
               rand only, N = 1000000, 64-bit values:         23 ns   (R²=1.000, 38144930 iterations in 158 samples)
                 .rank(), N = 1000000, 64-bit values:       4811 ns   (R²=1.000, 221926 iterations in 104 samples)
               .select(), N = 1000000, 64-bit values:      48121 ns   (R²=1.000, 22518 iterations in 80 samples)
                .count(), N = 1000000, 64-bit values:       4985 ns   (R²=1.000, 201750 iterations in 103 samples)
         .count_prefix(), N = 1000000, 64-bit values:       2468 ns   (R²=1.000, 432483 iterations in 111 samples)
        .search().next(), N = 1000000, 64-bit values:      59725 ns   (R²=1.000, 16915 iterations in 77 samples)
 .search_prefix().next(), N = 1000000, 64-bit values:      30100 ns   (R²=1.000, 36274 iterations in 85 samples)
               .median(), N = 1000000, 64-bit values:       5731 ns   (R²=1.000, 183408 iterations in 102 samples)
             .quantile(), N = 1000000, 64-bit values:       5696 ns   (R²=1.000, 183408 iterations in 102 samples)
          .sum_ex3(k=16), N = 1000000, 64-bit values:       6057 ns   (R²=0.998, 166733 iterations in 101 samples)
         .sum_ex3(k=256), N = 1000000, 64-bit values:     108078 ns   (R²=1.000, 9542 iterations in 71 samples)
```

(wait for cpu cooldown)         21 ns   (R²=1.000, 41959424 iterations in 159 samples)
### N = 100000

```
    WaveletMatrix::new(), N = 100000, 64-bit values:   51164421 ns   (only 1 iteration)
           .lookup(rand), N = 100000, 64-bit values:       3095 ns   (R²=1.000, 324928 iterations in 108 samples)
               vec[rand], N = 100000, 64-bit values:         22 ns   (R²=1.000, 38144930 iterations in 158 samples)
               rand only, N = 100000, 64-bit values:         22 ns   (R²=1.000, 38144930 iterations in 158 samples)
                 .rank(), N = 100000, 64-bit values:       4689 ns   (R²=1.000, 221926 iterations in 104 samples)
               .select(), N = 100000, 64-bit values:      36599 ns   (R²=1.000, 29976 iterations in 83 samples)
                .count(), N = 100000, 64-bit values:       4819 ns   (R²=1.000, 221926 iterations in 104 samples)
         .count_prefix(), N = 100000, 64-bit values:       2236 ns   (R²=1.000, 475732 iterations in 112 samples)
        .search().next(), N = 100000, 64-bit values:      44625 ns   (R²=1.000, 22518 iterations in 80 samples)
 .search_prefix().next(), N = 100000, 64-bit values:      26002 ns   (R²=1.000, 39903 iterations in 86 samples)
               .median(), N = 100000, 64-bit values:       5676 ns   (R²=1.000, 183408 iterations in 102 samples)
             .quantile(), N = 100000, 64-bit values:       6564 ns   (R²=1.000, 166733 iterations in 101 samples)
          .sum_ex3(k=16), N = 100000, 64-bit values:       5136 ns   (R²=1.000, 201750 iterations in 103 samples)
         .sum_ex3(k=256), N = 100000, 64-bit values:     107403 ns   (R²=1.000, 9542 iterations in 71 samples)
```

(wait for cpu cooldown)         21 ns   (R²=1.000, 41959424 iterations in 159 samples)
### N = 10000

```
    WaveletMatrix::new(), N = 10000, 64-bit values:    5193883 ns   (only 1 iteration)
           .lookup(rand), N = 10000, 64-bit values:       2697 ns   (R²=1.000, 393165 iterations in 110 samples)
               vec[rand], N = 10000, 64-bit values:         22 ns   (R²=1.000, 38144930 iterations in 158 samples)
               rand only, N = 10000, 64-bit values:         22 ns   (R²=1.000, 38144930 iterations in 158 samples)
                 .rank(), N = 10000, 64-bit values:       4690 ns   (R²=1.000, 221926 iterations in 104 samples)
               .select(), N = 10000, 64-bit values:      29097 ns   (R²=1.000, 36274 iterations in 85 samples)
                .count(), N = 10000, 64-bit values:       4576 ns   (R²=1.000, 221926 iterations in 104 samples)
         .count_prefix(), N = 10000, 64-bit values:       2136 ns   (R²=1.000, 475732 iterations in 112 samples)
        .search().next(), N = 10000, 64-bit values:      35700 ns   (R²=1.000, 29976 iterations in 83 samples)
 .search_prefix().next(), N = 10000, 64-bit values:      19366 ns   (R²=1.000, 53116 iterations in 89 samples)
               .median(), N = 10000, 64-bit values:       5898 ns   (R²=1.000, 183408 iterations in 102 samples)
             .quantile(), N = 10000, 64-bit values:       5487 ns   (R²=0.998, 201750 iterations in 103 samples)
          .sum_ex3(k=16), N = 10000, 64-bit values:       5114 ns   (R²=1.000, 201750 iterations in 103 samples)
         .sum_ex3(k=256), N = 10000, 64-bit values:     101342 ns   (R²=1.000, 10498 iterations in 72 samples)
```

(wait for cpu cooldown)         21 ns   (R²=1.000, 41959424 iterations in 159 samples)
### N = 1000

```
    WaveletMatrix::new(), N = 1000, 64-bit values:     558507 ns   (only 1 iteration)
           .lookup(rand), N = 1000, 64-bit values:       2852 ns   (R²=1.000, 357422 iterations in 109 samples)
               vec[rand], N = 1000, 64-bit values:         22 ns   (R²=1.000, 38144930 iterations in 158 samples)
               rand only, N = 1000, 64-bit values:         22 ns   (R²=1.000, 38144930 iterations in 158 samples)
                 .rank(), N = 1000, 64-bit values:       4582 ns   (R²=1.000, 221926 iterations in 104 samples)
               .select(), N = 1000, 64-bit values:      21465 ns   (R²=1.000, 48286 iterations in 88 samples)
                .count(), N = 1000, 64-bit values:       4596 ns   (R²=1.000, 221926 iterations in 104 samples)
         .count_prefix(), N = 1000, 64-bit values:       2316 ns   (R²=1.000, 432483 iterations in 111 samples)
        .search().next(), N = 1000, 64-bit values:      27322 ns   (R²=1.000, 39903 iterations in 86 samples)
 .search_prefix().next(), N = 1000, 64-bit values:      13666 ns   (R²=1.000, 77774 iterations in 93 samples)
               .median(), N = 1000, 64-bit values:       5788 ns   (R²=1.000, 183408 iterations in 102 samples)
             .quantile(), N = 1000, 64-bit values:       5507 ns   (R²=1.000, 183408 iterations in 102 samples)
          .sum_ex3(k=16), N = 1000, 64-bit values:       7320 ns   (R²=1.000, 137793 iterations in 99 samples)
         .sum_ex3(k=256), N = 1000, 64-bit values:     296972 ns   (R²=1.000, 3671 iterations in 61 samples)
```

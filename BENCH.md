
running 10 tests
test rsdic_simple::tests::rsdic_sanity ... ignored
test wavelet_matrix::tests::count_panic_end ... ignored
test wavelet_matrix::tests::count_panic_start ... ignored
test wavelet_matrix::tests::example ... ignored
test wavelet_matrix::tests::layers_4 ... ignored
test wavelet_matrix::tests::layers_64 ... ignored
test wavelet_matrix::tests::layers_7 ... ignored
test wavelet_matrix::tests::lookup_panic ... ignored
test wavelet_matrix::tests::new_panic ... ignored
test wavelet_matrix::tests::wavelet_matrix_sanity ... ignored

test result: ok. 0 passed; 0 failed; 10 ignored; 0 measured; 0 filtered out

(wait for cpu cooldown)         18 ns   (R²=1.000, 50770905 iterations in 161 samples)
# Overall Performance

## 16-bit Values

(wait for cpu cooldown)         18 ns   (R²=1.000, 50770905 iterations in 161 samples)
### N = 10000000

```
    WaveletMatrix::new(), N = 10000000, 16-bit values: 1567600914 ns   (only 1 iteration)
           .lookup(rand), N = 10000000, 16-bit values:        641 ns   (R²=1.000, 1642386 iterations in 125 samples)
               vec[rand], N = 10000000, 16-bit values:         19 ns   (R²=1.000, 46155367 iterations in 160 samples)
               rand only, N = 10000000, 16-bit values:         18 ns   (R²=1.000, 50770905 iterations in 161 samples)
                 .rank(), N = 10000000, 16-bit values:       1064 ns   (R²=1.000, 1019788 iterations in 120 samples)
               .select(), N = 10000000, 16-bit values:      14809 ns   (R²=1.000, 70702 iterations in 92 samples)
            .select_lt(), N = 10000000, 16-bit values:      36116 ns   (R²=1.000, 29976 iterations in 83 samples)
                .count(), N = 10000000, 16-bit values:       1319 ns   (R²=1.000, 766179 iterations in 117 samples)
         .count_prefix(), N = 10000000, 16-bit values:        600 ns   (R²=1.000, 1806626 iterations in 126 samples)
        .search().next(), N = 10000000, 16-bit values:      21881 ns   (R²=1.000, 48286 iterations in 88 samples)
 .search_prefix().next(), N = 10000000, 16-bit values:      10535 ns   (R²=1.000, 103522 iterations in 96 samples)
               .median(), N = 10000000, 16-bit values:       1318 ns   (R²=1.000, 766179 iterations in 117 samples)
             .quantile(), N = 10000000, 16-bit values:       1589 ns   (R²=1.000, 633203 iterations in 115 samples)
            .top_k(k=16), N = 10000000, 16-bit values:    4241699 ns   (R²=1.000, 243 iterations in 33 samples)
     .top_k_ranges(k=16), N = 10000000, 16-bit values:       5901 ns   (R²=1.000, 183408 iterations in 102 samples)
    .top_k_ranges(k=256), N = 10000000, 16-bit values:      91675 ns   (R²=1.000, 11549 iterations in 73 samples)
  .top_k_ranges(k=65536), N = 10000000, 16-bit values:    8210121 ns   (R²=1.000, 133 iterations in 27 samples)
          .sum_ex3(k=16), N = 10000000, 16-bit values:       6161 ns   (R²=1.000, 166733 iterations in 101 samples)
         .sum_ex3(k=256), N = 10000000, 16-bit values:      98538 ns   (R²=1.000, 10498 iterations in 72 samples)
```

(wait for cpu cooldown)         18 ns   (R²=1.000, 50770905 iterations in 161 samples)
### N = 1000000

```
    WaveletMatrix::new(), N = 1000000, 16-bit values:  166783153 ns   (only 1 iteration)
           .lookup(rand), N = 1000000, 16-bit values:        802 ns   (R²=1.000, 1357342 iterations in 123 samples)
               vec[rand], N = 1000000, 16-bit values:         19 ns   (R²=1.000, 46155367 iterations in 160 samples)
               rand only, N = 1000000, 16-bit values:         18 ns   (R²=1.000, 46155367 iterations in 160 samples)
                 .rank(), N = 1000000, 16-bit values:       1196 ns   (R²=1.000, 842798 iterations in 118 samples)
               .select(), N = 1000000, 16-bit values:      10993 ns   (R²=1.000, 94110 iterations in 95 samples)
            .select_lt(), N = 1000000, 16-bit values:      44536 ns   (R²=1.000, 22518 iterations in 80 samples)
                .count(), N = 1000000, 16-bit values:       1185 ns   (R²=1.000, 927079 iterations in 119 samples)
         .count_prefix(), N = 1000000, 16-bit values:        602 ns   (R²=1.000, 1806626 iterations in 126 samples)
        .search().next(), N = 1000000, 16-bit values:      17889 ns   (R²=1.000, 58429 iterations in 90 samples)
 .search_prefix().next(), N = 1000000, 16-bit values:       8432 ns   (R²=1.000, 125265 iterations in 98 samples)
               .median(), N = 1000000, 16-bit values:       1206 ns   (R²=1.000, 842798 iterations in 118 samples)
             .quantile(), N = 1000000, 16-bit values:       1577 ns   (R²=1.000, 633203 iterations in 115 samples)
            .top_k(k=16), N = 1000000, 16-bit values:     138762 ns   (R²=1.000, 7883 iterations in 69 samples)
     .top_k_ranges(k=16), N = 1000000, 16-bit values:       6193 ns   (R²=1.000, 166733 iterations in 101 samples)
    .top_k_ranges(k=256), N = 1000000, 16-bit values:     100934 ns   (R²=1.000, 10498 iterations in 72 samples)
  .top_k_ranges(k=65536), N = 1000000, 16-bit values:     400524 ns   (R²=1.000, 2503 iterations in 57 samples)
          .sum_ex3(k=16), N = 1000000, 16-bit values:       6485 ns   (R²=1.000, 166733 iterations in 101 samples)
         .sum_ex3(k=256), N = 1000000, 16-bit values:      95116 ns   (R²=1.000, 11549 iterations in 73 samples)
```

(wait for cpu cooldown)         18 ns   (R²=1.000, 50770905 iterations in 161 samples)
### N = 100000

```
    WaveletMatrix::new(), N = 100000, 16-bit values:   13933270 ns   (only 1 iteration)
           .lookup(rand), N = 100000, 16-bit values:        839 ns   (R²=1.000, 1233946 iterations in 122 samples)
               vec[rand], N = 100000, 16-bit values:         19 ns   (R²=1.000, 46155367 iterations in 160 samples)
               rand only, N = 100000, 16-bit values:         18 ns   (R²=1.000, 46155367 iterations in 160 samples)
                 .rank(), N = 100000, 16-bit values:       1152 ns   (R²=1.000, 927079 iterations in 119 samples)
               .select(), N = 100000, 16-bit values:       8776 ns   (R²=1.000, 113876 iterations in 97 samples)
            .select_lt(), N = 100000, 16-bit values:      31674 ns   (R²=1.000, 32975 iterations in 84 samples)
                .count(), N = 100000, 16-bit values:       1201 ns   (R²=1.000, 842798 iterations in 118 samples)
         .count_prefix(), N = 100000, 16-bit values:        642 ns   (R²=1.000, 1642386 iterations in 125 samples)
        .search().next(), N = 100000, 16-bit values:      14455 ns   (R²=1.000, 70702 iterations in 92 samples)
 .search_prefix().next(), N = 100000, 16-bit values:       7589 ns   (R²=1.000, 137793 iterations in 99 samples)
               .median(), N = 100000, 16-bit values:       1186 ns   (R²=1.000, 842798 iterations in 118 samples)
             .quantile(), N = 100000, 16-bit values:       1633 ns   (R²=1.000, 633203 iterations in 115 samples)
            .top_k(k=16), N = 100000, 16-bit values:     389070 ns   (R²=1.000, 2755 iterations in 58 samples)
     .top_k_ranges(k=16), N = 100000, 16-bit values:       5707 ns   (R²=1.000, 183408 iterations in 102 samples)
    .top_k_ranges(k=256), N = 100000, 16-bit values:     101650 ns   (R²=1.000, 10498 iterations in 72 samples)
  .top_k_ranges(k=65536), N = 100000, 16-bit values:    1195400 ns   (R²=1.000, 869 iterations in 46 samples)
          .sum_ex3(k=16), N = 100000, 16-bit values:       6069 ns   (R²=1.000, 166733 iterations in 101 samples)
         .sum_ex3(k=256), N = 100000, 16-bit values:      94737 ns   (R²=1.000, 11549 iterations in 73 samples)
```

(wait for cpu cooldown)         18 ns   (R²=1.000, 50770905 iterations in 161 samples)
### N = 10000

```
    WaveletMatrix::new(), N = 10000, 16-bit values:    1372453 ns   (only 1 iteration)
           .lookup(rand), N = 10000, 16-bit values:        721 ns   (R²=1.000, 1493077 iterations in 124 samples)
               vec[rand], N = 10000, 16-bit values:         19 ns   (R²=1.000, 46155367 iterations in 160 samples)
               rand only, N = 10000, 16-bit values:         18 ns   (R²=1.000, 46155367 iterations in 160 samples)
                 .rank(), N = 10000, 16-bit values:       1250 ns   (R²=1.000, 842798 iterations in 118 samples)
               .select(), N = 10000, 16-bit values:       8028 ns   (R²=1.000, 125265 iterations in 98 samples)
            .select_lt(), N = 10000, 16-bit values:      34791 ns   (R²=1.000, 29976 iterations in 83 samples)
                .count(), N = 10000, 16-bit values:       1294 ns   (R²=1.000, 842798 iterations in 118 samples)
         .count_prefix(), N = 10000, 16-bit values:        624 ns   (R²=1.000, 1642386 iterations in 125 samples)
        .search().next(), N = 10000, 16-bit values:      11186 ns   (R²=1.000, 94110 iterations in 95 samples)
 .search_prefix().next(), N = 10000, 16-bit values:       6615 ns   (R²=1.000, 151574 iterations in 100 samples)
               .median(), N = 10000, 16-bit values:       1679 ns   (R²=1.000, 633203 iterations in 115 samples)
             .quantile(), N = 10000, 16-bit values:       1589 ns   (R²=1.000, 633203 iterations in 115 samples)
            .top_k(k=16), N = 10000, 16-bit values:      93205 ns   (R²=1.000, 11549 iterations in 73 samples)
     .top_k_ranges(k=16), N = 10000, 16-bit values:       5859 ns   (R²=1.000, 183408 iterations in 102 samples)
    .top_k_ranges(k=256), N = 10000, 16-bit values:     377702 ns   (R²=1.000, 2755 iterations in 58 samples)
  .top_k_ranges(k=65536), N = 10000, 16-bit values:     377772 ns   (R²=1.000, 2755 iterations in 58 samples)
          .sum_ex3(k=16), N = 10000, 16-bit values:       6156 ns   (R²=1.000, 166733 iterations in 101 samples)
         .sum_ex3(k=256), N = 10000, 16-bit values:     379384 ns   (R²=1.000, 2755 iterations in 58 samples)
```

(wait for cpu cooldown)         18 ns   (R²=1.000, 50770905 iterations in 161 samples)
### N = 1000

```
    WaveletMatrix::new(), N = 1000, 16-bit values:     166436 ns   (only 1 iteration)
           .lookup(rand), N = 1000, 16-bit values:        589 ns   (R²=1.000, 1806626 iterations in 126 samples)
               vec[rand], N = 1000, 16-bit values:         18 ns   (R²=1.000, 46155367 iterations in 160 samples)
               rand only, N = 1000, 16-bit values:         18 ns   (R²=1.000, 46155367 iterations in 160 samples)
                 .rank(), N = 1000, 16-bit values:       1128 ns   (R²=1.000, 927079 iterations in 119 samples)
               .select(), N = 1000, 16-bit values:       5173 ns   (R²=1.000, 201750 iterations in 103 samples)
            .select_lt(), N = 1000, 16-bit values:      22331 ns   (R²=1.000, 48286 iterations in 88 samples)
                .count(), N = 1000, 16-bit values:       1038 ns   (R²=1.000, 1019788 iterations in 120 samples)
         .count_prefix(), N = 1000, 16-bit values:        540 ns   (R²=1.000, 1987290 iterations in 127 samples)
        .search().next(), N = 1000, 16-bit values:       8144 ns   (R²=1.000, 125265 iterations in 98 samples)
 .search_prefix().next(), N = 1000, 16-bit values:       4554 ns   (R²=1.000, 221926 iterations in 104 samples)
               .median(), N = 1000, 16-bit values:       1562 ns   (R²=1.000, 696525 iterations in 116 samples)
             .quantile(), N = 1000, 16-bit values:       1255 ns   (R²=1.000, 842798 iterations in 118 samples)
            .top_k(k=16), N = 1000, 16-bit values:      78675 ns   (R²=1.000, 12705 iterations in 74 samples)
     .top_k_ranges(k=16), N = 1000, 16-bit values:       4697 ns   (R²=1.000, 221926 iterations in 104 samples)
    .top_k_ranges(k=256), N = 1000, 16-bit values:     313116 ns   (R²=1.000, 3336 iterations in 60 samples)
  .top_k_ranges(k=65536), N = 1000, 16-bit values:     313315 ns   (R²=1.000, 3336 iterations in 60 samples)
          .sum_ex3(k=16), N = 1000, 16-bit values:       5000 ns   (R²=1.000, 201750 iterations in 103 samples)
         .sum_ex3(k=256), N = 1000, 16-bit values:     315594 ns   (R²=1.000, 3336 iterations in 60 samples)
```

## 32-bit Values

(wait for cpu cooldown)         18 ns   (R²=1.000, 50770905 iterations in 161 samples)
### N = 10000000

```
    WaveletMatrix::new(), N = 10000000, 32-bit values: 3074784747 ns   (only 1 iteration)
           .lookup(rand), N = 10000000, 32-bit values:       1846 ns   (R²=1.000, 575638 iterations in 114 samples)
               vec[rand], N = 10000000, 32-bit values:         19 ns   (R²=1.000, 46155367 iterations in 160 samples)
               rand only, N = 10000000, 32-bit values:         18 ns   (R²=1.000, 50770905 iterations in 161 samples)
                 .rank(), N = 10000000, 32-bit values:       2681 ns   (R²=1.000, 393165 iterations in 110 samples)
               .select(), N = 10000000, 32-bit values:      56231 ns   (R²=1.000, 18608 iterations in 78 samples)
            .select_lt(), N = 10000000, 32-bit values:     122726 ns   (R²=1.000, 8673 iterations in 70 samples)
                .count(), N = 10000000, 32-bit values:       2586 ns   (R²=1.000, 393165 iterations in 110 samples)
         .count_prefix(), N = 10000000, 32-bit values:       1404 ns   (R²=1.000, 766179 iterations in 117 samples)
        .search().next(), N = 10000000, 32-bit values:      44335 ns   (R²=1.000, 24771 iterations in 81 samples)
 .search_prefix().next(), N = 10000000, 32-bit values:      21653 ns   (R²=1.000, 48286 iterations in 88 samples)
               .median(), N = 10000000, 32-bit values:       3091 ns   (R²=1.000, 324928 iterations in 108 samples)
             .quantile(), N = 10000000, 32-bit values:       3105 ns   (R²=1.000, 324928 iterations in 108 samples)
            .top_k(k=16), N = 10000000, 32-bit values: Only generated 4 sample(s) - we can't fit a regression line to that! Try making your benchmark faster.
     .top_k_ranges(k=16), N = 10000000, 32-bit values:       5209 ns   (R²=1.000, 201750 iterations in 103 samples)
    .top_k_ranges(k=256), N = 10000000, 32-bit values:      91538 ns   (R²=1.000, 11549 iterations in 73 samples)
  .top_k_ranges(k=65536), N = 10000000, 32-bit values:   43091566 ns   (R²=0.998, 23 iterations in 12 samples)
          .sum_ex3(k=16), N = 10000000, 32-bit values:       5427 ns   (R²=1.000, 183408 iterations in 102 samples)
         .sum_ex3(k=256), N = 10000000, 32-bit values:      97829 ns   (R²=1.000, 10498 iterations in 72 samples)
```

(wait for cpu cooldown)         18 ns   (R²=1.000, 50770905 iterations in 161 samples)
### N = 1000000

```
    WaveletMatrix::new(), N = 1000000, 32-bit values:  305564817 ns   (only 1 iteration)
           .lookup(rand), N = 1000000, 32-bit values:       1599 ns   (R²=1.000, 633203 iterations in 115 samples)
               vec[rand], N = 1000000, 32-bit values:         19 ns   (R²=1.000, 46155367 iterations in 160 samples)
               rand only, N = 1000000, 32-bit values:         19 ns   (R²=1.000, 46155367 iterations in 160 samples)
                 .rank(), N = 1000000, 32-bit values:       2232 ns   (R²=1.000, 475732 iterations in 112 samples)
               .select(), N = 1000000, 32-bit values:      21917 ns   (R²=1.000, 48286 iterations in 88 samples)
            .select_lt(), N = 1000000, 32-bit values:      60624 ns   (R²=1.000, 16915 iterations in 77 samples)
                .count(), N = 1000000, 32-bit values:       2650 ns   (R²=1.000, 393165 iterations in 110 samples)
         .count_prefix(), N = 1000000, 32-bit values:       1274 ns   (R²=1.000, 842798 iterations in 118 samples)
        .search().next(), N = 1000000, 32-bit values:      31141 ns   (R²=1.000, 32975 iterations in 84 samples)
 .search_prefix().next(), N = 1000000, 32-bit values:      17124 ns   (R²=1.000, 58429 iterations in 90 samples)
               .median(), N = 1000000, 32-bit values:       3132 ns   (R²=1.000, 324928 iterations in 108 samples)
             .quantile(), N = 1000000, 32-bit values:       2979 ns   (R²=1.000, 357422 iterations in 109 samples)
            .top_k(k=16), N = 1000000, 32-bit values:   15013804 ns   (R²=0.999, 70 iterations in 21 samples)
     .top_k_ranges(k=16), N = 1000000, 32-bit values:       5372 ns   (R²=1.000, 201750 iterations in 103 samples)
    .top_k_ranges(k=256), N = 1000000, 32-bit values:      91477 ns   (R²=1.000, 11549 iterations in 73 samples)
  .top_k_ranges(k=65536), N = 1000000, 32-bit values:  144852153 ns   (R²=1.000, 6 iterations in 5 samples)
          .sum_ex3(k=16), N = 1000000, 32-bit values:       5632 ns   (R²=1.000, 183408 iterations in 102 samples)
         .sum_ex3(k=256), N = 1000000, 32-bit values:      95726 ns   (R²=1.000, 10498 iterations in 72 samples)
```

(wait for cpu cooldown)         18 ns   (R²=1.000, 50770905 iterations in 161 samples)
### N = 100000

```
    WaveletMatrix::new(), N = 100000, 32-bit values:   26177218 ns   (only 1 iteration)
           .lookup(rand), N = 100000, 32-bit values:       1805 ns   (R²=1.000, 575638 iterations in 114 samples)
               vec[rand], N = 100000, 32-bit values:         18 ns   (R²=1.000, 46155367 iterations in 160 samples)
               rand only, N = 100000, 32-bit values:         18 ns   (R²=1.000, 46155367 iterations in 160 samples)
                 .rank(), N = 100000, 32-bit values:       2392 ns   (R²=1.000, 432483 iterations in 111 samples)
               .select(), N = 100000, 32-bit values:      18016 ns   (R²=1.000, 58429 iterations in 90 samples)
            .select_lt(), N = 100000, 32-bit values:      54864 ns   (R²=1.000, 18608 iterations in 78 samples)
                .count(), N = 100000, 32-bit values:       2398 ns   (R²=1.000, 432483 iterations in 111 samples)
         .count_prefix(), N = 100000, 32-bit values:       1155 ns   (R²=1.000, 927079 iterations in 119 samples)
        .search().next(), N = 100000, 32-bit values:      26205 ns   (R²=1.000, 39903 iterations in 86 samples)
 .search_prefix().next(), N = 100000, 32-bit values:      15223 ns   (R²=1.000, 70702 iterations in 92 samples)
               .median(), N = 100000, 32-bit values:       2797 ns   (R²=1.000, 357422 iterations in 109 samples)
             .quantile(), N = 100000, 32-bit values:       3175 ns   (R²=1.000, 324928 iterations in 108 samples)
            .top_k(k=16), N = 100000, 32-bit values:    6527452 ns   (R²=1.000, 163 iterations in 29 samples)
     .top_k_ranges(k=16), N = 100000, 32-bit values:       5164 ns   (R²=1.000, 201750 iterations in 103 samples)
    .top_k_ranges(k=256), N = 100000, 32-bit values:      93326 ns   (R²=1.000, 11549 iterations in 73 samples)
  .top_k_ranges(k=65536), N = 100000, 32-bit values:   70440909 ns   (R²=1.000, 14 iterations in 9 samples)
          .sum_ex3(k=16), N = 100000, 32-bit values:       5526 ns   (R²=1.000, 183408 iterations in 102 samples)
         .sum_ex3(k=256), N = 100000, 32-bit values:      96592 ns   (R²=1.000, 10498 iterations in 72 samples)
```

(wait for cpu cooldown)         18 ns   (R²=1.000, 46155367 iterations in 160 samples)
### N = 10000

```
    WaveletMatrix::new(), N = 10000, 32-bit values:    2642605 ns   (only 1 iteration)
           .lookup(rand), N = 10000, 32-bit values:       1468 ns   (R²=1.000, 696525 iterations in 116 samples)
               vec[rand], N = 10000, 32-bit values:         18 ns   (R²=1.000, 46155367 iterations in 160 samples)
               rand only, N = 10000, 32-bit values:         18 ns   (R²=1.000, 46155367 iterations in 160 samples)
                 .rank(), N = 10000, 32-bit values:       2272 ns   (R²=1.000, 475732 iterations in 112 samples)
               .select(), N = 10000, 32-bit values:      15531 ns   (R²=1.000, 70702 iterations in 92 samples)
            .select_lt(), N = 10000, 32-bit values:      55469 ns   (R²=1.000, 18608 iterations in 78 samples)
                .count(), N = 10000, 32-bit values:       2520 ns   (R²=1.000, 432483 iterations in 111 samples)
         .count_prefix(), N = 10000, 32-bit values:       1317 ns   (R²=1.000, 766179 iterations in 117 samples)
        .search().next(), N = 10000, 32-bit values:      20367 ns   (R²=1.000, 53116 iterations in 89 samples)
 .search_prefix().next(), N = 10000, 32-bit values:      11192 ns   (R²=1.000, 94110 iterations in 95 samples)
               .median(), N = 10000, 32-bit values:       2934 ns   (R²=1.000, 357422 iterations in 109 samples)
             .quantile(), N = 10000, 32-bit values:       2849 ns   (R²=1.000, 357422 iterations in 109 samples)
            .top_k(k=16), N = 10000, 32-bit values:     336383 ns   (R²=1.000, 3032 iterations in 59 samples)
     .top_k_ranges(k=16), N = 10000, 32-bit values:       5961 ns   (R²=1.000, 183408 iterations in 102 samples)
    .top_k_ranges(k=256), N = 10000, 32-bit values:     103863 ns   (R²=1.000, 10498 iterations in 72 samples)
  .top_k_ranges(k=65536), N = 10000, 32-bit values:    3473914 ns   (R²=1.000, 297 iterations in 35 samples)
          .sum_ex3(k=16), N = 10000, 32-bit values:       6327 ns   (R²=1.000, 166733 iterations in 101 samples)
         .sum_ex3(k=256), N = 10000, 32-bit values:      97958 ns   (R²=1.000, 10498 iterations in 72 samples)
```

(wait for cpu cooldown)         18 ns   (R²=1.000, 46155367 iterations in 160 samples)
### N = 1000

```
    WaveletMatrix::new(), N = 1000, 32-bit values:     298251 ns   (only 1 iteration)
           .lookup(rand), N = 1000, 32-bit values:       1510 ns   (R²=1.000, 696525 iterations in 116 samples)
               vec[rand], N = 1000, 32-bit values:         18 ns   (R²=1.000, 46155367 iterations in 160 samples)
               rand only, N = 1000, 32-bit values:         18 ns   (R²=1.000, 46155367 iterations in 160 samples)
                 .rank(), N = 1000, 32-bit values:       2141 ns   (R²=1.000, 475732 iterations in 112 samples)
               .select(), N = 1000, 32-bit values:      10275 ns   (R²=1.000, 103522 iterations in 96 samples)
            .select_lt(), N = 1000, 32-bit values:      44027 ns   (R²=1.000, 24771 iterations in 81 samples)
                .count(), N = 1000, 32-bit values:       2293 ns   (R²=1.000, 475732 iterations in 112 samples)
         .count_prefix(), N = 1000, 32-bit values:       1222 ns   (R²=1.000, 842798 iterations in 118 samples)
        .search().next(), N = 1000, 32-bit values:      14844 ns   (R²=1.000, 70702 iterations in 92 samples)
 .search_prefix().next(), N = 1000, 32-bit values:       7667 ns   (R²=1.000, 137793 iterations in 99 samples)
               .median(), N = 1000, 32-bit values:       2853 ns   (R²=1.000, 357422 iterations in 109 samples)
             .quantile(), N = 1000, 32-bit values:       2778 ns   (R²=1.000, 393165 iterations in 110 samples)
            .top_k(k=16), N = 1000, 32-bit values:      90373 ns   (R²=1.000, 11549 iterations in 73 samples)
     .top_k_ranges(k=16), N = 1000, 32-bit values:       6606 ns   (R²=1.000, 151574 iterations in 100 samples)
    .top_k_ranges(k=256), N = 1000, 32-bit values:     122460 ns   (R²=1.000, 8673 iterations in 70 samples)
  .top_k_ranges(k=65536), N = 1000, 32-bit values:     122334 ns   (R²=1.000, 8673 iterations in 70 samples)
          .sum_ex3(k=16), N = 1000, 32-bit values:       8710 ns   (R²=1.000, 125265 iterations in 98 samples)
         .sum_ex3(k=256), N = 1000, 32-bit values:     127358 ns   (R²=1.000, 7883 iterations in 69 samples)
```

## 64-bit Values

(wait for cpu cooldown)         18 ns   (R²=1.000, 46155367 iterations in 160 samples)
### N = 10000000

```
    WaveletMatrix::new(), N = 10000000, 64-bit values: 6165340320 ns   (only 1 iteration)
           .lookup(rand), N = 10000000, 64-bit values:       6413 ns   (R²=1.000, 166733 iterations in 101 samples)
               vec[rand], N = 10000000, 64-bit values:         19 ns   (R²=1.000, 46155367 iterations in 160 samples)
               rand only, N = 10000000, 64-bit values:         18 ns   (R²=1.000, 50770905 iterations in 161 samples)
                 .rank(), N = 10000000, 64-bit values:       7683 ns   (R²=1.000, 137793 iterations in 99 samples)
               .select(), N = 10000000, 64-bit values:     128185 ns   (R²=1.000, 7883 iterations in 69 samples)
            .select_lt(), N = 10000000, 64-bit values:     223516 ns   (R²=1.000, 4890 iterations in 64 samples)
                .count(), N = 10000000, 64-bit values:       8936 ns   (R²=1.000, 113876 iterations in 97 samples)
         .count_prefix(), N = 10000000, 64-bit values:       2741 ns   (R²=1.000, 393165 iterations in 110 samples)
        .search().next(), N = 10000000, 64-bit values:     125237 ns   (R²=1.000, 8673 iterations in 70 samples)
 .search_prefix().next(), N = 10000000, 64-bit values:      42978 ns   (R²=1.000, 24771 iterations in 81 samples)
               .median(), N = 10000000, 64-bit values:       7892 ns   (R²=0.996, 137793 iterations in 99 samples)
             .quantile(), N = 10000000, 64-bit values:       9052 ns   (R²=1.000, 113876 iterations in 97 samples)
            .top_k(k=16), N = 10000000, 64-bit values: Only generated 0 sample(s) - we can't fit a regression line to that! Try making your benchmark faster.
     .top_k_ranges(k=16), N = 10000000, 64-bit values:       5326 ns   (R²=1.000, 201750 iterations in 103 samples)
    .top_k_ranges(k=256), N = 10000000, 64-bit values:      90179 ns   (R²=1.000, 11549 iterations in 73 samples)
  .top_k_ranges(k=65536), N = 10000000, 64-bit values:   43711190 ns   (R²=0.997, 23 iterations in 12 samples)
          .sum_ex3(k=16), N = 10000000, 64-bit values:       5812 ns   (R²=1.000, 183408 iterations in 102 samples)
         .sum_ex3(k=256), N = 10000000, 64-bit values:     106596 ns   (R²=1.000, 9542 iterations in 71 samples)
```

(wait for cpu cooldown)         18 ns   (R²=1.000, 50770905 iterations in 161 samples)
### N = 1000000

```
    WaveletMatrix::new(), N = 1000000, 64-bit values:  643721809 ns   (only 1 iteration)
           .lookup(rand), N = 1000000, 64-bit values:       3364 ns   (R²=1.000, 324928 iterations in 108 samples)
               vec[rand], N = 1000000, 64-bit values:         19 ns   (R²=1.000, 46155367 iterations in 160 samples)
               rand only, N = 1000000, 64-bit values:         18 ns   (R²=1.000, 46155367 iterations in 160 samples)
                 .rank(), N = 1000000, 64-bit values:       5297 ns   (R²=1.000, 201750 iterations in 103 samples)
               .select(), N = 1000000, 64-bit values:      48043 ns   (R²=1.000, 22518 iterations in 80 samples)
            .select_lt(), N = 1000000, 64-bit values:     143972 ns   (R²=1.000, 7165 iterations in 68 samples)
                .count(), N = 1000000, 64-bit values:       5277 ns   (R²=1.000, 201750 iterations in 103 samples)
         .count_prefix(), N = 1000000, 64-bit values:       2524 ns   (R²=1.000, 432483 iterations in 111 samples)
        .search().next(), N = 1000000, 64-bit values:      59778 ns   (R²=1.000, 16915 iterations in 77 samples)
 .search_prefix().next(), N = 1000000, 64-bit values:      31425 ns   (R²=1.000, 32975 iterations in 84 samples)
               .median(), N = 1000000, 64-bit values:       5562 ns   (R²=1.000, 183408 iterations in 102 samples)
             .quantile(), N = 1000000, 64-bit values:       5998 ns   (R²=1.000, 166733 iterations in 101 samples)
            .top_k(k=16), N = 1000000, 64-bit values: Only generated 2 sample(s) - we can't fit a regression line to that! Try making your benchmark faster.
     .top_k_ranges(k=16), N = 1000000, 64-bit values:       4652 ns   (R²=1.000, 221926 iterations in 104 samples)
    .top_k_ranges(k=256), N = 1000000, 64-bit values:      89270 ns   (R²=1.000, 11549 iterations in 73 samples)
  .top_k_ranges(k=65536), N = 1000000, 64-bit values:   41024434 ns   (R²=0.999, 26 iterations in 13 samples)
          .sum_ex3(k=16), N = 1000000, 64-bit values:       5230 ns   (R²=1.000, 201750 iterations in 103 samples)
         .sum_ex3(k=256), N = 1000000, 64-bit values:     103763 ns   (R²=1.000, 10498 iterations in 72 samples)
```

(wait for cpu cooldown)         18 ns   (R²=1.000, 50770905 iterations in 161 samples)
### N = 100000

```
    WaveletMatrix::new(), N = 100000, 64-bit values:   51841138 ns   (only 1 iteration)
           .lookup(rand), N = 100000, 64-bit values:       3294 ns   (R²=1.000, 324928 iterations in 108 samples)
               vec[rand], N = 100000, 64-bit values:         18 ns   (R²=1.000, 46155367 iterations in 160 samples)
               rand only, N = 100000, 64-bit values:         18 ns   (R²=1.000, 46155367 iterations in 160 samples)
                 .rank(), N = 100000, 64-bit values:       4879 ns   (R²=1.000, 221926 iterations in 104 samples)
               .select(), N = 100000, 64-bit values:      36531 ns   (R²=1.000, 29976 iterations in 83 samples)
            .select_lt(), N = 100000, 64-bit values:     177656 ns   (R²=1.000, 5919 iterations in 66 samples)
                .count(), N = 100000, 64-bit values:       5168 ns   (R²=1.000, 201750 iterations in 103 samples)
         .count_prefix(), N = 100000, 64-bit values:       2558 ns   (R²=1.000, 393165 iterations in 110 samples)
        .search().next(), N = 100000, 64-bit values:      46268 ns   (R²=1.000, 22518 iterations in 80 samples)
 .search_prefix().next(), N = 100000, 64-bit values:      25886 ns   (R²=1.000, 39903 iterations in 86 samples)
               .median(), N = 100000, 64-bit values:       5850 ns   (R²=1.000, 183408 iterations in 102 samples)
             .quantile(), N = 100000, 64-bit values:       5646 ns   (R²=1.000, 183408 iterations in 102 samples)
            .top_k(k=16), N = 100000, 64-bit values:    2977391 ns   (R²=1.000, 362 iterations in 37 samples)
     .top_k_ranges(k=16), N = 100000, 64-bit values:       5289 ns   (R²=1.000, 201750 iterations in 103 samples)
    .top_k_ranges(k=256), N = 100000, 64-bit values:      92680 ns   (R²=1.000, 11549 iterations in 73 samples)
  .top_k_ranges(k=65536), N = 100000, 64-bit values:   84045470 ns   (R²=1.000, 12 iterations in 8 samples)
          .sum_ex3(k=16), N = 100000, 64-bit values:       5302 ns   (R²=1.000, 201750 iterations in 103 samples)
         .sum_ex3(k=256), N = 100000, 64-bit values:     102642 ns   (R²=1.000, 10498 iterations in 72 samples)
```

(wait for cpu cooldown)         18 ns   (R²=1.000, 50770905 iterations in 161 samples)
### N = 10000

```
    WaveletMatrix::new(), N = 10000, 64-bit values:    5180959 ns   (only 1 iteration)
           .lookup(rand), N = 10000, 64-bit values:       2725 ns   (R²=1.000, 393165 iterations in 110 samples)
               vec[rand], N = 10000, 64-bit values:         18 ns   (R²=1.000, 46155367 iterations in 160 samples)
               rand only, N = 10000, 64-bit values:         18 ns   (R²=1.000, 46155367 iterations in 160 samples)
                 .rank(), N = 10000, 64-bit values:       4956 ns   (R²=1.000, 201750 iterations in 103 samples)
               .select(), N = 10000, 64-bit values:      32023 ns   (R²=1.000, 32975 iterations in 84 samples)
            .select_lt(), N = 10000, 64-bit values:      98362 ns   (R²=1.000, 10498 iterations in 72 samples)
                .count(), N = 10000, 64-bit values:       4950 ns   (R²=1.000, 221926 iterations in 104 samples)
         .count_prefix(), N = 10000, 64-bit values:       2438 ns   (R²=1.000, 432483 iterations in 111 samples)
        .search().next(), N = 10000, 64-bit values:      39275 ns   (R²=1.000, 27250 iterations in 82 samples)
 .search_prefix().next(), N = 10000, 64-bit values:      20817 ns   (R²=1.000, 48286 iterations in 88 samples)
               .median(), N = 10000, 64-bit values:       5681 ns   (R²=1.000, 183408 iterations in 102 samples)
             .quantile(), N = 10000, 64-bit values:       5820 ns   (R²=1.000, 183408 iterations in 102 samples)
            .top_k(k=16), N = 10000, 64-bit values:     224888 ns   (R²=1.000, 4444 iterations in 63 samples)
     .top_k_ranges(k=16), N = 10000, 64-bit values:       7710 ns   (R²=1.000, 137793 iterations in 99 samples)
    .top_k_ranges(k=256), N = 10000, 64-bit values:     668357 ns   (R²=1.000, 1549 iterations in 52 samples)
  .top_k_ranges(k=65536), N = 10000, 64-bit values:     670251 ns   (R²=1.000, 1549 iterations in 52 samples)
          .sum_ex3(k=16), N = 10000, 64-bit values:       7402 ns   (R²=1.000, 137793 iterations in 99 samples)
         .sum_ex3(k=256), N = 10000, 64-bit values:     705965 ns   (R²=1.000, 1549 iterations in 52 samples)
```

(wait for cpu cooldown)         18 ns   (R²=1.000, 50770905 iterations in 161 samples)
### N = 1000

```
    WaveletMatrix::new(), N = 1000, 64-bit values:     552925 ns   (only 1 iteration)
           .lookup(rand), N = 1000, 64-bit values:       2813 ns   (R²=1.000, 357422 iterations in 109 samples)
               vec[rand], N = 1000, 64-bit values:         18 ns   (R²=1.000, 46155367 iterations in 160 samples)
               rand only, N = 1000, 64-bit values:         18 ns   (R²=1.000, 46155367 iterations in 160 samples)
                 .rank(), N = 1000, 64-bit values:       4498 ns   (R²=1.000, 244120 iterations in 105 samples)
               .select(), N = 1000, 64-bit values:      20729 ns   (R²=1.000, 48286 iterations in 88 samples)
            .select_lt(), N = 1000, 64-bit values:      61857 ns   (R²=1.000, 16915 iterations in 77 samples)
                .count(), N = 1000, 64-bit values:       4449 ns   (R²=1.000, 244120 iterations in 105 samples)
         .count_prefix(), N = 1000, 64-bit values:       2234 ns   (R²=1.000, 475732 iterations in 112 samples)
        .search().next(), N = 1000, 64-bit values:      26600 ns   (R²=1.000, 39903 iterations in 86 samples)
 .search_prefix().next(), N = 1000, 64-bit values:      15167 ns   (R²=1.000, 70702 iterations in 92 samples)
               .median(), N = 1000, 64-bit values:       5760 ns   (R²=1.000, 183408 iterations in 102 samples)
             .quantile(), N = 1000, 64-bit values:       5660 ns   (R²=1.000, 183408 iterations in 102 samples)
            .top_k(k=16), N = 1000, 64-bit values:      25803 ns   (R²=1.000, 39903 iterations in 86 samples)
     .top_k_ranges(k=16), N = 1000, 64-bit values:      25878 ns   (R²=1.000, 39903 iterations in 86 samples)
    .top_k_ranges(k=256), N = 1000, 64-bit values:      25888 ns   (R²=1.000, 39903 iterations in 86 samples)
  .top_k_ranges(k=65536), N = 1000, 64-bit values:      25884 ns   (R²=1.000, 39903 iterations in 86 samples)
          .sum_ex3(k=16), N = 1000, 64-bit values:      25550 ns   (R²=1.000, 39903 iterations in 86 samples)
         .sum_ex3(k=256), N = 1000, 64-bit values:      25548 ns   (R²=1.000, 39903 iterations in 86 samples)
```


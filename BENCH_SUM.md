
running 6 tests
test rsdic_simple::tests::rsdic_sanity ... ignored
test wavelet_matrix::tests::example ... ignored
test wavelet_matrix::tests::layers_4 ... ignored
test wavelet_matrix::tests::layers_64 ... ignored
test wavelet_matrix::tests::layers_7 ... ignored
test wavelet_matrix::tests::wavelet_matrix_sanity ... ignored

test result: ok. 0 passed; 0 failed; 6 ignored; 0 measured; 0 filtered out

# Statistical Performance

## Uniform distribution

For uniform distribution, `.sum()` produce good results using smaller number of `k`.

```
              actual sum, N = 10000, 0 <= val < 256: 1287225
  .sum_expreriment1(k=5), N = 10000, 0 <= val < 256: (1291728, 10000), error = 0.34982229214006877%,      12204 ns   (R²=0.953, 85553 iterations in 94 samples)
  .sum_expreriment2(k=5), N = 10000, 0 <= val < 256: (1291728, 10000), error = 0.34982229214006877%,      10506 ns   (R²=0.899, 85553 iterations in 94 samples)
  .sum_expreriment3(k=5), N = 10000, 0 <= val < 256: (1012608..1560849, 10000), error = -0.03861018858396939%,       8289 ns   (R²=0.984, 125265 iterations in 98 samples)
```

```
              actual sum, N = 10000, 0 <= val < 257: 1296454
  .sum_expreriment1(k=5), N = 10000, 0 <= val < 257: (1309312, 10000), error = 0.9917821997540984%,       6404 ns   (R²=0.970, 166733 iterations in 101 samples)
  .sum_expreriment2(k=5), N = 10000, 0 <= val < 257: (1309312, 10000), error = 0.9917821997540984%,       6567 ns   (R²=0.965, 151574 iterations in 100 samples)
  .sum_expreriment3(k=5), N = 10000, 0 <= val < 257: (985088..1623537, 10000), error = 0.6061148332297174%,       6872 ns   (R²=0.951, 151574 iterations in 100 samples)
```

```
              actual sum, N = 10000, 256 <= val < 1024: 6390720
  .sum_expreriment1(k=5), N = 10000, 256 <= val < 1024: (6396736, 10000), error = 0.09413649792198688%,      14330 ns   (R²=0.891, 70702 iterations in 92 samples)
  .sum_expreriment2(k=5), N = 10000, 256 <= val < 1024: (6396736, 10000), error = 0.09413649792198688%,      15339 ns   (R²=0.964, 70702 iterations in 92 samples)
  .sum_expreriment3(k=5), N = 10000, 256 <= val < 1024: (5545728..7237745, 10000), error = 0.0158980521756547%,      10318 ns   (R²=0.956, 94110 iterations in 95 samples)
```

```
              actual sum, N = 10000, 255 <= val < 1024: 6441272
  .sum_expreriment1(k=5), N = 10000, 255 <= val < 1024: (6030080, 10000), error = -6.383708062631108%,       8114 ns   (R²=0.973, 125265 iterations in 98 samples)
  .sum_expreriment2(k=5), N = 10000, 255 <= val < 1024: (6434048, 10000), error = -0.11215176132912878%,       7618 ns   (R²=0.956, 151574 iterations in 100 samples)
  .sum_expreriment3(k=5), N = 10000, 255 <= val < 1024: (5373696..7484401, 10000), error = -0.18977618085371958%,       6452 ns   (R²=0.987, 151574 iterations in 100 samples)
```

```
              actual sum, N = 10000, 256 <= val < 1025: 6404188
  .sum_expreriment1(k=5), N = 10000, 256 <= val < 1025: (6004096, 10000), error = -6.247349390742433%,       5470 ns   (R²=0.983, 183408 iterations in 102 samples)
  .sum_expreriment2(k=5), N = 10000, 256 <= val < 1025: (6426368, 10000), error = 0.3463358664673804%,       6639 ns   (R²=0.992, 151574 iterations in 100 samples)
  .sum_expreriment3(k=5), N = 10000, 256 <= val < 1025: (5355008..7487729, 10000), error = 0.2682619560824885%,       6796 ns   (R²=0.992, 151574 iterations in 100 samples)
```

```
              actual sum, N = 10000, 400 <= val < 2000: 11982145
  .sum_expreriment1(k=5), N = 10000, 400 <= val < 2000: (11904128, 10000), error = -0.6511104647790525%,       5561 ns   (R²=0.988, 183408 iterations in 102 samples)
  .sum_expreriment2(k=5), N = 10000, 400 <= val < 2000: (11904128, 10000), error = -0.6511104647790525%,       6000 ns   (R²=0.985, 183408 iterations in 102 samples)
  .sum_expreriment3(k=5), N = 10000, 400 <= val < 2000: (9751808..14046449, 10000), error = -0.6928392203566223%,       6016 ns   (R²=0.982, 183408 iterations in 102 samples)
```

### Some value ranges require greater k to achieve error < 1%

#### Bad (k=5)

```
              actual sum, N = 10000, 300 <= val < 1500: 8981391
  .sum_expreriment1(k=5), N = 10000, 300 <= val < 1500: (8741248, 10000), error = -2.673784049709004%,       8163 ns   (R²=0.922, 137793 iterations in 99 samples)
  .sum_expreriment2(k=5), N = 10000, 300 <= val < 1500: (8741248, 10000), error = -2.673784049709004%,       8138 ns   (R²=0.940, 113876 iterations in 97 samples)
  .sum_expreriment3(k=5), N = 10000, 300 <= val < 1500: (7234048..10238449, 10000), error = -2.729454713640682%,       7179 ns   (R²=0.974, 151574 iterations in 100 samples)
```

#### Good (k=10)

```
              actual sum, N = 10000, 300 <= val < 1500: 9034801
 .sum_expreriment1(k=10), N = 10000, 300 <= val < 1500: (9042688, 10000), error = 0.08729577995132377%,      18261 ns   (R²=0.931, 64273 iterations in 91 samples)
 .sum_expreriment2(k=10), N = 10000, 300 <= val < 1500: (9042688, 10000), error = 0.08729577995132377%,      19587 ns   (R²=0.962, 53116 iterations in 89 samples)
 .sum_expreriment3(k=10), N = 10000, 300 <= val < 1500: (8402688..9672689, 10000), error = 0.031954217918026086%,      18074 ns   (R²=0.708, 43895 iterations in 87 samples)
```


### Finding sufficient k that achieve error < 1% for various ranges

```
              actual sum, N = 1000, 36184 <= val < 49255: 42678133
  .sum_expreriment3(k=1), N = 1000, 36184 <= val < 49255: (0..65535001, 1000), error = -23.221805414965083%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 36184 <= val < 49255: (32817152..49200153, 1000), error = -3.911794829450482%, (bench omitted)
  .sum_expreriment3(k=3), N = 1000, 36184 <= val < 49255: (37978112..46193689, 1000), error = -1.3876731674274505%, (bench omitted)
  .sum_expreriment3(k=4), N = 1000, 36184 <= val < 49255: (39239680..44874777, 1000), error = -1.4548551128044893%, (bench omitted)
  .sum_expreriment3(k=5), N = 1000, 36184 <= val < 49255: (40542208..44674073, 1000), error = -0.16400201948852824%, (bench omitted)
```

```
              actual sum, N = 1000, 35349 <= val < 36820: 36087673
  .sum_expreriment3(k=1), N = 1000, 35349 <= val < 36820: (0..65535001, 1000), error = -9.200296732903782%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 35349 <= val < 36820: (35503104..36526105, 1000), error = -0.2024763414365897%, (bench omitted)
```

```
              actual sum, N = 1000, 29486 <= val < 36451: 32966875
  .sum_expreriment3(k=1), N = 1000, 29486 <= val < 36451: (0..65535001, 1000), error = -0.604773731207462%, (bench omitted)
```

```
              actual sum, N = 1000, 13698 <= val < 52511: 32881395
  .sum_expreriment3(k=1), N = 1000, 13698 <= val < 52511: (0..65535001, 1000), error = -0.3463812894799628%, (bench omitted)
```

```
              actual sum, N = 1000, 58185 <= val < 59533: 58860555
  .sum_expreriment3(k=1), N = 1000, 58185 <= val < 59533: (0..65535001, 1000), error = -44.33029046362203%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 58185 <= val < 59533: (57561088..59608089, 1000), error = -0.4688487901617645%, (bench omitted)
```

```
              actual sum, N = 1000, 53955 <= val < 59647: 56755805
  .sum_expreriment3(k=1), N = 1000, 53955 <= val < 59647: (0..65535001, 1000), error = -42.26581756703125%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 53955 <= val < 59647: (52420608..60611609, 1000), error = -0.4223303677923342%, (bench omitted)
```

```
              actual sum, N = 1000, 13196 <= val < 42182: 27374821
  .sum_expreriment3(k=1), N = 1000, 13196 <= val < 42182: (0..65535001, 1000), error = 19.69941282903731%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 13196 <= val < 42182: (10289152..43056153, 1000), error = -2.565017685412445%, (bench omitted)
  .sum_expreriment3(k=3), N = 1000, 13196 <= val < 42182: (19431424..40959001, 1000), error = 10.302865542024914%, (bench omitted)
  .sum_expreriment3(k=4), N = 1000, 13196 <= val < 42182: (21807104..33618969, 1000), error = 1.2354966631562632%, (bench omitted)
  .sum_expreriment3(k=5), N = 1000, 13196 <= val < 42182: (22159360..31398937, 1000), error = -2.1759886576061995%, (bench omitted)
  .sum_expreriment3(k=6), N = 1000, 13196 <= val < 42182: (22724608..30776345, 1000), error = -2.2807272420155735%, (bench omitted)
  .sum_expreriment3(k=7), N = 1000, 13196 <= val < 42182: (23306240..30247961, 1000), error = -2.183469985064012%, (bench omitted)
  .sum_expreriment3(k=8), N = 1000, 13196 <= val < 42182: (23801856..29645849, 1000), error = -2.377984498967135%, (bench omitted)
  .sum_expreriment3(k=9), N = 1000, 13196 <= val < 42182: (25516032..29475865, 1000), error = 0.4424759526281468%, (bench omitted)
```

```
              actual sum, N = 1000, 27347 <= val < 38748: 33126479
  .sum_expreriment3(k=1), N = 1000, 27347 <= val < 38748: (0..65535001, 1000), error = -1.08366180420201%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 27347 <= val < 38748: (17530880..50297881, 1000), error = 2.378462860480886%, (bench omitted)
  .sum_expreriment3(k=3), N = 1000, 27347 <= val < 38748: (29622272..35621913, 1000), error = -1.5226097527600202%, (bench omitted)
  .sum_expreriment3(k=4), N = 1000, 27347 <= val < 38748: (31084544..35179545, 1000), error = 0.016799249929339004%, (bench omitted)
```

```
              actual sum, N = 1000, 6778 <= val < 63986: 35917150
  .sum_expreriment3(k=1), N = 1000, 6778 <= val < 63986: (0..65535001, 1000), error = -8.769209138252895%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 6778 <= val < 63986: (18841600..51608601, 1000), error = -1.9267954166742072%, (bench omitted)
  .sum_expreriment3(k=3), N = 1000, 6778 <= val < 63986: (23314432..46660633, 1000), error = -2.5882287430934805%, (bench omitted)
  .sum_expreriment3(k=4), N = 1000, 6778 <= val < 63986: (27492352..43875353, 1000), error = -0.649544855312852%, (bench omitted)
```

```
              actual sum, N = 1000, 17111 <= val < 63052: 40771970
  .sum_expreriment3(k=1), N = 1000, 17111 <= val < 63052: (0..65535001, 1000), error = -19.632286592970612%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 17111 <= val < 63052: (21790720..54557721, 1000), error = -6.371411535915483%, (bench omitted)
  .sum_expreriment3(k=3), N = 1000, 17111 <= val < 63052: (27197440..49069081, 1000), error = -6.471872710590143%, (bench omitted)
  .sum_expreriment3(k=4), N = 1000, 17111 <= val < 63052: (34070528..47709209, 1000), error = 0.2891643450144793%, (bench omitted)
```

```
              actual sum, N = 1000, 5231 <= val < 41924: 23324723
  .sum_expreriment3(k=1), N = 1000, 5231 <= val < 41924: (0..65535001, 1000), error = 40.48398345395142%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 5231 <= val < 41924: (8028160..40795161, 1000), error = 4.6600210429079905%, (bench omitted)
  .sum_expreriment3(k=3), N = 1000, 5231 <= val < 41924: (15122432..35519513, 1000), error = 8.558511069992129%, (bench omitted)
  .sum_expreriment3(k=4), N = 1000, 5231 <= val < 41924: (16900096..29735961, 1000), error = -0.028703449125633773%, (bench omitted)
```

```
              actual sum, N = 1000, 1343 <= val < 25963: 13616723
  .sum_expreriment3(k=1), N = 1000, 1343 <= val < 25963: (0..32767001, 1000), error = 20.31896367429961%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 1343 <= val < 25963: (6340608..22723609, 1000), error = 6.7225058481398206%, (bench omitted)
  .sum_expreriment3(k=3), N = 1000, 1343 <= val < 25963: (9142272..20503577, 1000), error = 8.858232630567574%, (bench omitted)
  .sum_expreriment3(k=4), N = 1000, 1343 <= val < 25963: (9560064..17751065, 1000), error = 0.28524484194912386%, (bench omitted)
```

```
              actual sum, N = 1000, 23646 <= val < 42991: 33310918
  .sum_expreriment3(k=1), N = 1000, 23646 <= val < 42991: (0..65535001, 1000), error = -1.6313510183057698%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 23646 <= val < 42991: (17596416..50363417, 1000), error = 2.008344531363561%, (bench omitted)
  .sum_expreriment3(k=3), N = 1000, 23646 <= val < 42991: (25903104..37887001, 1000), error = -4.250456261817822%, (bench omitted)
  .sum_expreriment3(k=4), N = 1000, 23646 <= val < 42991: (29351936..37542937, 1000), error = 0.40982959400878716%, (bench omitted)
```

```
              actual sum, N = 1000, 19034 <= val < 60913: 39641289
  .sum_expreriment3(k=1), N = 1000, 19034 <= val < 60913: (0..65535001, 1000), error = -17.33997348068071%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 19034 <= val < 60913: (21823488..54590489, 1000), error = -3.6181997008220397%, (bench omitted)
  .sum_expreriment3(k=3), N = 1000, 19034 <= val < 60913: (26329088..48184345, 1000), error = -6.015377047905783%, (bench omitted)
  .sum_expreriment3(k=4), N = 1000, 19034 <= val < 60913: (33406976..46586905, 1000), error = 0.897173146917599%, (bench omitted)
```

```
              actual sum, N = 1000, 32948 <= val < 46839: 39923944
  .sum_expreriment3(k=1), N = 1000, 32948 <= val < 46839: (0..65535001, 1000), error = -17.92519296189775%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 32948 <= val < 46839: (36257792..44448793, 1000), error = 1.0754147936887197%, (bench omitted)
  .sum_expreriment3(k=3), N = 1000, 32948 <= val < 46839: (37498880..43338777, 1000), error = 1.2395669125274797%, (bench omitted)
  .sum_expreriment3(k=4), N = 1000, 32948 <= val < 46839: (38051840..42146841, 1000), error = 0.4393253331885246%, (bench omitted)
```

```
              actual sum, N = 1000, 29243 <= val < 41340: 35557177
  .sum_expreriment3(k=1), N = 1000, 29243 <= val < 41340: (0..65535001, 1000), error = -7.845608778222185%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 29243 <= val < 41340: (24215552..56982553, 1000), error = 14.179626802206485%, (bench omitted)
  .sum_expreriment3(k=3), N = 1000, 29243 <= val < 41340: (24436736..39042073, 1000), error = -10.736996921887247%, (bench omitted)
  .sum_expreriment3(k=4), N = 1000, 29243 <= val < 41340: (30248960..37661721, 1000), error = -4.504961122194824%, (bench omitted)
  .sum_expreriment3(k=5), N = 1000, 29243 <= val < 41340: (33845248..37282841, 1000), error = 0.01931255678705877%, (bench omitted)
```

```
              actual sum, N = 1000, 38594 <= val < 57840: 48141632
  .sum_expreriment3(k=1), N = 1000, 38594 <= val < 57840: (0..65535001, 1000), error = -31.935211502593013%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 38594 <= val < 57840: (40026112..56409113, 1000), error = 0.15782597482362043%, (bench omitted)
```

```
              actual sum, N = 1000, 51874 <= val < 61748: 56941847
  .sum_expreriment3(k=1), N = 1000, 51874 <= val < 61748: (0..65535001, 1000), error = -42.45444830758651%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 51874 <= val < 61748: (53116928..61307929, 1000), error = 0.47518830922361893%, (bench omitted)
```

```
              actual sum, N = 1000, 5787 <= val < 43531: 23810029
  .sum_expreriment3(k=1), N = 1000, 5787 <= val < 43531: (0..65535001, 1000), error = 37.620579966534265%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 5787 <= val < 43531: (8585216..41352217, 1000), error = 4.866382145103645%, (bench omitted)
  .sum_expreriment3(k=3), N = 1000, 5787 <= val < 43531: (15876096..36551705, 1000), error = 10.096043982138788%, (bench omitted)
  .sum_expreriment3(k=4), N = 1000, 5787 <= val < 43531: (17457152..30194713, 1000), error = 0.06679118282468283%, (bench omitted)
```

```
              actual sum, N = 1000, 25218 <= val < 30051: 27595624
  .sum_expreriment3(k=1), N = 1000, 25218 <= val < 30051: (0..32767001, 1000), error = -40.63007960972363%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 25218 <= val < 30051: (25718784..29813785, 1000), error = 0.6184313860777346%, (bench omitted)
```

```
              actual sum, N = 1000, 17598 <= val < 64963: 41763687
  .sum_expreriment3(k=1), N = 1000, 17598 <= val < 64963: (0..65535001, 1000), error = -21.54069155819504%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 17598 <= val < 64963: (22446080..55213081, 1000), error = -7.025498012184604%, (bench omitted)
  .sum_expreriment3(k=3), N = 1000, 17598 <= val < 64963: (28393472..49937433, 1000), error = -6.221277829229972%, (bench omitted)
  .sum_expreriment3(k=4), N = 1000, 17598 <= val < 64963: (35061760..48471065, 1000), error = 0.006524807065046723%, (bench omitted)
```

```
              actual sum, N = 1000, 9015 <= val < 33982: 21146009
  .sum_expreriment3(k=1), N = 1000, 9015 <= val < 33982: (0..65535001, 1000), error = 54.95831861227336%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 9015 <= val < 33982: (1507328..34274329, 1000), error = -15.393831526317802%, (bench omitted)
  .sum_expreriment3(k=3), N = 1000, 9015 <= val < 33982: (12042240..29178905, 1000), error = -2.5320948269718415%, (bench omitted)
  .sum_expreriment3(k=4), N = 1000, 9015 <= val < 33982: (14516224..26385433, 1000), error = -3.2875281572045103%, (bench omitted)
  .sum_expreriment3(k=5), N = 1000, 9015 <= val < 33982: (17809408..25734169, 1000), error = 2.959324381257948%, (bench omitted)
  .sum_expreriment3(k=6), N = 1000, 9015 <= val < 33982: (18493440..25144345, 1000), error = 3.1820803632496326%, (bench omitted)
  .sum_expreriment3(k=7), N = 1000, 9015 <= val < 33982: (19152896..24566809, 1000), error = 3.375781217155445%, (bench omitted)
  .sum_expreriment3(k=8), N = 1000, 9015 <= val < 33982: (19333120..23243801, 1000), error = 0.6736543051693584%, (bench omitted)
```

```
              actual sum, N = 1000, 22107 <= val < 41824: 31851073
  .sum_expreriment3(k=1), N = 1000, 22107 <= val < 41824: (0..65535001, 1000), error = 2.8772248897234953%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 22107 <= val < 41824: (14909440..47676441, 1000), error = -1.7523208715762888%, (bench omitted)
  .sum_expreriment3(k=3), N = 1000, 22107 <= val < 41824: (27271168..39189529, 1000), error = 4.3303878647981495%, (bench omitted)
  .sum_expreriment3(k=4), N = 1000, 22107 <= val < 41824: (27705344..35896345, 1000), error = -0.15769955379525205%, (bench omitted)
```

```
              actual sum, N = 1000, 17860 <= val < 59718: 38441265
  .sum_expreriment3(k=1), N = 1000, 17860 <= val < 59718: (0..65535001, 1000), error = -14.75956891637151%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 17860 <= val < 59718: (20676608..53443609, 1000), error = -3.592902054602001%, (bench omitted)
  .sum_expreriment3(k=3), N = 1000, 17860 <= val < 59718: (24772608..47201305, 1000), error = -6.384568770044378%, (bench omitted)
  .sum_expreriment3(k=4), N = 1000, 17860 <= val < 59718: (32219136..45480985, 1000), error = 1.0634275432923448%, (bench omitted)
  .sum_expreriment3(k=5), N = 1000, 17860 <= val < 59718: (33931264..44170265, 1000), error = 1.5855331503788963%, (bench omitted)
  .sum_expreriment3(k=6), N = 1000, 17860 <= val < 59718: (34365440..42556441, 1000), error = 0.05118197853270437%, (bench omitted)
```

```
              actual sum, N = 1000, 37937 <= val < 62388: 50035395
  .sum_expreriment3(k=1), N = 1000, 37937 <= val < 62388: (0..65535001, 1000), error = -34.51135940867459%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 37937 <= val < 62388: (41615360..57998361, 1000), error = -0.45674666903299155%, (bench omitted)
```

```
              actual sum, N = 1000, 50898 <= val < 61155: 56068140
  .sum_expreriment3(k=1), N = 1000, 50898 <= val < 61155: (0..65535001, 1000), error = -41.55771887563953%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 50898 <= val < 61155: (52355072..60546073, 1000), error = 0.6820843352392285%, (bench omitted)
```

```
              actual sum, N = 1000, 51097 <= val < 64333: 57704257
  .sum_expreriment3(k=1), N = 1000, 51097 <= val < 64333: (0..65535001, 1000), error = -43.214761434325375%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 51097 <= val < 64333: (53485568..61676569, 1000), error = -0.21348338303706088%, (bench omitted)
```

```
              actual sum, N = 1000, 15926 <= val < 50327: 33261277
  .sum_expreriment3(k=1), N = 1000, 15926 <= val < 50327: (0..65535001, 1000), error = -1.4845401155223235%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 15926 <= val < 50327: (17072128..49839129, 1000), error = 0.5843161102924581%, (bench omitted)
```

```
              actual sum, N = 1000, 34892 <= val < 57703: 46497670
  .sum_expreriment3(k=1), N = 1000, 34892 <= val < 57703: (0..65535001, 1000), error = -29.52872692330605%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 34892 <= val < 57703: (39141376..55524377, 1000), error = 1.7962319402241016%, (bench omitted)
  .sum_expreriment3(k=3), N = 1000, 34892 <= val < 57703: (42057728..53435417, 1000), error = 2.685945338766437%, (bench omitted)
  .sum_expreriment3(k=4), N = 1000, 34892 <= val < 57703: (42221568..50412569, 1000), error = -0.3884108601570788%, (bench omitted)
```

```
              actual sum, N = 1000, 36733 <= val < 49541: 43442015
  .sum_expreriment3(k=1), N = 1000, 36733 <= val < 49541: (0..65535001, 1000), error = -24.57186896141903%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 36733 <= val < 49541: (33423360..49806361, 1000), error = -4.205962821936321%, (bench omitted)
  .sum_expreriment3(k=3), N = 1000, 36733 <= val < 49541: (38821888..47340569, 1000), error = -0.8305024525220573%, (bench omitted)
```

```
              actual sum, N = 1000, 24009 <= val < 57648: 40714786
  .sum_expreriment3(k=1), N = 1000, 24009 <= val < 57648: (0..65535001, 1000), error = -19.519409975530756%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 24009 <= val < 57648: (23920640..56687641, 1000), error = -1.0085918172331791%, (bench omitted)
  .sum_expreriment3(k=3), N = 1000, 24009 <= val < 57648: (28164096..48970777, 1000), error = -5.274128175449578%, (bench omitted)
  .sum_expreriment3(k=4), N = 1000, 24009 <= val < 57648: (34521088..47045657, 1000), error = 0.16845477218030816%, (bench omitted)
```

```
              actual sum, N = 1000, 22558 <= val < 62106: 42079354
  .sum_expreriment3(k=1), N = 1000, 22558 <= val < 62106: (0..65535001, 1000), error = -22.129270330528364%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 22558 <= val < 62106: (24182784..56949785, 1000), error = -3.5957538701758587%, (bench omitted)
  .sum_expreriment3(k=3), N = 1000, 22558 <= val < 62106: (29392896..50068505, 1000), error = -5.581487776642199%, (bench omitted)
  .sum_expreriment3(k=4), N = 1000, 22558 <= val < 62106: (35430400..48372761, 1000), error = -0.4224732157247471%, (bench omitted)
```

```
              actual sum, N = 1000, 37004 <= val < 49417: 43322967
  .sum_expreriment3(k=1), N = 1000, 37004 <= val < 49417: (0..65535001, 1000), error = -24.364598574238926%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 37004 <= val < 49417: (33259520..49642521, 1000), error = -4.3209113540169115%, (bench omitted)
  .sum_expreriment3(k=3), N = 1000, 37004 <= val < 49417: (38674432..47111193, 1000), error = -0.992902909904578%, (bench omitted)
```

```
              actual sum, N = 1000, 14886 <= val < 32238: 23847026
  .sum_expreriment3(k=1), N = 1000, 14886 <= val < 32238: (0..32767001, 1000), error = -31.297512738066374%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 14886 <= val < 32238: (15040512..31423513, 1000), error = -2.578996643019553%, (bench omitted)
  .sum_expreriment3(k=3), N = 1000, 14886 <= val < 32238: (18907136..27769881, 1000), error = -2.1324168472831793%, (bench omitted)
  .sum_expreriment3(k=4), N = 1000, 14886 <= val < 32238: (19836928..26766361, 1000), error = -2.287002161191924%, (bench omitted)
  .sum_expreriment3(k=5), N = 1000, 14886 <= val < 32238: (20774912..25877529, 1000), error = -2.1839452852527605%, (bench omitted)
  .sum_expreriment3(k=6), N = 1000, 14886 <= val < 32238: (21714944..25644057, 1000), error = -0.7025026936272892%, (bench omitted)
```

```
              actual sum, N = 1000, 12257 <= val < 64234: 38114810
  .sum_expreriment3(k=1), N = 1000, 12257 <= val < 64234: (0..65535001, 1000), error = -14.029480928804317%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 12257 <= val < 64234: (19628032..52395033, 1000), error = -5.518269669978677%, (bench omitted)
  .sum_expreriment3(k=3), N = 1000, 12257 <= val < 64234: (24379392..47332377, 1000), error = -5.926635866740513%, (bench omitted)
  .sum_expreriment3(k=4), N = 1000, 12257 <= val < 64234: (29458432..45841433, 1000), error = -1.2196781251172444%, (bench omitted)
  .sum_expreriment3(k=5), N = 1000, 12257 <= val < 64234: (30695424..44538905, 1000), error = -1.3056499560144732%, (bench omitted)
  .sum_expreriment3(k=6), N = 1000, 12257 <= val < 64234: (31858688..43170841, 1000), error = -1.5743119275683126%, (bench omitted)
  .sum_expreriment3(k=7), N = 1000, 12257 <= val < 64234: (33030144..41966617, 1000), error = -1.617297843016927%, (bench omitted)
  .sum_expreriment3(k=8), N = 1000, 12257 <= val < 64234: (34140160..41647129, 1000), error = -0.5802626328191063%, (bench omitted)
```

```
              actual sum, N = 1000, 2522 <= val < 65511: 32866557
  .sum_expreriment3(k=1), N = 1000, 2522 <= val < 65511: (0..65535001, 1000), error = -0.30139147218858364%, (bench omitted)
```

```
              actual sum, N = 1000, 4517 <= val < 6958: 5741104
  .sum_expreriment3(k=1), N = 1000, 4517 <= val < 6958: (0..8191001, 1000), error = -28.66354624476407%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 4517 <= val < 6958: (4794368..6841369, 1000), error = 1.3370947469336907%, (bench omitted)
  .sum_expreriment3(k=3), N = 1000, 4517 <= val < 6958: (5211136..6583321, 1000), error = 2.719407277763998%, (bench omitted)
  .sum_expreriment3(k=4), N = 1000, 4517 <= val < 6958: (5318656..6133273, 1000), error = -0.2637123452214069%, (bench omitted)
```

```
              actual sum, N = 1000, 17249 <= val < 46392: 32381318
  .sum_expreriment3(k=1), N = 1000, 17249 <= val < 46392: (0..65535001, 1000), error = 1.1926074164121423%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 17249 <= val < 46392: (16023552..48790553, 1000), error = 0.07947174972927291%, (bench omitted)
```

```
              actual sum, N = 1000, 1519 <= val < 61294: 30448802
  .sum_expreriment3(k=1), N = 1000, 1519 <= val < 61294: (0..65535001, 1000), error = 7.615071358144074%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 1519 <= val < 61294: (15269888..48036889, 1000), error = 3.9561031005423466%, (bench omitted)
  .sum_expreriment3(k=3), N = 1000, 1519 <= val < 61294: (19496960..43514905, 1000), error = 3.471827889977412%, (bench omitted)
  .sum_expreriment3(k=4), N = 1000, 1519 <= val < 61294: (22593536..38976537, 1000), error = 1.1042601938821763%, (bench omitted)
  .sum_expreriment3(k=5), N = 1000, 1519 <= val < 61294: (23691264..37805081, 1000), error = 0.9831913912409427%, (bench omitted)
```

```
              actual sum, N = 1000, 28683 <= val < 43291: 36096940
  .sum_expreriment3(k=1), N = 1000, 28683 <= val < 43291: (0..65535001, 1000), error = -9.223607319623214%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 28683 <= val < 43291: (23887872..56654873, 1000), error = 11.564503805585737%, (bench omitted)
  .sum_expreriment3(k=3), N = 1000, 28683 <= val < 43291: (25190400..40041497, 1000), error = -9.643454542130165%, (bench omitted)
  .sum_expreriment3(k=4), N = 1000, 28683 <= val < 43291: (30842880..38919193, 1000), error = -3.368440648985759%, (bench omitted)
  .sum_expreriment3(k=5), N = 1000, 28683 <= val < 43291: (34488320..37977113, 1000), error = 0.3761426868870325%, (bench omitted)
```

```
              actual sum, N = 1000, 11143 <= val < 47503: 29505066
  .sum_expreriment3(k=1), N = 1000, 11143 <= val < 47503: (0..65535001, 1000), error = 11.057199465339275%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 11143 <= val < 47503: (13434880..46201881, 1000), error = 1.061898997277281%, (bench omitted)
  .sum_expreriment3(k=3), N = 1000, 11143 <= val < 47503: (20873216..43973657, 1000), error = 9.891081077398708%, (bench omitted)
  .sum_expreriment3(k=4), N = 1000, 11143 <= val < 47503: (22642688..35306521, 1000), error = -1.7978675255293446%, (bench omitted)
  .sum_expreriment3(k=5), N = 1000, 11143 <= val < 47503: (24297472..33602585, 1000), error = -1.8811616960965278%, (bench omitted)
  .sum_expreriment3(k=6), N = 1000, 11143 <= val < 47503: (25870336..33086489, 1000), error = -0.09033702890208753%, (bench omitted)
```

```
              actual sum, N = 1000, 20797 <= val < 28973: 25007783
  .sum_expreriment3(k=1), N = 1000, 20797 <= val < 28973: (0..32767001, 1000), error = -34.486395695292146%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 20797 <= val < 28973: (20930560..29121561, 1000), error = 0.07308524710087255%, (bench omitted)
```

```
              actual sum, N = 1000, 7404 <= val < 43445: 24833949
  .sum_expreriment3(k=1), N = 1000, 7404 <= val < 43445: (0..65535001, 1000), error = 31.946393221633823%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 7404 <= val < 43445: (8749056..41516057, 1000), error = 1.2024144851066578%, (bench omitted)
  .sum_expreriment3(k=3), N = 1000, 7404 <= val < 43445: (16400384..37157913, 1000), error = 7.832821916482151%, (bench omitted)
  .sum_expreriment3(k=4), N = 1000, 7404 <= val < 43445: (18399232..30956569, 1000), error = -0.6283696563925455%, (bench omitted)
```

```
              actual sum, N = 1000, 15229 <= val < 56532: 35929591
  .sum_expreriment3(k=1), N = 1000, 15229 <= val < 56532: (0..65535001, 1000), error = -8.800798762223593%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 15229 <= val < 56532: (19070976..51837977, 1000), error = -1.32235014865602%, (bench omitted)
  .sum_expreriment3(k=3), N = 1000, 15229 <= val < 56532: (22102016..45333529, 1000), error = -6.155981569620428%, (bench omitted)
  .sum_expreriment3(k=4), N = 1000, 15229 <= val < 56532: (28442624..44825625, 1000), error = 1.9608711938858419%, (bench omitted)
  .sum_expreriment3(k=5), N = 1000, 15229 <= val < 56532: (30015488..43146265, 1000), error = 1.8126702305072164%, (bench omitted)
  .sum_expreriment3(k=6), N = 1000, 15229 <= val < 56532: (31490048..41450521, 1000), error = 1.5048682296439166%, (bench omitted)
  .sum_expreriment3(k=7), N = 1000, 15229 <= val < 56532: (31924224..39521305, 1000), error = -0.5756452947098675%, (bench omitted)
```

```
              actual sum, N = 1000, 27152 <= val < 38551: 32796811
  .sum_expreriment3(k=1), N = 1000, 27152 <= val < 38551: (0..65535001, 1000), error = -0.08937149407605513%, (bench omitted)
```

```
              actual sum, N = 1000, 9921 <= val < 34261: 22367154
  .sum_expreriment3(k=1), N = 1000, 9921 <= val < 34261: (0..65535001, 1000), error = 46.49829835302247%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 9921 <= val < 34261: (1900544..34667545, 1000), error = -18.2549375749816%, (bench omitted)
  .sum_expreriment3(k=3), N = 1000, 9921 <= val < 34261: (13107200..30440473, 1000), error = -2.652630728075642%, (bench omitted)
  .sum_expreriment3(k=4), N = 1000, 9921 <= val < 34261: (16007168..27737113, 1000), error = -2.213129126754347%, (bench omitted)
  .sum_expreriment3(k=5), N = 1000, 9921 <= val < 34261: (18841600..27008025, 1000), error = 2.493200520727849%, (bench omitted)
  .sum_expreriment3(k=6), N = 1000, 9921 <= val < 34261: (19574784..26389529, 1000), error = 2.7495764548319377%, (bench omitted)
  .sum_expreriment3(k=7), N = 1000, 9921 <= val < 34261: (20275200..26033177, 1000), error = 3.5187042571442033%, (bench omitted)
  .sum_expreriment3(k=8), N = 1000, 9921 <= val < 34261: (20447232..24413209, 1000), error = 0.2819580890800859%, (bench omitted)
```

```
              actual sum, N = 1000, 31633 <= val < 50522: 40953684
  .sum_expreriment3(k=1), N = 1000, 31633 <= val < 50522: (0..65535001, 1000), error = -19.98888305140021%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 31633 <= val < 50522: (30736384..63503385, 1000), error = 15.056520922513345%, (bench omitted)
  .sum_expreriment3(k=3), N = 1000, 31633 <= val < 50522: (31850496..49249305, 1000), error = -0.9859528144037054%, (bench omitted)
```

```
              actual sum, N = 1000, 6809 <= val < 7555: 7182138
  .sum_expreriment3(k=1), N = 1000, 6809 <= val < 7555: (0..8191001, 1000), error = -42.97658997919561%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 6809 <= val < 7555: (6669312..7692313, 1000), error = -0.018462468975115765%, (bench omitted)
```

```
              actual sum, N = 1000, 25427 <= val < 54059: 39562735
  .sum_expreriment3(k=1), N = 1000, 25427 <= val < 54059: (0..65535001, 1000), error = -17.175847423086395%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 25427 <= val < 54059: (23986176..56753177, 1000), error = 2.0396491799669563%, (bench omitted)
  .sum_expreriment3(k=3), N = 1000, 25427 <= val < 54059: (26771456..47545369, 1000), error = -6.077241626495236%, (bench omitted)
  .sum_expreriment3(k=4), N = 1000, 25427 <= val < 54059: (28999680..45169689, 1000), error = -6.263598813378296%, (bench omitted)
  .sum_expreriment3(k=5), N = 1000, 25427 <= val < 54059: (36192256..43195417, 1000), error = 0.3313749668722347%, (bench omitted)
```

```
              actual sum, N = 1000, 13569 <= val < 58718: 35860869
  .sum_expreriment3(k=1), N = 1000, 13569 <= val < 58718: (0..65535001, 1000), error = -8.626029112679896%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 13569 <= val < 58718: (18055168..50822169, 1000), error = -3.9658854892780204%, (bench omitted)
  .sum_expreriment3(k=3), N = 1000, 13569 <= val < 58718: (21528576..45267993, 1000), error = -6.867053333258601%, (bench omitted)
  .sum_expreriment3(k=4), N = 1000, 13569 <= val < 58718: (27836416..44219417, 1000), error = 0.4658197212119985%, (bench omitted)
```

```
              actual sum, N = 1000, 42302 <= val < 63626: 52804743
  .sum_expreriment3(k=1), N = 1000, 42302 <= val < 63626: (0..65535001, 1000), error = -37.94591519932215%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 42302 <= val < 63626: (43794432..60177433, 1000), error = -1.5506391158839652%, (bench omitted)
  .sum_expreriment3(k=3), N = 1000, 42302 <= val < 63626: (46080000..56949785, 1000), error = -2.4426801963603912%, (bench omitted)
  .sum_expreriment3(k=4), N = 1000, 42302 <= val < 63626: (49582080..56159257, 1000), error = 0.12484673961958304%, (bench omitted)
```

```
              actual sum, N = 1000, 23781 <= val < 32491: 28120986
  .sum_expreriment3(k=1), N = 1000, 23781 <= val < 32491: (0..32767001, 1000), error = -41.739240579971124%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 23781 <= val < 32491: (23764992..31955993, 1000), error = -0.9263330951482284%, (bench omitted)
```

```
              actual sum, N = 1000, 831 <= val < 15847: 8238488
  .sum_expreriment3(k=1), N = 1000, 831 <= val < 15847: (0..16383001, 1000), error = -0.5703473744211316%, (bench omitted)
```

```
              actual sum, N = 1000, 42369 <= val < 50617: 46416880
  .sum_expreriment3(k=1), N = 1000, 42369 <= val < 50617: (0..65535001, 1000), error = -29.40606951608984%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 42369 <= val < 50617: (35536896..51919897, 1000), error = -5.792039447718158%, (bench omitted)
  .sum_expreriment3(k=3), N = 1000, 42369 <= val < 50617: (44339200..50510873, 1000), error = 2.171959855983427%, (bench omitted)
  .sum_expreriment3(k=4), N = 1000, 42369 <= val < 50617: (44840960..48630809, 1000), error = 0.6872586007504167%, (bench omitted)
```

```
              actual sum, N = 1000, 8621 <= val < 40399: 24499203
  .sum_expreriment3(k=1), N = 1000, 8621 <= val < 40399: (0..65535001, 1000), error = 33.749248904137815%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 8621 <= val < 40399: (8060928..40827929, 1000), error = -0.22357870172348054%, (bench omitted)
```

```
              actual sum, N = 1000, 43058 <= val < 51115: 47194766
  .sum_expreriment3(k=1), N = 1000, 43058 <= val < 51115: (0..65535001, 1000), error = -30.569631386666902%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 43058 <= val < 51115: (37060608..53443609, 1000), error = -4.1162572985317905%, (bench omitted)
  .sum_expreriment3(k=3), N = 1000, 43058 <= val < 51115: (45137920..52452377, 1000), error = 3.3910158596824065%, (bench omitted)
  .sum_expreriment3(k=4), N = 1000, 43058 <= val < 51115: (45676544..48755737, 1000), error = 0.045288920385790236%, (bench omitted)
```

```
              actual sum, N = 1000, 13291 <= val < 40373: 26550654
  .sum_expreriment3(k=1), N = 1000, 13291 <= val < 40373: (0..65535001, 1000), error = 23.41503904197614%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 13291 <= val < 40373: (9043968..41810969, 1000), error = -4.230351538609934%, (bench omitted)
  .sum_expreriment3(k=3), N = 1000, 13291 <= val < 40373: (18939904..39844889, 1000), error = 10.703096051795937%, (bench omitted)
  .sum_expreriment3(k=4), N = 1000, 13291 <= val < 40373: (21209088..37166105, 1000), error = 9.931740287828692%, (bench omitted)
  .sum_expreriment3(k=5), N = 1000, 13291 <= val < 40373: (21921792..29756441, 1000), error = -2.6799264530357707%, (bench omitted)
  .sum_expreriment3(k=6), N = 1000, 13291 <= val < 40373: (22507520..29207577, 1000), error = -2.610504434278719%, (bench omitted)
  .sum_expreriment3(k=7), N = 1000, 13291 <= val < 40373: (23015424..28584985, 1000), error = -2.8264840481895472%, (bench omitted)
  .sum_expreriment3(k=8), N = 1000, 13291 <= val < 40373: (24664064..28402713, 1000), error = -0.06503041318680888%, (bench omitted)
```

```
              actual sum, N = 1000, 19616 <= val < 42846: 31246293
  .sum_expreriment3(k=1), N = 1000, 19616 <= val < 42846: (0..65535001, 1000), error = 4.868439913816337%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 19616 <= val < 42846: (14123008..46890009, 1000), error = -2.3675928533346338%, (bench omitted)
  .sum_expreriment3(k=3), N = 1000, 19616 <= val < 42846: (26394624..38116377, 1000), error = 3.2298455371970043%, (bench omitted)
  .sum_expreriment3(k=4), N = 1000, 19616 <= val < 42846: (27131904..35322905, 1000), error = -0.06045197105461438%, (bench omitted)
```

```
              actual sum, N = 1000, 23167 <= val < 60242: 41010611
  .sum_expreriment3(k=1), N = 1000, 23167 <= val < 60242: (0..65535001, 1000), error = -20.09994681620325%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 23167 <= val < 60242: (23560192..56327193, 1000), error = -2.601568164882986%, (bench omitted)
  .sum_expreriment3(k=3), N = 1000, 23167 <= val < 60242: (28164096..49151001, 1000), error = -5.737693105815956%, (bench omitted)
  .sum_expreriment3(k=4), N = 1000, 23167 <= val < 60242: (34398208..47193113, 1000), error = -0.5241350829910825%, (bench omitted)
```

```
              actual sum, N = 1000, 6631 <= val < 11743: 9211161
  .sum_expreriment3(k=1), N = 1000, 6631 <= val < 11743: (0..16383001, 1000), error = -11.06984233583584%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 6631 <= val < 11743: (5849088..14040089, 1000), error = 7.962373038534447%, (bench omitted)
  .sum_expreriment3(k=3), N = 1000, 6631 <= val < 11743: (6457344..10261529, 1000), error = -9.246662825674202%, (bench omitted)
  .sum_expreriment3(k=4), N = 1000, 6631 <= val < 11743: (8425472..10045465, 1000), error = 0.26388638739459663%, (bench omitted)
```

```
              actual sum, N = 1000, 5229 <= val < 61757: 33127241
  .sum_expreriment3(k=1), N = 1000, 5229 <= val < 61757: (0..65535001, 1000), error = -1.08593709931956%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 5229 <= val < 61757: (15794176..48561177, 1000), error = -2.866417399505138%, (bench omitted)
  .sum_expreriment3(k=3), N = 1000, 5229 <= val < 61757: (20873216..45153305, 1000), error = -0.344070307575569%, (bench omitted)
```

```
              actual sum, N = 1000, 22672 <= val < 38715: 30634757
  .sum_expreriment3(k=1), N = 1000, 22672 <= val < 38715: (0..65535001, 1000), error = 6.96184076145928%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 22672 <= val < 38715: (11993088..44760089, 1000), error = -7.371264606407682%, (bench omitted)
  .sum_expreriment3(k=3), N = 1000, 22672 <= val < 38715: (26714112..37903385, 1000), error = 5.4643521409358655%, (bench omitted)
  .sum_expreriment3(k=4), N = 1000, 22672 <= val < 38715: (27746304..33770521, 1000), error = 0.40364282961343545%, (bench omitted)
```

```
              actual sum, N = 1000, 5982 <= val < 47323: 26776856
  .sum_expreriment3(k=1), N = 1000, 5982 <= val < 47323: (0..65535001, 1000), error = 22.372469717878754%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 5982 <= val < 47323: (11534336..44301337, 1000), error = 4.261067841571841%, (bench omitted)
  .sum_expreriment3(k=3), N = 1000, 5982 <= val < 47323: (17874944..40025113, 1000), error = 8.115859457137162%, (bench omitted)
  .sum_expreriment3(k=4), N = 1000, 5982 <= val < 47323: (19668992..32881689, 1000), error = -1.8729458006571047%, (bench omitted)
  .sum_expreriment3(k=5), N = 1000, 5982 <= val < 47323: (21053440..31382553, 1000), error = -2.0871008904107335%, (bench omitted)
  .sum_expreriment3(k=6), N = 1000, 5982 <= val < 47323: (22683648..30874649, 1000), error = 0.00855963074977884%, (bench omitted)
```

```
              actual sum, N = 1000, 41247 <= val < 53874: 47449273
  .sum_expreriment3(k=1), N = 1000, 41247 <= val < 53874: (0..65535001, 1000), error = -30.942039933888978%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 41247 <= val < 53874: (38584320..54967321, 1000), error = -1.4193115245411663%, (bench omitted)
  .sum_expreriment3(k=3), N = 1000, 41247 <= val < 53874: (45215744..50764825, 1000), error = 1.1401881752750986%, (bench omitted)
  .sum_expreriment3(k=4), N = 1000, 41247 <= val < 53874: (45408256..49503257, 1000), error = 0.013663012286826818%, (bench omitted)
```

```
              actual sum, N = 1000, 53673 <= val < 63954: 58763816
  .sum_expreriment3(k=1), N = 1000, 53673 <= val < 63954: (0..65535001, 1000), error = -44.23864508731019%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 53673 <= val < 63954: (54296576..62487577, 1000), error = -0.6326001701455195%, (bench omitted)
```

```
              actual sum, N = 1000, 692 <= val < 23663: 11948880
  .sum_expreriment3(k=1), N = 1000, 692 <= val < 23663: (0..32767001, 1000), error = 37.11326919343068%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 692 <= val < 23663: (4898816..21281817, 1000), error = 9.552661002537477%, (bench omitted)
  .sum_expreriment3(k=3), N = 1000, 692 <= val < 23663: (7823360..18463769, 1000), error = 9.998292727017093%, (bench omitted)
  .sum_expreriment3(k=4), N = 1000, 692 <= val < 23663: (8536064..15264793, 1000), error = -0.405494071410877%, (bench omitted)
```

```
              actual sum, N = 1000, 8745 <= val < 51832: 31413752
  .sum_expreriment3(k=1), N = 1000, 8745 <= val < 51832: (0..65535001, 1000), error = 4.3094120052899125%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 8745 <= val < 51832: (15269888..48036889, 1000), error = 0.7628378806835936%, (bench omitted)
```

```
              actual sum, N = 1000, 21440 <= val < 36133: 28708700
  .sum_expreriment3(k=1), N = 1000, 21440 <= val < 36133: (0..65535001, 1000), error = 14.137874581572834%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 21440 <= val < 36133: (7274496..40041497, 1000), error = -17.592938725891454%, (bench omitted)
  .sum_expreriment3(k=3), N = 1000, 21440 <= val < 36133: (24535040..38181913, 1000), error = 9.22987108437512%, (bench omitted)
  .sum_expreriment3(k=4), N = 1000, 21440 <= val < 36133: (25731072..33483801, 1000), error = 3.1305353429448215%, (bench omitted)
  .sum_expreriment3(k=5), N = 1000, 21440 <= val < 36133: (26957824..30454809, 1000), error = -0.008304102937437084%, (bench omitted)
```

```
              actual sum, N = 1000, 3839 <= val < 11552: 7744115
  .sum_expreriment3(k=1), N = 1000, 3839 <= val < 11552: (0..16383001, 1000), error = 5.777096543633456%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 3839 <= val < 11552: (3547136..11738137, 1000), error = -1.3104015113411926%, (bench omitted)
  .sum_expreriment3(k=3), N = 1000, 3839 <= val < 11552: (5758976..11627545, 1000), error = 12.256339168516996%, (bench omitted)
  .sum_expreriment3(k=4), N = 1000, 3839 <= val < 11552: (6369280..9358361, 1000), error = 1.5457544212605314%, (bench omitted)
  .sum_expreriment3(k=5), N = 1000, 3839 <= val < 11552: (6703104..8805401, 1000), error = 0.13089939909208476%, (bench omitted)
```

```
              actual sum, N = 1000, 553 <= val < 52463: 26529940
  .sum_expreriment3(k=1), N = 1000, 553 <= val < 52463: (0..65535001, 1000), error = 23.511398819597783%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 553 <= val < 52463: (12353536..45120537, 1000), error = 8.319264951221149%, (bench omitted)
  .sum_expreriment3(k=3), N = 1000, 553 <= val < 52463: (17498112..40057881, 1000), error = 8.47365655557457%, (bench omitted)
  .sum_expreriment3(k=4), N = 1000, 553 <= val < 52463: (18612224..34995225, 1000), error = 1.0319812257396737%, (bench omitted)
  .sum_expreriment3(k=5), N = 1000, 553 <= val < 52463: (19759104..33569817, 1000), error = 0.507049770938042%, (bench omitted)
```

```
              actual sum, N = 1000, 13813 <= val < 37448: 25519233
  .sum_expreriment3(k=1), N = 1000, 13813 <= val < 37448: (0..65535001, 1000), error = 28.40315380952084%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 13813 <= val < 37448: (5963776..38730777, 1000), error = -12.429672161385101%, (bench omitted)
  .sum_expreriment3(k=3), N = 1000, 13813 <= val < 37448: (17596416..36961305, 1000), error = 6.895297362581392%, (bench omitted)
  .sum_expreriment3(k=4), N = 1000, 13813 <= val < 37448: (20480000..34028569, 1000), error = 6.798993527744349%, (bench omitted)
  .sum_expreriment3(k=5), N = 1000, 13813 <= val < 37448: (21274624..28883993, 1000), error = -1.7238958553338968%, (bench omitted)
  .sum_expreriment3(k=6), N = 1000, 13813 <= val < 37448: (21987328..28154905, 1000), error = -1.7559971336129108%, (bench omitted)
  .sum_expreriment3(k=7), N = 1000, 13813 <= val < 37448: (22978560..27515929, 1000), error = -1.0658196506141073%, (bench omitted)
  .sum_expreriment3(k=8), N = 1000, 13813 <= val < 37448: (23607296..27304985, 1000), error = -0.24723705449924768%, (bench omitted)
```

```
              actual sum, N = 1000, 5454 <= val < 41541: 23282977
  .sum_expreriment3(k=1), N = 1000, 5454 <= val < 41541: (0..65535001, 1000), error = 40.73586895696371%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 5454 <= val < 41541: (7766016..40533017, 1000), error = 3.7217706309635576%, (bench omitted)
  .sum_expreriment3(k=3), N = 1000, 5454 <= val < 41541: (15138816..35404825, 1000), error = 8.542047694330497%, (bench omitted)
  .sum_expreriment3(k=4), N = 1000, 5454 <= val < 41541: (17022976..29719577, 1000), error = 0.3792427403076505%, (bench omitted)
```

```
              actual sum, N = 1000, 5005 <= val < 49841: 27860967
  .sum_expreriment3(k=1), N = 1000, 5005 <= val < 49841: (0..65535001, 1000), error = 17.610777831221725%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 5005 <= val < 49841: (13500416..46267417, 1000), error = 7.260871455036002%, (bench omitted)
  .sum_expreriment3(k=3), N = 1000, 5005 <= val < 49841: (18972672..42105881, 1000), error = 9.613122904169119%, (bench omitted)
  .sum_expreriment3(k=4), N = 1000, 5005 <= val < 49841: (19202048..35585049, 1000), error = -1.6776840516698504%, (bench omitted)
  .sum_expreriment3(k=5), N = 1000, 5005 <= val < 49841: (20774912..33897497, 1000), error = -1.8835060534689985%, (bench omitted)
  .sum_expreriment3(k=6), N = 1000, 5005 <= val < 49841: (22093824..32480281, 1000), error = -2.059924912153982%, (bench omitted)
  .sum_expreriment3(k=7), N = 1000, 5005 <= val < 49841: (23609344..31915033, 1000), error = -0.3545426115324712%, (bench omitted)
```

```
              actual sum, N = 1000, 12456 <= val < 14983: 13739168
  .sum_expreriment3(k=1), N = 1000, 12456 <= val < 14983: (0..16383001, 1000), error = -40.37848580059579%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 12456 <= val < 14983: (12847104..14894105, 1000), error = 0.9566518147241521%, (bench omitted)
```

```
              actual sum, N = 1000, 10294 <= val < 19998: 15084536
  .sum_expreriment3(k=1), N = 1000, 10294 <= val < 19998: (0..32767001, 1000), error = 8.611229407387805%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 10294 <= val < 19998: (5931008..22314009, 1000), error = -6.377577672922786%, (bench omitted)
  .sum_expreriment3(k=3), N = 1000, 10294 <= val < 19998: (12877824..18455577, 1000), error = 3.859343104753106%, (bench omitted)
  .sum_expreriment3(k=4), N = 1000, 10294 <= val < 19998: (13312000..16546841, 1000), error = -1.0283113779568693%, (bench omitted)
  .sum_expreriment3(k=5), N = 1000, 10294 <= val < 19998: (13639680..16133145, 1000), error = -1.3134245561149511%, (bench omitted)
  .sum_expreriment3(k=6), N = 1000, 10294 <= val < 19998: (14199808..16023577, 1000), error = 0.1800254247130969%, (bench omitted)
```

```
              actual sum, N = 1000, 16239 <= val < 51354: 33670461
  .sum_expreriment3(k=1), N = 1000, 16239 <= val < 51354: (0..65535001, 1000), error = -2.681760133904908%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 16239 <= val < 51354: (17432576..50199577, 1000), error = 0.4324710612070325%, (bench omitted)
```

```
              actual sum, N = 1000, 51381 <= val < 54243: 52855557
  .sum_expreriment3(k=1), N = 1000, 51381 <= val < 54243: (0..65535001, 1000), error = -38.00557243205289%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 51381 <= val < 54243: (50696192..54791193, 1000), error = -0.21164283634358444%, (bench omitted)
```

```
              actual sum, N = 1000, 8461 <= val < 20697: 14567161
  .sum_expreriment3(k=1), N = 1000, 8461 <= val < 20697: (0..32767001, 1000), error = 12.468723315407855%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 8461 <= val < 20697: (5799936..22182937, 1000), error = -3.952211415800237%, (bench omitted)
  .sum_expreriment3(k=3), N = 1000, 8461 <= val < 20697: (12431360..17976345, 1000), error = 4.370728105497014%, (bench omitted)
  .sum_expreriment3(k=4), N = 1000, 8461 <= val < 20697: (12488704..16583705, 1000), error = -0.2125122390011341%, (bench omitted)
```

```
              actual sum, N = 1000, 45742 <= val < 53983: 49854492
  .sum_expreriment3(k=1), N = 1000, 45742 <= val < 53983: (0..65535001, 1000), error = -34.273726026533375%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 45742 <= val < 53983: (42074112..58457113, 1000), error = 0.8246398338589028%, (bench omitted)
```

```
              actual sum, N = 1000, 34043 <= val < 52856: 43399163
  .sum_expreriment3(k=1), N = 1000, 34043 <= val < 52856: (0..65535001, 1000), error = -24.497391804537795%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 34043 <= val < 52856: (35766272..52149273, 1000), error = 1.2871423349800548%, (bench omitted)
  .sum_expreriment3(k=3), N = 1000, 34043 <= val < 52856: (39477248..49167385, 1000), error = 2.1271216682220344%, (bench omitted)
  .sum_expreriment3(k=4), N = 1000, 34043 <= val < 52856: (40382464..48217113, 1000), error = 2.0752128330216877%, (bench omitted)
  .sum_expreriment3(k=5), N = 1000, 34043 <= val < 52856: (41324544..46169113, 1000), error = 0.8010868781040779%, (bench omitted)
```

```
              actual sum, N = 1000, 54990 <= val < 62954: 58908121
  .sum_expreriment3(k=1), N = 1000, 54990 <= val < 62954: (0..65535001, 1000), error = -44.37524157322893%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 54990 <= val < 62954: (54927360..63118361, 1000), error = 0.1947762007211196%, (bench omitted)
```

```
              actual sum, N = 1000, 34469 <= val < 38482: 36437454
  .sum_expreriment3(k=1), N = 1000, 34469 <= val < 38482: (0..65535001, 1000), error = -10.071927637973827%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 34469 <= val < 38482: (34336768..38431769, 1000), error = -0.14596519284799644%, (bench omitted)
```

```
              actual sum, N = 1000, 6770 <= val < 14375: 10676202
  .sum_expreriment3(k=1), N = 1000, 6770 <= val < 14375: (0..16383001, 1000), error = -23.273276395482213%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 6770 <= val < 14375: (6832128..15023129, 1000), error = 2.3550135151058402%, (bench omitted)
  .sum_expreriment3(k=3), N = 1000, 6770 <= val < 14375: (7987200..12762137, 1000), error = -2.8243564518543205%, (bench omitted)
  .sum_expreriment3(k=4), N = 1000, 6770 <= val < 14375: (8572928..12217369, 1000), error = -2.6325279345594996%, (bench omitted)
  .sum_expreriment3(k=5), N = 1000, 6770 <= val < 14375: (9256960..11643929, 1000), error = -2.1145909378634835%, (bench omitted)
  .sum_expreriment3(k=6), N = 1000, 6770 <= val < 14375: (9758720..11512857, 1000), error = -0.3785428563453558%, (bench omitted)
```

```
              actual sum, N = 1000, 24166 <= val < 65100: 44438735
  .sum_expreriment3(k=1), N = 1000, 24166 <= val < 65100: (0..65535001, 1000), error = -26.26365264447784%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 24166 <= val < 65100: (25722880..58489881, 1000), error = -5.248472981960445%, (bench omitted)
  .sum_expreriment3(k=3), N = 1000, 24166 <= val < 65100: (32145408..52050969, 1000), error = -5.266907350085461%, (bench omitted)
  .sum_expreriment3(k=4), N = 1000, 24166 <= val < 65100: (37306368..50469913, 1000), error = -1.238997914769626%, (bench omitted)
  .sum_expreriment3(k=5), N = 1000, 24166 <= val < 65100: (38748160..48700441, 1000), error = -1.6076852772699313%, (bench omitted)
  .sum_expreriment3(k=6), N = 1000, 24166 <= val < 65100: (40427520..48618521, 1000), error = 0.18966561491905654%, (bench omitted)
```

```
              actual sum, N = 1000, 28230 <= val < 53275: 40482871
  .sum_expreriment3(k=1), N = 1000, 28230 <= val < 53275: (0..65535001, 1000), error = -19.058359274963475%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 28230 <= val < 53275: (26279936..59046937, 1000), error = 5.386389221258542%, (bench omitted)
  .sum_expreriment3(k=3), N = 1000, 28230 <= val < 53275: (29016064..48643097, 1000), error = -4.083927249132109%, (bench omitted)
  .sum_expreriment3(k=4), N = 1000, 28230 <= val < 53275: (31637504..46062617, 1000), error = -4.0333379517475425%, (bench omitted)
  .sum_expreriment3(k=5), N = 1000, 28230 <= val < 53275: (37105664..43985945, 1000), error = 0.15545587169447542%, (bench omitted)
```

```
              actual sum, N = 1000, 39732 <= val < 41913: 40815280
  .sum_expreriment3(k=1), N = 1000, 39732 <= val < 41913: (0..65535001, 1000), error = -19.717566558406556%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 39732 <= val < 41913: (36429824..44620825, 1000), error = -0.7104104149230386%, (bench omitted)
```

```
              actual sum, N = 1000, 8730 <= val < 16323: 12602523
  .sum_expreriment3(k=1), N = 1000, 8730 <= val < 16323: (0..16383001, 1000), error = -35.00111049192293%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 8730 <= val < 16323: (10391552..14486553, 1000), error = -1.2971291542177705%, (bench omitted)
  .sum_expreriment3(k=3), N = 1000, 8730 <= val < 16323: (10948608..13943833, 1000), error = -1.2402516543711128%, (bench omitted)
  .sum_expreriment3(k=4), N = 1000, 8730 <= val < 16323: (11528192..13575193, 1000), error = -0.4033398709131497%, (bench omitted)
```

```
              actual sum, N = 1000, 17458 <= val < 32819: 25211620
  .sum_expreriment3(k=1), N = 1000, 17458 <= val < 32819: (0..65535001, 1000), error = 29.969831371407313%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 17458 <= val < 32819: (163840..32930841, 1000), error = -34.366216847628195%, (bench omitted)
  .sum_expreriment3(k=3), N = 1000, 17458 <= val < 32819: (20725760..29039641, 1000), error = -1.3046365128460606%, (bench omitted)
  .sum_expreriment3(k=4), N = 1000, 17458 <= val < 32819: (21815296..27999257, 1000), error = -1.207157651908128%, (bench omitted)
  .sum_expreriment3(k=5), N = 1000, 17458 <= val < 32819: (23044096..27282457, 1000), error = -0.19175285047133028%, (bench omitted)
```

```
              actual sum, N = 1000, 34224 <= val < 58863: 46503144
  .sum_expreriment3(k=1), N = 1000, 34224 <= val < 58863: (0..65535001, 1000), error = -29.537022271010322%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 34224 <= val < 58863: (39256064..55639065, 1000), error = 2.0308734394388477%, (bench omitted)
  .sum_expreriment3(k=3), N = 1000, 34224 <= val < 58863: (41984000..53419033, 1000), error = 2.576969849608448%, (bench omitted)
  .sum_expreriment3(k=4), N = 1000, 34224 <= val < 58863: (42418176..50609177, 1000), error = 0.02264793107321948%, (bench omitted)
```

```
              actual sum, N = 1000, 36237 <= val < 55763: 46081053
  .sum_expreriment3(k=1), N = 1000, 36237 <= val < 55763: (0..65535001, 1000), error = -28.89159889640543%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 36237 <= val < 55763: (38551552..54934553, 1000), error = 1.4365969458206609%, (bench omitted)
  .sum_expreriment3(k=3), N = 1000, 36237 <= val < 55763: (41795584..52878361, 1000), error = 2.725456382257584%, (bench omitted)
  .sum_expreriment3(k=4), N = 1000, 36237 <= val < 55763: (42618880..49187865, 1000), error = -0.38558363672809304%, (bench omitted)
```

```
              actual sum, N = 1000, 14162 <= val < 16565: 15361703
  .sum_expreriment3(k=1), N = 1000, 14162 <= val < 16565: (0..32767001, 1000), error = 6.651586741391889%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 14162 <= val < 16565: (1196032..17579033, 1000), error = -38.89003061704812%, (bench omitted)
  .sum_expreriment3(k=3), N = 1000, 14162 <= val < 16565: (14356480..17450009, 1000), error = 3.5252666973186497%, (bench omitted)
  .sum_expreriment3(k=4), N = 1000, 14162 <= val < 16565: (14795776..17004569, 1000), error = 3.50526891452074%, (bench omitted)
  .sum_expreriment3(k=5), N = 1000, 14162 <= val < 16565: (14899712..15988761, 1000), error = 0.5372646509309548%, (bench omitted)
```

```
              actual sum, N = 1000, 35347 <= val < 40913: 38202635
  .sum_expreriment3(k=1), N = 1000, 35347 <= val < 40913: (0..65535001, 1000), error = -14.227120720861269%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 35347 <= val < 40913: (35753984..39848985, 1000), error = -1.0500610756299926%, (bench omitted)
  .sum_expreriment3(k=3), N = 1000, 35347 <= val < 40913: (36544512..39146521, 1000), error = -0.9348020103848856%, (bench omitted)
```

```
              actual sum, N = 1000, 18732 <= val < 25571: 22096578
  .sum_expreriment3(k=1), N = 1000, 18732 <= val < 25571: (0..32767001, 1000), error = -25.85503510996137%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 18732 <= val < 25571: (17432576..25623577, 1000), error = -2.572805617231772%, (bench omitted)
  .sum_expreriment3(k=3), N = 1000, 18732 <= val < 25571: (19988480..24607769, 1000), error = 0.9121140839092823%, (bench omitted)
```

```
              actual sum, N = 1000, 24757 <= val < 62326: 43810389
  .sum_expreriment3(k=1), N = 1000, 24757 <= val < 62326: (0..65535001, 1000), error = -25.206096663510564%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 24757 <= val < 62326: (25821184..58588185, 1000), error = -3.6651238134406885%, (bench omitted)
  .sum_expreriment3(k=3), N = 1000, 24757 <= val < 62326: (31752192..51608601, 1000), error = -4.861844527333459%, (bench omitted)
  .sum_expreriment3(k=4), N = 1000, 24757 <= val < 62326: (33529856..49896473, 1000), error = -4.787049482715162%, (bench omitted)
  .sum_expreriment3(k=5), N = 1000, 24757 <= val < 62326: (38084608..48012313, 1000), error = -1.7391514145195104%, (bench omitted)
  .sum_expreriment3(k=6), N = 1000, 24757 <= val < 62326: (40308736..47557657, 1000), error = 0.28031479017454053%, (bench omitted)
```

```
              actual sum, N = 1000, 14976 <= val < 60775: 37794281
  .sum_expreriment3(k=1), N = 1000, 14976 <= val < 60775: (0..65535001, 1000), error = -13.30037473129863%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 14976 <= val < 60775: (20119552..52886553, 1000), error = -3.4164666341979095%, (bench omitted)
  .sum_expreriment3(k=3), N = 1000, 14976 <= val < 60775: (24477696..47184921, 1000), error = -5.193836072711636%, (bench omitted)
  .sum_expreriment3(k=4), N = 1000, 14976 <= val < 60775: (30179328..46562329, 1000), error = 1.525487414352452%, (bench omitted)
  .sum_expreriment3(k=5), N = 1000, 14976 <= val < 60775: (31571968..45104153, 1000), error = 1.4387864661322702%, (bench omitted)
  .sum_expreriment3(k=6), N = 1000, 14976 <= val < 60775: (32808960..43490329, 1000), error = 0.9402560138662249%, (bench omitted)
```

```
              actual sum, N = 1000, 11474 <= val < 45549: 28497769
  .sum_expreriment3(k=1), N = 1000, 11474 <= val < 45549: (0..65535001, 1000), error = 14.982685135808351%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 11474 <= val < 45549: (12517376..45284377, 1000), error = 1.4145212560323581%, (bench omitted)
  .sum_expreriment3(k=3), N = 1000, 11474 <= val < 45549: (20054016..42695705, 1000), error = 10.095846450295811%, (bench omitted)
  .sum_expreriment3(k=4), N = 1000, 11474 <= val < 45549: (21913600..34528281, 1000), error = -0.9714058668943524%, (bench omitted)
```

```
              actual sum, N = 1000, 16287 <= val < 39805: 28386942
  .sum_expreriment3(k=1), N = 1000, 16287 <= val < 39805: (0..65535001, 1000), error = 15.431595273629686%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 16287 <= val < 39805: (10485760..43252761, 1000), error = -5.346408922806831%, (bench omitted)
  .sum_expreriment3(k=3), N = 1000, 16287 <= val < 39805: (21577728..43203609, 1000), error = 14.104111672190687%, (bench omitted)
  .sum_expreriment3(k=4), N = 1000, 16287 <= val < 39805: (24379392..40459289, 1000), error = 14.205115859256695%, (bench omitted)
  .sum_expreriment3(k=5), N = 1000, 16287 <= val < 39805: (25092096..31906841, 1000), error = 0.396400570374928%, (bench omitted)
```

```
              actual sum, N = 1000, 10445 <= val < 14392: 12452736
  .sum_expreriment3(k=1), N = 1000, 10445 <= val < 14392: (0..16383001, 1000), error = -34.2192751857905%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 10445 <= val < 14392: (10395648..14490649, 1000), error = -0.07699512781769405%, (bench omitted)
```

```
              actual sum, N = 1000, 6890 <= val < 20894: 13853991
  .sum_expreriment3(k=1), N = 1000, 6890 <= val < 20894: (0..32767001, 1000), error = 18.258341585468045%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 6890 <= val < 20894: (4997120..21380121, 1000), error = -4.802738792020292%, (bench omitted)
  .sum_expreriment3(k=3), N = 1000, 6890 <= val < 20894: (10067968..20757529, 1000), error = 11.251321009231201%, (bench omitted)
  .sum_expreriment3(k=4), N = 1000, 6890 <= val < 20894: (11309056..19463193, 1000), error = 11.0591453394188%, (bench omitted)
  .sum_expreriment3(k=5), N = 1000, 6890 <= val < 20894: (11399168..15805465, 1000), error = -1.8166245380121873%, (bench omitted)
  .sum_expreriment3(k=6), N = 1000, 6890 <= val < 20894: (11728896..15488025, 1000), error = -1.7722763065170173%, (bench omitted)
  .sum_expreriment3(k=7), N = 1000, 6890 <= val < 20894: (12044288..15182873, 1000), error = -1.7353194469377091%, (bench omitted)
  .sum_expreriment3(k=8), N = 1000, 6890 <= val < 20894: (12347392..14906393, 1000), error = -1.6392316120315078%, (bench omitted)
  .sum_expreriment3(k=9), N = 1000, 6890 <= val < 20894: (12741632..14824473, 1000), error = -0.5120473948626068%, (bench omitted)
```

```
              actual sum, N = 1000, 11983 <= val < 37793: 25104967
  .sum_expreriment3(k=1), N = 1000, 11983 <= val < 37793: (0..65535001, 1000), error = 30.521979973126435%, (bench omitted)
  .sum_expreriment3(k=2), N = 1000, 11983 <= val < 37793: (6684672..39451673, 1000), error = -8.113115623693112%, (bench omitted)
  .sum_expreriment3(k=3), N = 1000, 11983 <= val < 37793: (17121280..36846617, 1000), error = 7.48449898380667%, (bench omitted)
  .sum_expreriment3(k=4), N = 1000, 11983 <= val < 37793: (19668992..34176025, 1000), error = 7.239766537036277%, (bench omitted)
  .sum_expreriment3(k=5), N = 1000, 11983 <= val < 37793: (20439040..28597273, 1000), error = -2.3374298799118116%, (bench omitted)
  .sum_expreriment3(k=6), N = 1000, 11983 <= val < 37793: (22372352..27954201, 1000), error = 0.23226081117732597%, (bench omitted)
```

The distribution of sufficient k is:

```
 1:  5 *****
 2: 25 *************************
 3:  6 ******
 4: 32 ********************************
 5: 13 *************
 6: 10 **********
 7:  2 **
 8:  5 *****
 9:  2 **
```


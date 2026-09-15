---
splat_schema: 1
splat_id: d1bc292c-ffb9-56fa-8f3a-db0d56b6233d
splat_kind: rule
splat_key: dnd2024:rules/parts_of_a_stat_block_experience_points
splat_name: Experience Points
splat_origin:
  key: dnd2024:reference/pages/256
splat_sources:
- title: SRD 5.2.1
  locator: pp. 256
  url: https://www.dndbeyond.com/srd
---

# Experience Points

## Source text

<!-- SRD page 256 -->

The number of Experience Points (XP) a monster\
is worth is based on its CR, as detailed in the Ex-\
perience Points by Challenge Rating table. XP is\
awarded for defeating the monster in combat or\
otherwise neutralizing it.\
Unless a rule says otherwise, a monster sum-\
moned by a spell or another magical ability is worth\
the XP noted in its stat block.\
Experience Points by Challenge Rating\
CR XP CR XP\
0 0 or 10 14 11,500\
1/8 25 15 13,000\
1/4 50 16 15,000\
1/2 100 17 18,000\
1 200 18 20,000\
2 450 19 22,000\
3 700 20 25,000\
4 1,100 21 33,000\
5 1,800 22 41,000\
6 2,300 23 50,000\
7 2,900 24 62,000\
8 3,900 25 75,000\
9 5,000 26 90,000\
10 5,900 27 105,000\
CR XP CR XP\
11 7,200 28 120,000\
12 8,400 29 135,000\
13 10,000 30 155,000\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/parts_of_a_stat_block_experience_points ` |
| ` implementation ` | ` dnd2024:rules_parts_of_a_stat_block_experience_points ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` cr_zero_xp_options ` | ` [0, 10] ` |
| ` award_events ` | ` ["defeated_in_combat", "otherwise_neutralized"] ` |
| ` magically_summoned_xp ` | ` stat_block_unless_overridden ` |

#### Xp table

<!-- splat:table
path: splat_rule/parameters/xp_table
key: Key
order:
- row_1
- row_2
- row_3
- row_4
- row_5
- row_6
- row_7
- row_8
- row_9
- row_10
- row_11
- row_12
- row_13
- row_14
- row_15
- row_16
- row_17
- row_18
- row_19
- row_20
- row_21
- row_22
- row_23
- row_24
- row_25
- row_26
- row_27
- row_28
- row_29
- row_30
- row_31
- row_32
- row_33
- row_34
columns:
  Key:
    type: key
  Xp:
    path: xp
    type: integer
    nullable: true
  Details:
    type: section
    optional: true
-->

| Key | Xp | Details |
| --- | --- | --- |
| ` row_1 ` | null | [Rules](#^splat-67cb40b8-row_1 "section:row_1") |
| ` row_2 ` | ` 25 ` | [Rules](#^splat-67cb40b8-row_2 "section:row_2") |
| ` row_3 ` | ` 50 ` | [Rules](#^splat-67cb40b8-row_3 "section:row_3") |
| ` row_4 ` | ` 100 ` | [Rules](#^splat-67cb40b8-row_4 "section:row_4") |
| ` row_5 ` | ` 200 ` | [Rules](#^splat-67cb40b8-row_5 "section:row_5") |
| ` row_6 ` | ` 450 ` | [Rules](#^splat-67cb40b8-row_6 "section:row_6") |
| ` row_7 ` | ` 700 ` | [Rules](#^splat-67cb40b8-row_7 "section:row_7") |
| ` row_8 ` | ` 1100 ` | [Rules](#^splat-67cb40b8-row_8 "section:row_8") |
| ` row_9 ` | ` 1800 ` | [Rules](#^splat-67cb40b8-row_9 "section:row_9") |
| ` row_10 ` | ` 2300 ` | [Rules](#^splat-67cb40b8-row_10 "section:row_10") |
| ` row_11 ` | ` 2900 ` | [Rules](#^splat-67cb40b8-row_11 "section:row_11") |
| ` row_12 ` | ` 3900 ` | [Rules](#^splat-67cb40b8-row_12 "section:row_12") |
| ` row_13 ` | ` 5000 ` | [Rules](#^splat-67cb40b8-row_13 "section:row_13") |
| ` row_14 ` | ` 5900 ` | [Rules](#^splat-67cb40b8-row_14 "section:row_14") |
| ` row_15 ` | ` 7200 ` | [Rules](#^splat-67cb40b8-row_15 "section:row_15") |
| ` row_16 ` | ` 8400 ` | [Rules](#^splat-67cb40b8-row_16 "section:row_16") |
| ` row_17 ` | ` 10000 ` | [Rules](#^splat-67cb40b8-row_17 "section:row_17") |
| ` row_18 ` | ` 11500 ` | [Rules](#^splat-67cb40b8-row_18 "section:row_18") |
| ` row_19 ` | ` 13000 ` | [Rules](#^splat-67cb40b8-row_19 "section:row_19") |
| ` row_20 ` | ` 15000 ` | [Rules](#^splat-67cb40b8-row_20 "section:row_20") |
| ` row_21 ` | ` 18000 ` | [Rules](#^splat-67cb40b8-row_21 "section:row_21") |
| ` row_22 ` | ` 20000 ` | [Rules](#^splat-67cb40b8-row_22 "section:row_22") |
| ` row_23 ` | ` 22000 ` | [Rules](#^splat-67cb40b8-row_23 "section:row_23") |
| ` row_24 ` | ` 25000 ` | [Rules](#^splat-67cb40b8-row_24 "section:row_24") |
| ` row_25 ` | ` 33000 ` | [Rules](#^splat-67cb40b8-row_25 "section:row_25") |
| ` row_26 ` | ` 41000 ` | [Rules](#^splat-67cb40b8-row_26 "section:row_26") |
| ` row_27 ` | ` 50000 ` | [Rules](#^splat-67cb40b8-row_27 "section:row_27") |
| ` row_28 ` | ` 62000 ` | [Rules](#^splat-67cb40b8-row_28 "section:row_28") |
| ` row_29 ` | ` 75000 ` | [Rules](#^splat-67cb40b8-row_29 "section:row_29") |
| ` row_30 ` | ` 90000 ` | [Rules](#^splat-67cb40b8-row_30 "section:row_30") |
| ` row_31 ` | ` 105000 ` | [Rules](#^splat-67cb40b8-row_31 "section:row_31") |
| ` row_32 ` | ` 120000 ` | [Rules](#^splat-67cb40b8-row_32 "section:row_32") |
| ` row_33 ` | ` 135000 ` | [Rules](#^splat-67cb40b8-row_33 "section:row_33") |
| ` row_34 ` | ` 155000 ` | [Rules](#^splat-67cb40b8-row_34 "section:row_34") |

##### Row 1

^splat-67cb40b8-row_1

###### Cr

<!-- splat:yaml path=splat_rule/parameters/xp_table/@row_1/cr -->

```yaml
0
```

##### Row 2

^splat-67cb40b8-row_2

###### Cr

<!-- splat:yaml path=splat_rule/parameters/xp_table/@row_2/cr -->

```yaml
0.125
```

##### Row 3

^splat-67cb40b8-row_3

###### Cr

<!-- splat:yaml path=splat_rule/parameters/xp_table/@row_3/cr -->

```yaml
0.25
```

##### Row 4

^splat-67cb40b8-row_4

###### Cr

<!-- splat:yaml path=splat_rule/parameters/xp_table/@row_4/cr -->

```yaml
0.5
```

##### Row 5

^splat-67cb40b8-row_5

###### Cr

<!-- splat:yaml path=splat_rule/parameters/xp_table/@row_5/cr -->

```yaml
1
```

##### Row 6

^splat-67cb40b8-row_6

###### Cr

<!-- splat:yaml path=splat_rule/parameters/xp_table/@row_6/cr -->

```yaml
2
```

##### Row 7

^splat-67cb40b8-row_7

###### Cr

<!-- splat:yaml path=splat_rule/parameters/xp_table/@row_7/cr -->

```yaml
3
```

##### Row 8

^splat-67cb40b8-row_8

###### Cr

<!-- splat:yaml path=splat_rule/parameters/xp_table/@row_8/cr -->

```yaml
4
```

##### Row 9

^splat-67cb40b8-row_9

###### Cr

<!-- splat:yaml path=splat_rule/parameters/xp_table/@row_9/cr -->

```yaml
5
```

##### Row 10

^splat-67cb40b8-row_10

###### Cr

<!-- splat:yaml path=splat_rule/parameters/xp_table/@row_10/cr -->

```yaml
6
```

##### Row 11

^splat-67cb40b8-row_11

###### Cr

<!-- splat:yaml path=splat_rule/parameters/xp_table/@row_11/cr -->

```yaml
7
```

##### Row 12

^splat-67cb40b8-row_12

###### Cr

<!-- splat:yaml path=splat_rule/parameters/xp_table/@row_12/cr -->

```yaml
8
```

##### Row 13

^splat-67cb40b8-row_13

###### Cr

<!-- splat:yaml path=splat_rule/parameters/xp_table/@row_13/cr -->

```yaml
9
```

##### Row 14

^splat-67cb40b8-row_14

###### Cr

<!-- splat:yaml path=splat_rule/parameters/xp_table/@row_14/cr -->

```yaml
10
```

##### Row 15

^splat-67cb40b8-row_15

###### Cr

<!-- splat:yaml path=splat_rule/parameters/xp_table/@row_15/cr -->

```yaml
11
```

##### Row 16

^splat-67cb40b8-row_16

###### Cr

<!-- splat:yaml path=splat_rule/parameters/xp_table/@row_16/cr -->

```yaml
12
```

##### Row 17

^splat-67cb40b8-row_17

###### Cr

<!-- splat:yaml path=splat_rule/parameters/xp_table/@row_17/cr -->

```yaml
13
```

##### Row 18

^splat-67cb40b8-row_18

###### Cr

<!-- splat:yaml path=splat_rule/parameters/xp_table/@row_18/cr -->

```yaml
14
```

##### Row 19

^splat-67cb40b8-row_19

###### Cr

<!-- splat:yaml path=splat_rule/parameters/xp_table/@row_19/cr -->

```yaml
15
```

##### Row 20

^splat-67cb40b8-row_20

###### Cr

<!-- splat:yaml path=splat_rule/parameters/xp_table/@row_20/cr -->

```yaml
16
```

##### Row 21

^splat-67cb40b8-row_21

###### Cr

<!-- splat:yaml path=splat_rule/parameters/xp_table/@row_21/cr -->

```yaml
17
```

##### Row 22

^splat-67cb40b8-row_22

###### Cr

<!-- splat:yaml path=splat_rule/parameters/xp_table/@row_22/cr -->

```yaml
18
```

##### Row 23

^splat-67cb40b8-row_23

###### Cr

<!-- splat:yaml path=splat_rule/parameters/xp_table/@row_23/cr -->

```yaml
19
```

##### Row 24

^splat-67cb40b8-row_24

###### Cr

<!-- splat:yaml path=splat_rule/parameters/xp_table/@row_24/cr -->

```yaml
20
```

##### Row 25

^splat-67cb40b8-row_25

###### Cr

<!-- splat:yaml path=splat_rule/parameters/xp_table/@row_25/cr -->

```yaml
21
```

##### Row 26

^splat-67cb40b8-row_26

###### Cr

<!-- splat:yaml path=splat_rule/parameters/xp_table/@row_26/cr -->

```yaml
22
```

##### Row 27

^splat-67cb40b8-row_27

###### Cr

<!-- splat:yaml path=splat_rule/parameters/xp_table/@row_27/cr -->

```yaml
23
```

##### Row 28

^splat-67cb40b8-row_28

###### Cr

<!-- splat:yaml path=splat_rule/parameters/xp_table/@row_28/cr -->

```yaml
24
```

##### Row 29

^splat-67cb40b8-row_29

###### Cr

<!-- splat:yaml path=splat_rule/parameters/xp_table/@row_29/cr -->

```yaml
25
```

##### Row 30

^splat-67cb40b8-row_30

###### Cr

<!-- splat:yaml path=splat_rule/parameters/xp_table/@row_30/cr -->

```yaml
26
```

##### Row 31

^splat-67cb40b8-row_31

###### Cr

<!-- splat:yaml path=splat_rule/parameters/xp_table/@row_31/cr -->

```yaml
27
```

##### Row 32

^splat-67cb40b8-row_32

###### Cr

<!-- splat:yaml path=splat_rule/parameters/xp_table/@row_32/cr -->

```yaml
28
```

##### Row 33

^splat-67cb40b8-row_33

###### Cr

<!-- splat:yaml path=splat_rule/parameters/xp_table/@row_33/cr -->

```yaml
29
```

##### Row 34

^splat-67cb40b8-row_34

###### Cr

<!-- splat:yaml path=splat_rule/parameters/xp_table/@row_34/cr -->

```yaml
30
```

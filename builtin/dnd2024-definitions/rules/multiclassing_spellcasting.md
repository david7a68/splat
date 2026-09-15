---
splat_schema: 1
splat_id: cfb889a4-de9b-5ded-9aeb-d96d9d17507a
splat_kind: rule
splat_key: dnd2024:rules/multiclassing_spellcasting
splat_name: Spellcasting
splat_origin:
  key: dnd2024:reference/pages/025
splat_sources:
- title: SRD 5.2.1
  locator: pp. 25, 26
  url: https://www.dndbeyond.com/srd
---

# Spellcasting

## Source text

<!-- SRD page 25 -->

Your capacity for spellcasting depends partly on\
your combined levels in all your spellcasting classes\
and partly on your individual levels in those classes.\
Once you have the Spellcasting feature from more\
than one class, use the rules below. If you multiclass\
but have the Spellcasting feature from only one\
class, follow the rules for that class.\
Spells Prepared. You determine what spells you\
can prepare for each class individually, as if you\
were a single-classed member of that class. If you\
are a level 4 Ranger / level 3 Sorcerer, for example,\
you can prepare five level 1 Ranger spells, and you\
can prepare six Sorcerer spells of level 1 or 2 (as\
well as four Sorcerer cantrips).\
Each spell you prepare is associated with one of\
your classes, and you use the spellcasting ability of\
that class when you cast the spell.\
Cantrips. If a cantrip of yours increases in power\
at higher levels, the increase is based on your total\
character level, not your level in a particular class,\
unless the spell says otherwise.\
Spell Slots. You determine your available spell\
slots by adding together the following:\
•  All your levels in the Bard, Cleric, Druid, Sorcerer,\
and Wizard classes\
•  Half your levels (round up) in the Paladin and\
Ranger classes\
Then look up this total level in the Level column of\
the Multiclass Spellcaster table. You use the slots for\
that level to cast spells of an appropriate level from\
any class whose Spellcasting feature you have.\
This table might give you spell slots of a higher\
level than the spells you prepare. You can use those\
slots but only to cast your lower-level spells. If a\
lower-level spell that you cast, like Burning Hands,\
has an enhanced effect when cast at a higher level,\
you can use the enhanced effect as normal.\
For example, if you are a level 4 Ranger / level 3\
Sorcerer, you count as a level 5 character when de-\
termining your spell slots, counting all your levels\
as a Sorcerer and half your Ranger levels. As shown\

<!-- SRD page 26 -->

in the Multiclass Spellcaster table, you have four\
level 1 spell slots, three level 2 slots, and two level 3\
slots. However, you can’t prepare any level 3 spells,\
nor can you prepare any level 2 Ranger spells. You\
can use the spell slots of those levels to cast the\
spells you do prepare—and potentially enhance\
their effects.\
Pact Magic. If you have the Pact Magic feature\
from the Warlock class and the Spellcasting feature,\
you can use the spell slots you gain from Pact Magic\
to cast spells you have prepared from classes with\
the Spellcasting feature, and you can use the spell\
slots you gain from the Spellcasting feature to cast\
Warlock spells you have prepared.\
Multiclass Spellcaster:\
Spell Slots per Spell Level\
Level 1 2 3 4 5 6 7 8 9\
1 2 — — — — — — — —\
2 3 — — — — — — — —\
3 4 2 — — — — — — —\
4 4 3 — — — — — — —\
5 4 3 2 — — — — — —\
6 4 3 3 — — — — — —\
7 4 3 3 1 — — — — —\
8 4 3 3 2 — — — — —\
9 4 3 3 3 1 — — — —\
10 4 3 3 3 2 — — — —\
11 4 3 3 3 2 1 — — —\
12 4 3 3 3 2 1 — — —\
13 4 3 3 3 2 1 1 — —\
14 4 3 3 3 2 1 1 — —\
15 4 3 3 3 2 1 1 1 —\
16 4 3 3 3 2 1 1 1 —\
17 4 3 3 3 2 1 1 1 1\
18 4 3 3 3 3 1 1 1 1\
19 4 3 3 3 3 2 1 1 1\
20 4 3 3 3 3 2 2 1 1\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/multiclassing_spellcasting ` |
| ` implementation ` | ` dnd2024:rules_multiclassing_spellcasting ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` multiclass_table_requires_spellcasting_classes_min ` | ` 2 ` |
| ` single_spellcasting_class ` | ` use_class_slots ` |
| ` cantrip_scaling ` | ` total_character_level_unless_spell_overrides ` |
| ` slots_allow_casting_any_prepared_spell_with_sufficient_level ` | ` true ` |
| ` higher_slots_do_not_unlock_preparation ` | ` true ` |
| ` upcast_effects ` | ` spell_definition ` |

#### Spell preparation

<!-- splat:values path=splat_rule/parameters/spell_preparation -->

| Field | Value |
| --- | --- |
| ` separate_by_class ` | ` true ` |
| ` use_only_class_level ` | ` true ` |
| ` spell_associated_with_class ` | ` true ` |
| ` casting_ability ` | ` associated_class_ability ` |

#### Effective caster level

<!-- splat:values path=splat_rule/parameters/effective_caster_level -->

| Field | Value |
| --- | --- |
| ` full_level_classes ` | ` ["bard", "cleric", "druid", "sorcerer", "wizard"] ` |
| ` half_level_classes ` | ` ["paladin", "ranger"] ` |
| ` half_rounding ` | ` up ` |
| ` round_each_class_before_sum ` | ` true ` |

#### Slots by effective level

<!-- splat:table
path: splat_rule/parameters/slots_by_effective_level
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
columns:
  Key:
    type: key
  Level:
    path: level
    type: integer
  Details:
    type: section
    optional: true
-->

| Key | Level | Details |
| --- | --- | --- |
| ` row_1 ` | ` 1 ` | [Rules](#^splat-7b5019ca-row_1 "section:row_1") |
| ` row_2 ` | ` 2 ` | [Rules](#^splat-7b5019ca-row_2 "section:row_2") |
| ` row_3 ` | ` 3 ` | [Rules](#^splat-7b5019ca-row_3 "section:row_3") |
| ` row_4 ` | ` 4 ` | [Rules](#^splat-7b5019ca-row_4 "section:row_4") |
| ` row_5 ` | ` 5 ` | [Rules](#^splat-7b5019ca-row_5 "section:row_5") |
| ` row_6 ` | ` 6 ` | [Rules](#^splat-7b5019ca-row_6 "section:row_6") |
| ` row_7 ` | ` 7 ` | [Rules](#^splat-7b5019ca-row_7 "section:row_7") |
| ` row_8 ` | ` 8 ` | [Rules](#^splat-7b5019ca-row_8 "section:row_8") |
| ` row_9 ` | ` 9 ` | [Rules](#^splat-7b5019ca-row_9 "section:row_9") |
| ` row_10 ` | ` 10 ` | [Rules](#^splat-7b5019ca-row_10 "section:row_10") |
| ` row_11 ` | ` 11 ` | [Rules](#^splat-7b5019ca-row_11 "section:row_11") |
| ` row_12 ` | ` 12 ` | [Rules](#^splat-7b5019ca-row_12 "section:row_12") |
| ` row_13 ` | ` 13 ` | [Rules](#^splat-7b5019ca-row_13 "section:row_13") |
| ` row_14 ` | ` 14 ` | [Rules](#^splat-7b5019ca-row_14 "section:row_14") |
| ` row_15 ` | ` 15 ` | [Rules](#^splat-7b5019ca-row_15 "section:row_15") |
| ` row_16 ` | ` 16 ` | [Rules](#^splat-7b5019ca-row_16 "section:row_16") |
| ` row_17 ` | ` 17 ` | [Rules](#^splat-7b5019ca-row_17 "section:row_17") |
| ` row_18 ` | ` 18 ` | [Rules](#^splat-7b5019ca-row_18 "section:row_18") |
| ` row_19 ` | ` 19 ` | [Rules](#^splat-7b5019ca-row_19 "section:row_19") |
| ` row_20 ` | ` 20 ` | [Rules](#^splat-7b5019ca-row_20 "section:row_20") |

##### Row 1

^splat-7b5019ca-row_1

###### Slots

<!-- splat:yaml path=splat_rule/parameters/slots_by_effective_level/@row_1/slots -->

```yaml
'1': 2
'2': 0
'3': 0
'4': 0
'5': 0
'6': 0
'7': 0
'8': 0
'9': 0
```

##### Row 2

^splat-7b5019ca-row_2

###### Slots

<!-- splat:yaml path=splat_rule/parameters/slots_by_effective_level/@row_2/slots -->

```yaml
'1': 3
'2': 0
'3': 0
'4': 0
'5': 0
'6': 0
'7': 0
'8': 0
'9': 0
```

##### Row 3

^splat-7b5019ca-row_3

###### Slots

<!-- splat:yaml path=splat_rule/parameters/slots_by_effective_level/@row_3/slots -->

```yaml
'1': 4
'2': 2
'3': 0
'4': 0
'5': 0
'6': 0
'7': 0
'8': 0
'9': 0
```

##### Row 4

^splat-7b5019ca-row_4

###### Slots

<!-- splat:yaml path=splat_rule/parameters/slots_by_effective_level/@row_4/slots -->

```yaml
'1': 4
'2': 3
'3': 0
'4': 0
'5': 0
'6': 0
'7': 0
'8': 0
'9': 0
```

##### Row 5

^splat-7b5019ca-row_5

###### Slots

<!-- splat:yaml path=splat_rule/parameters/slots_by_effective_level/@row_5/slots -->

```yaml
'1': 4
'2': 3
'3': 2
'4': 0
'5': 0
'6': 0
'7': 0
'8': 0
'9': 0
```

##### Row 6

^splat-7b5019ca-row_6

###### Slots

<!-- splat:yaml path=splat_rule/parameters/slots_by_effective_level/@row_6/slots -->

```yaml
'1': 4
'2': 3
'3': 3
'4': 0
'5': 0
'6': 0
'7': 0
'8': 0
'9': 0
```

##### Row 7

^splat-7b5019ca-row_7

###### Slots

<!-- splat:yaml path=splat_rule/parameters/slots_by_effective_level/@row_7/slots -->

```yaml
'1': 4
'2': 3
'3': 3
'4': 1
'5': 0
'6': 0
'7': 0
'8': 0
'9': 0
```

##### Row 8

^splat-7b5019ca-row_8

###### Slots

<!-- splat:yaml path=splat_rule/parameters/slots_by_effective_level/@row_8/slots -->

```yaml
'1': 4
'2': 3
'3': 3
'4': 2
'5': 0
'6': 0
'7': 0
'8': 0
'9': 0
```

##### Row 9

^splat-7b5019ca-row_9

###### Slots

<!-- splat:yaml path=splat_rule/parameters/slots_by_effective_level/@row_9/slots -->

```yaml
'1': 4
'2': 3
'3': 3
'4': 3
'5': 1
'6': 0
'7': 0
'8': 0
'9': 0
```

##### Row 10

^splat-7b5019ca-row_10

###### Slots

<!-- splat:yaml path=splat_rule/parameters/slots_by_effective_level/@row_10/slots -->

```yaml
'1': 4
'2': 3
'3': 3
'4': 3
'5': 2
'6': 0
'7': 0
'8': 0
'9': 0
```

##### Row 11

^splat-7b5019ca-row_11

###### Slots

<!-- splat:yaml path=splat_rule/parameters/slots_by_effective_level/@row_11/slots -->

```yaml
'1': 4
'2': 3
'3': 3
'4': 3
'5': 2
'6': 1
'7': 0
'8': 0
'9': 0
```

##### Row 12

^splat-7b5019ca-row_12

###### Slots

<!-- splat:yaml path=splat_rule/parameters/slots_by_effective_level/@row_12/slots -->

```yaml
'1': 4
'2': 3
'3': 3
'4': 3
'5': 2
'6': 1
'7': 0
'8': 0
'9': 0
```

##### Row 13

^splat-7b5019ca-row_13

###### Slots

<!-- splat:yaml path=splat_rule/parameters/slots_by_effective_level/@row_13/slots -->

```yaml
'1': 4
'2': 3
'3': 3
'4': 3
'5': 2
'6': 1
'7': 1
'8': 0
'9': 0
```

##### Row 14

^splat-7b5019ca-row_14

###### Slots

<!-- splat:yaml path=splat_rule/parameters/slots_by_effective_level/@row_14/slots -->

```yaml
'1': 4
'2': 3
'3': 3
'4': 3
'5': 2
'6': 1
'7': 1
'8': 0
'9': 0
```

##### Row 15

^splat-7b5019ca-row_15

###### Slots

<!-- splat:yaml path=splat_rule/parameters/slots_by_effective_level/@row_15/slots -->

```yaml
'1': 4
'2': 3
'3': 3
'4': 3
'5': 2
'6': 1
'7': 1
'8': 1
'9': 0
```

##### Row 16

^splat-7b5019ca-row_16

###### Slots

<!-- splat:yaml path=splat_rule/parameters/slots_by_effective_level/@row_16/slots -->

```yaml
'1': 4
'2': 3
'3': 3
'4': 3
'5': 2
'6': 1
'7': 1
'8': 1
'9': 0
```

##### Row 17

^splat-7b5019ca-row_17

###### Slots

<!-- splat:yaml path=splat_rule/parameters/slots_by_effective_level/@row_17/slots -->

```yaml
'1': 4
'2': 3
'3': 3
'4': 3
'5': 2
'6': 1
'7': 1
'8': 1
'9': 1
```

##### Row 18

^splat-7b5019ca-row_18

###### Slots

<!-- splat:yaml path=splat_rule/parameters/slots_by_effective_level/@row_18/slots -->

```yaml
'1': 4
'2': 3
'3': 3
'4': 3
'5': 3
'6': 1
'7': 1
'8': 1
'9': 1
```

##### Row 19

^splat-7b5019ca-row_19

###### Slots

<!-- splat:yaml path=splat_rule/parameters/slots_by_effective_level/@row_19/slots -->

```yaml
'1': 4
'2': 3
'3': 3
'4': 3
'5': 3
'6': 2
'7': 1
'8': 1
'9': 1
```

##### Row 20

^splat-7b5019ca-row_20

###### Slots

<!-- splat:yaml path=splat_rule/parameters/slots_by_effective_level/@row_20/slots -->

```yaml
'1': 4
'2': 3
'3': 3
'4': 3
'5': 3
'6': 2
'7': 2
'8': 1
'9': 1
```

#### Pact magic

<!-- splat:values path=splat_rule/parameters/pact_magic -->

| Field | Value |
| --- | --- |
| ` pact_slots_cast_other_class_prepared_spells ` | ` true ` |
| ` spellcasting_slots_cast_prepared_warlock_spells ` | ` true ` |

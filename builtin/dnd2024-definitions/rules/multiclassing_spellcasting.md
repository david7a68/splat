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

<!-- splat:rows path=splat_rule/parameters/slots_by_effective_level -->

| Row | level |
| --- | --- |
| ` 0 ` | ` 1 ` |
| ` 1 ` | ` 2 ` |
| ` 2 ` | ` 3 ` |
| ` 3 ` | ` 4 ` |
| ` 4 ` | ` 5 ` |
| ` 5 ` | ` 6 ` |
| ` 6 ` | ` 7 ` |
| ` 7 ` | ` 8 ` |
| ` 8 ` | ` 9 ` |
| ` 9 ` | ` 10 ` |
| ` 10 ` | ` 11 ` |
| ` 11 ` | ` 12 ` |
| ` 12 ` | ` 13 ` |
| ` 13 ` | ` 14 ` |
| ` 14 ` | ` 15 ` |
| ` 15 ` | ` 16 ` |
| ` 16 ` | ` 17 ` |
| ` 17 ` | ` 18 ` |
| ` 18 ` | ` 19 ` |
| ` 19 ` | ` 20 ` |

##### Slots

<!-- splat:yaml path=splat_rule/parameters/slots_by_effective_level/0/slots -->

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

##### Slots

<!-- splat:yaml path=splat_rule/parameters/slots_by_effective_level/1/slots -->

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

##### Slots

<!-- splat:yaml path=splat_rule/parameters/slots_by_effective_level/2/slots -->

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

##### Slots

<!-- splat:yaml path=splat_rule/parameters/slots_by_effective_level/3/slots -->

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

##### Slots

<!-- splat:yaml path=splat_rule/parameters/slots_by_effective_level/4/slots -->

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

##### Slots

<!-- splat:yaml path=splat_rule/parameters/slots_by_effective_level/5/slots -->

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

##### Slots

<!-- splat:yaml path=splat_rule/parameters/slots_by_effective_level/6/slots -->

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

##### Slots

<!-- splat:yaml path=splat_rule/parameters/slots_by_effective_level/7/slots -->

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

##### Slots

<!-- splat:yaml path=splat_rule/parameters/slots_by_effective_level/8/slots -->

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

##### Slots

<!-- splat:yaml path=splat_rule/parameters/slots_by_effective_level/9/slots -->

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

##### Slots

<!-- splat:yaml path=splat_rule/parameters/slots_by_effective_level/10/slots -->

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

##### Slots

<!-- splat:yaml path=splat_rule/parameters/slots_by_effective_level/11/slots -->

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

##### Slots

<!-- splat:yaml path=splat_rule/parameters/slots_by_effective_level/12/slots -->

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

##### Slots

<!-- splat:yaml path=splat_rule/parameters/slots_by_effective_level/13/slots -->

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

##### Slots

<!-- splat:yaml path=splat_rule/parameters/slots_by_effective_level/14/slots -->

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

##### Slots

<!-- splat:yaml path=splat_rule/parameters/slots_by_effective_level/15/slots -->

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

##### Slots

<!-- splat:yaml path=splat_rule/parameters/slots_by_effective_level/16/slots -->

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

##### Slots

<!-- splat:yaml path=splat_rule/parameters/slots_by_effective_level/17/slots -->

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

##### Slots

<!-- splat:yaml path=splat_rule/parameters/slots_by_effective_level/18/slots -->

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

##### Slots

<!-- splat:yaml path=splat_rule/parameters/slots_by_effective_level/19/slots -->

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

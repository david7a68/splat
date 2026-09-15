---
splat_schema: 1
splat_id: 324d5c1b-b439-56a5-8b13-dd48e1ea576b
splat_kind: rule
splat_key: dnd2024:rules/create_your_character_choose_languages
splat_name: Choose Languages
splat_origin:
  key: dnd2024:reference/pages/020
splat_sources:
- title: SRD 5.2.1
  locator: pp. 20
  url: https://www.dndbeyond.com/srd
---

# Choose Languages

## Source text

<!-- SRD page 20 -->

Your character knows at least three languages:\
Common plus two languages you roll or choose\
from the Standard Languages table. Knowledge of a\
language means your character can communicate in\
it, read it, and write it. Your class and other features\
might also give you languages.\
The Standard Languages table lists languages that\
are widespread in the setting. Every player charac-\
ter knows Common. The other standard languages\
originated with the first members of the most\
prominent species in the setting and have since\
spread widely.\
Standard Languages\
1d12 Language\
— Common\
1 Common Sign Language\
2 Draconic\
3–4 Dwarvish\
5–6 Elvish\
7 Giant\
8 Gnomish\
9 Goblin\
10–11 Halfling\
12 Orc\
The Rare Languages table lists languages that are\
either secret or derived from other planes of exis-\
tence and thus less widespread in the worlds of the\
Material Plane. Some features let a character learn a\
rare language.\
Rare Languages\
Language Language\
Abyssal Primordial*\
Celestial Sylvan\
Deep Speech Thieves’ Cant\
Druidic Undercommon\
Infernal\
*Primordial includes the Aquan, Auran, Ignan, and Terran dialects.\
Creatures that know one of these dialects can communicate with\
those that know a different one.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/create_your_character_choose_languages ` |
| ` implementation ` | ` dnd2024:rules_create_your_character_choose_languages ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` granted_languages ` | ` ["common"] ` |
| ` knowledge ` | ` ["communicate", "read", "write"] ` |
| ` additional_grants_source ` | ` features ` |
| ` rare_requires_feature ` | ` true ` |
| ` rare_languages ` | ` ["abyssal", "celestial", "deep_speech", "druidic", "infernal", "primordial", "sylvan", "thieves_cant", "undercommon"] ` |

#### Additional standard languages

<!-- splat:values path=splat_rule/parameters/additional_standard_languages -->

| Field | Value |
| --- | --- |
| ` count ` | ` 2 ` |
| ` selection ` | ` ["choose", "roll"] ` |

##### Dice

<!-- splat:values path=splat_rule/parameters/additional_standard_languages/dice -->

| Field | Value |
| --- | --- |
| ` count ` | ` 1 ` |
| ` sides ` | ` 12 ` |

##### Table

<!-- splat:table
path: splat_rule/parameters/additional_standard_languages/table
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
columns:
  Key:
    type: key
  Roll:
    path: ''
    type: range
  Language:
    path: language
    type: symbol
-->

| Key | Roll | Language |
| --- | --- | --- |
| ` row_1 ` | ` 1 ` | ` Common sign language ` |
| ` row_2 ` | ` 2 ` | ` Draconic ` |
| ` row_3 ` | ` 3–4 ` | ` Dwarvish ` |
| ` row_4 ` | ` 5–6 ` | ` Elvish ` |
| ` row_5 ` | ` 7 ` | ` Giant ` |
| ` row_6 ` | ` 8 ` | ` Gnomish ` |
| ` row_7 ` | ` 9 ` | ` Goblin ` |
| ` row_8 ` | ` 10–11 ` | ` Halfling ` |
| ` row_9 ` | ` 12 ` | ` Orc ` |

#### Primordial

<!-- splat:values path=splat_rule/parameters/primordial -->

| Field | Value |
| --- | --- |
| ` dialects ` | ` ["aquan", "auran", "ignan", "terran"] ` |
| ` mutually_intelligible ` | ` true ` |

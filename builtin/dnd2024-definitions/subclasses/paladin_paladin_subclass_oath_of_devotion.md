---
splat_schema: 1
splat_id: d4f23084-9bb3-5bee-a9a5-3af40aae9d48
splat_kind: rule_group
splat_key: dnd2024:subclasses/paladin_paladin_subclass_oath_of_devotion
splat_name: 'Paladin Subclass: Oath of Devotion'
splat_origin:
  key: dnd2024:reference/pages/056
splat_sources:
- title: SRD 5.2.1
  locator: pp. 56
  url: https://www.dndbeyond.com/srd
---

# Paladin Subclass: Oath of Devotion

## Source text

<!-- SRD page 56 -->

Uphold the Ideals of Justice and Order\
The Oath of Devotion binds Paladins to the ideals\
of justice and order. These Paladins meet the ar-\
chetype of the knight in shining armor. They hold\
themselves to the highest standards of conduct, and\
some—for better or worse—hold the rest of the\
world to the same standards.\
Many who swear this oath are devoted to gods\
of law and good and use their gods’ tenets as the\
measure of personal devotion. Others hold angels as\
their ideals and incorporate images of angelic wings\
into their helmets or coats of arms.\
These paladins share the following tenets:\
•    Let your word be your promise.\
•    Protect the weak and never fear to act.\
•    Let your honorable deeds be an example.\

## Rule group

<!-- splat:values path=splat_rule_group -->

| Field | Value |
| --- | --- |
| ` category ` | ` subclass ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:subclasses/paladin_paladin_subclass_oath_of_devotion ` |
| ` implementation ` | ` dnd2024:subclasses_paladin_paladin_subclass_oath_of_devotion ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` class ` | ` paladin ` |
| ` minimum_class_level ` | ` 3 ` |

#### Feature grants

<!-- splat:rows path=splat_rule/parameters/feature_grants -->

| Row | class_level |
| --- | --- |
| ` 0 ` | ` 15 ` |
| ` 1 ` | ` 20 ` |
| ` 2 ` | ` 3 ` |
| ` 3 ` | ` 3 ` |
| ` 4 ` | ` 7 ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/0/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/oath_of_devotion_level_15_smite_of_protection ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/1/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/oath_of_devotion_level_20_holy_nimbus ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/2/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/oath_of_devotion_level_3_oath_of_devotion_spells ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/3/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/oath_of_devotion_level_3_sacred_weapon ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/4/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/oath_of_devotion_level_7_aura_of_devotion ` |

#### Grant eligible features

<!-- splat:values path=splat_rule/parameters/grant_eligible_features -->

| Field | Value |
| --- | --- |
| ` "on" ` | ` ["subclass.selected", "character.level_gained"] ` |
| ` comparison ` | ` class_level_at_least_threshold ` |
| ` persistent ` | ` true ` |

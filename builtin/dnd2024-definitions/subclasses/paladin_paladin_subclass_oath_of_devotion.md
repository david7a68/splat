---
splat_schema: 2
splat_id: d4f23084-9bb3-5bee-a9a5-3af40aae9d48
splat_kind: rule_group
splat_key: dnd2024:subclasses/paladin_paladin_subclass_oath_of_devotion
splat_name: 'Paladin Subclass: Oath of Devotion'
splat_origin: dnd2024:reference/pages/056
splat_sources:
- SRD 5.2.1, pp. 56, https://www.dndbeyond.com/srd
---

# Paladin Subclass: Oath of Devotion

## Source text

<!-- SRD page 56 -->

### Uphold the Ideals of Justice and Order

The Oath of Devotion binds Paladins to the ideals of justice and order. These Paladins meet the archetype of the knight in shining armor. They hold themselves to the highest standards of conduct, and some—for better or worse—hold the rest of the world to the same standards.

Many who swear this oath are devoted to gods of law and good and use their gods’ tenets as the measure of personal devotion. Others hold angels as their ideals and incorporate images of angelic wings into their helmets or coats of arms.

These paladins share the following tenets:

•    Let your word be your promise.

•    Protect the weak and never fear to act.

•    Let your honorable deeds be an example.

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

<!-- splat:table
path: splat_rule/parameters/feature_grants
key: Item
order:
- oath_of_devotion_level_15_smite_of_protection
- oath_of_devotion_level_20_holy_nimbus
- oath_of_devotion_level_3_oath_of_devotion_spells
- oath_of_devotion_level_3_sacred_weapon
- oath_of_devotion_level_7_aura_of_devotion
columns:
  Item:
    path: definition
    type: reference
  Class level:
    path: class_level
    type: integer
-->

| Item | Class level |
| --- | --- |
| [Level 15: Smite of Protection](<../class_features/oath_of_devotion_level_15_smite_of_protection.md> "dnd2024:class_features/oath_of_devotion_level_15_smite_of_protection") | ` 15 ` |
| [Level 20: Holy Nimbus](<../class_features/oath_of_devotion_level_20_holy_nimbus.md> "dnd2024:class_features/oath_of_devotion_level_20_holy_nimbus") | ` 20 ` |
| [Level 3: Oath of Devotion Spells](<../class_features/oath_of_devotion_level_3_oath_of_devotion_spells.md> "dnd2024:class_features/oath_of_devotion_level_3_oath_of_devotion_spells") | ` 3 ` |
| [Level 3: Sacred Weapon](<../class_features/oath_of_devotion_level_3_sacred_weapon.md> "dnd2024:class_features/oath_of_devotion_level_3_sacred_weapon") | ` 3 ` |
| [Level 7: Aura of Devotion](<../class_features/oath_of_devotion_level_7_aura_of_devotion.md> "dnd2024:class_features/oath_of_devotion_level_7_aura_of_devotion") | ` 7 ` |

#### Grant eligible features

<!-- splat:values path=splat_rule/parameters/grant_eligible_features -->

| Field | Value |
| --- | --- |
| ` "on" ` | ` ["subclass.selected", "character.level_gained"] ` |
| ` comparison ` | ` class_level_at_least_threshold ` |
| ` persistent ` | ` true ` |

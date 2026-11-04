---
splat_schema: 2
splat_id: 850bcc82-f5d2-5fd2-bb97-a39bc776c3c4
splat_kind: rule_group
splat_key: dnd2024:subclasses/sorcerer_sorcerer_subclass_draconic_sorcery
splat_name: 'Sorcerer Subclass: Draconic Sorcery'
splat_origin: dnd2024:reference/pages/069
splat_sources:
- SRD 5.2.1, pp. 69, https://www.dndbeyond.com/srd
---

# Sorcerer Subclass: Draconic Sorcery

## Source text

<!-- SRD page 69 -->

### Breathe the Magic of Dragons

Your innate magic comes from the gift of a dragon. Perhaps an ancient dragon facing death bequeathed some of its magical power to you or your ancestor. You might have absorbed magic from a site infused with dragons’ power. Or perhaps you handled a treasure taken from a dragon’s hoard that was steeped in draconic power. Or you might have a dragon for an ancestor.

## Rule group

<!-- splat:values path=splat_rule_group -->

| Field | Value |
| --- | --- |
| ` category ` | ` subclass ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:subclasses/sorcerer_sorcerer_subclass_draconic_sorcery ` |
| ` implementation ` | ` dnd2024:subclasses_sorcerer_sorcerer_subclass_draconic_sorcery ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` class ` | ` sorcerer ` |
| ` minimum_class_level ` | ` 3 ` |

#### Feature grants

<!-- splat:table
path: splat_rule/parameters/feature_grants
key: Item
order:
- draconic_sorcery_level_14_dragon_wings
- draconic_sorcery_level_18_dragon_companion
- draconic_sorcery_level_3_draconic_resilience
- draconic_sorcery_level_3_draconic_spells
- draconic_sorcery_level_6_elemental_affinity
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
| [Level 14: Dragon Wings](<../class_features/draconic_sorcery_level_14_dragon_wings.md> "dnd2024:class_features/draconic_sorcery_level_14_dragon_wings") | ` 14 ` |
| [Level 18: Dragon Companion](<../class_features/draconic_sorcery_level_18_dragon_companion.md> "dnd2024:class_features/draconic_sorcery_level_18_dragon_companion") | ` 18 ` |
| [Level 3: Draconic Resilience](<../class_features/draconic_sorcery_level_3_draconic_resilience.md> "dnd2024:class_features/draconic_sorcery_level_3_draconic_resilience") | ` 3 ` |
| [Level 3: Draconic Spells](<../class_features/draconic_sorcery_level_3_draconic_spells.md> "dnd2024:class_features/draconic_sorcery_level_3_draconic_spells") | ` 3 ` |
| [Level 6: Elemental Affinity](<../class_features/draconic_sorcery_level_6_elemental_affinity.md> "dnd2024:class_features/draconic_sorcery_level_6_elemental_affinity") | ` 6 ` |

#### Grant eligible features

<!-- splat:values path=splat_rule/parameters/grant_eligible_features -->

| Field | Value |
| --- | --- |
| ` "on" ` | ` ["subclass.selected", "character.level_gained"] ` |
| ` comparison ` | ` class_level_at_least_threshold ` |
| ` persistent ` | ` true ` |

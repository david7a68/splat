---
splat_schema: 1
splat_id: 8c5bc5c2-5e14-5ec3-be06-811ab3553115
splat_kind: rule
splat_key: dnd2024:class_features/draconic_sorcery_level_3_draconic_spells
splat_name: 'Level 3: Draconic Spells'
splat_origin:
  key: dnd2024:reference/pages/070
splat_sources:
- title: SRD 5.2.1
  locator: pp. 70
  url: https://www.dndbeyond.com/srd
---

# Level 3: Draconic Spells

## Source text

<!-- SRD page 70 -->

When you reach a Sorcerer level specified in the\
Draconic Spells table, you thereafter always have\
the listed spells prepared.\
Draconic Spells\
Sorcerer\
Level Spells\
3 Alter Self, Chromatic Orb, Command,\
Dragon’s Breath\
5 Fear, Fly\
7 Arcane Eye, Charm Monster\
9 Legend Lore, Summon Dragon\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/draconic_sorcery_level_3_draconic_spells ` |
| ` implementation ` | ` dnd2024:class_features_draconic_sorcery_level_3_draconic_spells ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` class ` | ` sorcerer ` |
| ` include_lower_levels ` | ` true ` |
| ` counts_against_prepared_limit ` | ` false ` |

#### Always prepared

<!-- splat:rows path=splat_rule/parameters/always_prepared -->

| Row | minimum_class_level |
| --- | --- |
| ` 0 ` | ` 3 ` |
| ` 1 ` | ` 5 ` |
| ` 2 ` | ` 7 ` |
| ` 3 ` | ` 9 ` |

##### Spells

<!-- splat:rows path=splat_rule/parameters/always_prepared/0/spells -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:spells/alter_self ` |
| ` 1 ` | ` dnd2024:spells/chromatic_orb ` |
| ` 2 ` | ` dnd2024:spells/command ` |
| ` 3 ` | ` dnd2024:spells/dragon_s_breath ` |

##### Spells

<!-- splat:rows path=splat_rule/parameters/always_prepared/1/spells -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:spells/fear ` |
| ` 1 ` | ` dnd2024:spells/fly ` |

##### Spells

<!-- splat:rows path=splat_rule/parameters/always_prepared/2/spells -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:spells/arcane_eye ` |
| ` 1 ` | ` dnd2024:spells/charm_monster ` |

##### Spells

<!-- splat:rows path=splat_rule/parameters/always_prepared/3/spells -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:spells/legend_lore ` |
| ` 1 ` | ` dnd2024:spells/summon_dragon ` |

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` sorcerer ` |
| ` minimum_level ` | ` 3 ` |

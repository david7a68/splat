---
splat_schema: 2
splat_id: 8c5bc5c2-5e14-5ec3-be06-811ab3553115
splat_kind: rule
splat_key: dnd2024:class_features/draconic_sorcery_level_3_draconic_spells
splat_name: 'Level 3: Draconic Spells'
splat_origin: dnd2024:reference/pages/070
splat_sources:
- SRD 5.2.1, pp. 70, https://www.dndbeyond.com/srd
---

# Level 3: Draconic Spells

## Source text

<!-- SRD page 70 -->

When you reach a Sorcerer level specified in the Draconic Spells table, you thereafter always have the listed spells prepared.

**Draconic Spells**

| Sorcerer Level | Spells |
| --- | --- |
| 3 | Alter Self, Chromatic Orb, Command, Dragon’s Breath |
| 5 | Fear, Fly |
| 7 | Arcane Eye, Charm Monster |
| 9 | Legend Lore, Summon Dragon |

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

<!-- splat:table
path: splat_rule/parameters/always_prepared
key: Key
order:
- row_1
- row_2
- row_3
- row_4
columns:
  Key:
    type: key
  Minimum class level:
    path: minimum_class_level
    type: integer
  Details:
    type: section
    optional: true
-->

| Key | Minimum class level | Details |
| --- | --- | --- |
| ` row_1 ` | ` 3 ` | [Rules](#^splat-403d5a47-row_1 "section:row_1") |
| ` row_2 ` | ` 5 ` | [Rules](#^splat-403d5a47-row_2 "section:row_2") |
| ` row_3 ` | ` 7 ` | [Rules](#^splat-403d5a47-row_3 "section:row_3") |
| ` row_4 ` | ` 9 ` | [Rules](#^splat-403d5a47-row_4 "section:row_4") |

##### Row 1

^splat-403d5a47-row_1

###### Spells

<!-- splat:rows path=splat_rule/parameters/always_prepared/@row_1/spells -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:spells/alter_self ` |
| ` 1 ` | ` dnd2024:spells/chromatic_orb ` |
| ` 2 ` | ` dnd2024:spells/command ` |
| ` 3 ` | ` dnd2024:spells/dragon_s_breath ` |

##### Row 2

^splat-403d5a47-row_2

###### Spells

<!-- splat:rows path=splat_rule/parameters/always_prepared/@row_2/spells -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:spells/fear ` |
| ` 1 ` | ` dnd2024:spells/fly ` |

##### Row 3

^splat-403d5a47-row_3

###### Spells

<!-- splat:rows path=splat_rule/parameters/always_prepared/@row_3/spells -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:spells/arcane_eye ` |
| ` 1 ` | ` dnd2024:spells/charm_monster ` |

##### Row 4

^splat-403d5a47-row_4

###### Spells

<!-- splat:rows path=splat_rule/parameters/always_prepared/@row_4/spells -->

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

---
splat_schema: 1
splat_id: aa37a87f-10b2-5eae-b22b-db3e9e858235
splat_kind: spell
splat_key: dnd2024:spells/vicious_mockery
splat_name: Vicious Mockery
splat_origin:
  key: dnd2024:reference/spell/vicious_mockery
splat_sources:
- title: SRD 5.2.1
  locator: pp. 171, 172
  url: https://www.dndbeyond.com/srd
---

# Vicious Mockery

## Source text

<!-- SRD page 171 -->

Enchantment Cantrip (Bard)\
Casting Time: Action\
Range: 60 feet\
Components: V\
Duration: Instantaneous

<!-- SRD page 172 -->

You unleash a string of insults laced with subtle enchantments at one creature you can see or hear within range. The target must succeed on a Wisdom saving throw or take 1d6 Psychic damage and have Disadvantage on the next attack roll it makes before the end of its next turn.

Cantrip Upgrade. The damage increases by 1d6 when you reach levels 5 (2d6), 11 (3d6), and 17 (4d6).

Original layout: [[../../srd-5.2.1/pages/171|page 171]], [[../../srd-5.2.1/pages/172|page 172]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 0 ` |
| ` school ` | ` enchantment ` |
| ` classes ` | ` ["bard"] ` |
| ` components ` | ` ["verbal"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` Instantaneous ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` feet ` | ` 60 ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/vicious_mockery ` |
| ` implementation ` | ` dnd2024:spell_vicious_mockery ` |

### Parameters

#### Targets

<!-- splat:values path=splat_rule/parameters/targets -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` maximum ` | ` 1 ` |
| ` within_spell_range ` | ` true ` |
| ` perception_any ` | ` ["caster_sees_target", "caster_hears_target"] ` |

#### Save

<!-- splat:values path=splat_rule/parameters/save -->

| Field | Value |
| --- | --- |
| ` ability ` | ` wisdom ` |

##### Dc

<!-- splat:values path=splat_rule/parameters/save/dc -->

| Field | Value |
| --- | --- |
| ` binding ` | ` save_dc ` |

##### Failure

<!-- splat:values path=splat_rule/parameters/save/failure -->

| Field | Value |
| --- | --- |
| ` disadvantage_next_attack ` | ` true ` |
| ` until ` | ` target_next_turn_end ` |

###### Damage

<!-- splat:values path=splat_rule/parameters/save/failure/damage -->

| Field | Value |
| --- | --- |
| ` type ` | ` psychic ` |

###### Dice

<!-- splat:values path=splat_rule/parameters/save/failure/damage/dice -->

| Field | Value |
| --- | --- |
| ` sides ` | ` 6 ` |
| ` add ` | ` 0 ` |

###### Count

###### By caster level

<!-- splat:table
path: splat_rule/parameters/save/failure/damage/dice/count/by_caster_level
key: Key
order:
- row_1
- row_2
- row_3
- row_4
columns:
  Key:
    type: key
  Minimum:
    path: minimum
    type: integer
  Count:
    path: count
    type: integer
-->

| Key | Minimum | Count |
| --- | --- | --- |
| ` row_1 ` | ` 1 ` | ` 1 ` |
| ` row_2 ` | ` 5 ` | ` 2 ` |
| ` row_3 ` | ` 11 ` | ` 3 ` |
| ` row_4 ` | ` 17 ` | ` 4 ` |

##### Success

<!-- splat:values path=splat_rule/parameters/save/success -->

| Field | Value |
| --- | --- |
| ` effect ` | ` none ` |

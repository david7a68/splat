---
splat_schema: 2
splat_id: 4619c253-1d46-5a03-99bb-40d356164048
splat_kind: spell
splat_key: dnd2024:spells/ray_of_frost
splat_name: Ray of Frost
splat_origin: dnd2024:reference/spell/ray_of_frost
splat_sources:
- SRD 5.2.1, pp. 157, https://www.dndbeyond.com/srd
---

# Ray of Frost


## Spell

| Field | Value |
| --- | --- |
| Level | Cantrip |
| School | Evocation |
| Classes | Sorcerer, Wizard |
| Casting Time | Action |
| Range | 60 feet |
| Components | V, S |
| Duration | Instantaneous |
| Ritual | No |
| Concentration | No |


## Source text

<!-- SRD page 157 -->

Evocation Cantrip (Sorcerer, Wizard)\
Casting Time: Action\
Range: 60 feet\
Components: V, S\
Duration: Instantaneous

A frigid beam of blue-white light streaks toward a creature within range. Make a ranged spell attack against the target. On a hit, it takes 1d8 Cold damage, and its Speed is reduced by 10 feet until the start of your next turn.

Cantrip Upgrade. The damage increases by 1d8 when you reach levels 5 (2d8), 11 (3d8), and 17 (4d8).

Original layout: [[../../srd-5.2.1/pages/157|page 157]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/ray_of_frost ` |
| ` implementation ` | ` dnd2024:spell_ray_of_frost ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` miss ` | ` no_effect ` |

#### Spell lifetime

<!-- splat:values path=splat_rule/parameters/spell_lifetime -->

| Field | Value |
| --- | --- |
| ` concentration ` | ` false ` |
| ` instantaneous ` | ` true ` |

#### Target

<!-- splat:values path=splat_rule/parameters/target -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` range_feet ` | ` 60 ` |

#### Attack

<!-- splat:values path=splat_rule/parameters/attack -->

| Field | Value |
| --- | --- |
| ` kind ` | ` ranged_spell_attack ` |

#### Hit

<!-- splat:values path=splat_rule/parameters/hit -->

| Field | Value |
| --- | --- |
| ` speed_modifier_feet ` | ` -10 ` |

##### Damage

<!-- splat:values path=splat_rule/parameters/hit/damage -->

| Field | Value |
| --- | --- |
| ` type ` | ` cold ` |

###### Dice

<!-- splat:values path=splat_rule/parameters/hit/damage/dice -->

| Field | Value |
| --- | --- |
| ` sides ` | ` 8 ` |

###### Counts by character level

<!-- splat:table
path: splat_rule/parameters/hit/damage/dice/counts_by_character_level
key: Key
order:
- row_1
- row_2
- row_3
- row_4
columns:
  Key:
    type: key
  Minimum level:
    path: minimum_level
    type: integer
  Count:
    path: count
    type: integer
-->

| Key | Minimum level | Count |
| --- | --- | --- |
| ` row_1 ` | ` 1 ` | ` 1 ` |
| ` row_2 ` | ` 5 ` | ` 2 ` |
| ` row_3 ` | ` 11 ` | ` 3 ` |
| ` row_4 ` | ` 17 ` | ` 4 ` |

##### Speed modifier until

<!-- splat:values path=splat_rule/parameters/hit/speed_modifier_until -->

| Field | Value |
| --- | --- |
| ` event ` | ` turn.start ` |
| ` subject ` | ` caster ` |
| ` occurrence ` | ` next ` |

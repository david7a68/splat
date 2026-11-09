---
splat_schema: 2
splat_id: 53786ff2-4da2-5c8b-8ab6-452fe723c13e
splat_kind: spell
splat_key: dnd2024:spells/chromatic_orb
splat_name: Chromatic Orb
splat_origin: dnd2024:reference/spell/chromatic_orb
splat_sources:
- SRD 5.2.1, pp. 115, https://www.dndbeyond.com/srd
---

# Chromatic Orb


## Spell

| Field | Value |
| --- | --- |
| Level | 1 |
| School | Evocation |
| Classes | Sorcerer, Wizard |
| Casting Time | Action |
| Range | 90 feet |
| Components | V, S, M (a diamond worth 50+ GP) |
| Duration | Instantaneous |
| Ritual | No |
| Concentration | No |


## Source text

<!-- SRD page 115 -->

Level 1 Evocation (Sorcerer, Wizard)\
Casting Time: Action\
Range: 90 feet\
Components: V, S, M (a diamond worth 50+ GP)\
Duration: Instantaneous

You hurl an orb of energy at a target within range. Choose Acid, Cold, Fire, Lightning, Poison, or Thunder for the type of orb you create, and then make a ranged spell attack against the target. On a hit, the target takes 3d8 damage of the chosen type.

If you roll the same number on two or more of the d8s, the orb leaps to a different target of your choice within 30 feet of the target. Make an attack roll against the new target, and make a new damage roll. The orb can’t leap again unless you cast the spell with a level 2+ spell slot.

Using a Higher-Level Spell Slot. The damage increases by 1d8 for each spell slot level above 1. The orb can leap a maximum number of times equal to the level of the slot expended, and a creature can be targeted only once by each casting of this spell.

Original layout: [[../../srd-5.2.1/pages/115|page 115]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/chromatic_orb ` |
| ` implementation ` | ` dnd2024:spell_chromatic_orb ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_type_choice ` | ` ["acid", "cold", "fire", "lightning", "poison", "thunder"] ` |

#### Targets

<!-- splat:values path=splat_rule/parameters/targets -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature_or_object ` |
| ` within_spell_range ` | ` true ` |

#### Attack

<!-- splat:yaml path=splat_rule/parameters/attack -->

```yaml
kind: ranged_spell
hit:
  dice:
    count:
      op: add
      args:
      - 3
      - op: multiply
        args:
        - 1
        - op: subtract
          args:
          - binding: cast_level
          - 1
    sides: 8
    add: 0
  type: chosen
```

#### Leap

<!-- splat:yaml path=splat_rule/parameters/leap -->

```yaml
trigger:
  damage_roll_d8_matching_faces_minimum: 2
distance_from_previous_target_maximum_feet: 30
target: different_target_caster_choice
repeat_attack_and_damage_roll: true
maximum_leaps:
  binding: cast_level
maximum_targets_per_creature_per_cast: 1
```

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> material:
>   consumed: false
> ```


---
splat_schema: 1
splat_id: e9e546bf-ca59-56f1-89e6-bd90198954a9
splat_kind: spell
splat_key: dnd2024:spells/disintegrate
splat_name: Disintegrate
splat_origin:
  key: dnd2024:reference/spell/disintegrate
splat_sources:
- title: SRD 5.2.1
  locator: pp. 124
  url: https://www.dndbeyond.com/srd
---

# Disintegrate

## Source text

<!-- SRD page 124 -->

Level 6 Transmutation (Sorcerer, Wizard)\
Casting Time: Action\
Range: 60 feet\
Components: V, S, M (a lodestone and dust)\
Duration: Instantaneous

You launch a green ray at a target you can see within range. The target can be a creature, a nonmagical object, or a creation of magical force, such as the wall created by Wall of Force.

A creature targeted by this spell makes a Dexterity saving throw. On a failed save, the target takes 10d6 + 40 Force damage. If this damage reduces it to 0 Hit Points, it and everything nonmagical it is wearing and carrying are disintegrated into gray dust. The target can be revived only by a True Resurrection or a Wish spell.

This spell automatically disintegrates a Large or smaller nonmagical object or a creation of magical force. If such a target is Huge or larger, this spell disintegrates a 10-foot-Cube portion of it.

Using a Higher-Level Spell Slot. The damage increases by 3d6 for each spell slot level above 6.

Original layout: [[../../srd-5.2.1/pages/124|page 124]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 6 ` |
| ` school ` | ` transmutation ` |
| ` classes ` | ` ["sorcerer", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` Instantaneous ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` feet ` | ` 60 ` |

### Material

<!-- splat:values path=splat_spell/material -->

| Field | Value |
| --- | --- |
| ` description ` | ` a lodestone and dust ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/disintegrate ` |
| ` implementation ` | ` dnd2024:spell_disintegrate ` |

### Parameters

#### Targets

<!-- splat:values path=splat_rule/parameters/targets -->

| Field | Value |
| --- | --- |
| ` kind_choices ` | ` ["creature", "nonmagical_object", "magical_force_creation"] ` |
| ` visible ` | ` true ` |
| ` within_spell_range ` | ` true ` |

#### Creature

<!-- splat:yaml path=splat_rule/parameters/creature -->

```yaml
save:
  ability: dexterity
  dc:
    binding: save_dc
  failure:
    damage:
      dice:
        count:
          op: add
          args:
          - 10
          - op: multiply
            args:
            - 3
            - op: subtract
              args:
              - binding: cast_level
              - 6
        sides: 6
        add: 40
      type: force
    if_damage_reduces_to_zero_hp:
      transform_to: gray_dust
      include_worn_carried_nonmagical_items: true
      revival_spells_only:
      - key: dnd2024:spells/true_resurrection
      - key: dnd2024:spells/wish
  success:
    effect: none
```

#### Object or force

<!-- splat:yaml path=splat_rule/parameters/object_or_force -->

```yaml
automatic_disintegration: true
maximum_size_for_whole_target: large
huge_or_larger:
  portion_shape: cube
  side_feet: 10
```

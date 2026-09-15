---
splat_schema: 1
splat_id: 9e6c6bc6-acae-5416-8013-5c35fb48a40e
splat_kind: spell
splat_key: dnd2024:spells/ice_knife
splat_name: Ice Knife
splat_origin:
  key: dnd2024:reference/spell/ice_knife
splat_sources:
- title: SRD 5.2.1
  locator: pp. 141
  url: https://www.dndbeyond.com/srd
---

# Ice Knife

## Source text

<!-- SRD page 141 -->

Level 1 Conjuration (Druid, Sorcerer, Wizard)\
Casting Time: Action\
Range: 60 feet\
Components: S, M (a drop of water or a piece of ice)\
Duration: Instantaneous\
You create a shard of ice and fling it at one creature\
within range. Make a ranged spell attack against\
the target. On a hit, the target takes 1d10 Piercing\
damage. Hit or miss, the shard then explodes. The\
target and each creature within 5 feet of it must\
succeed on a Dexterity saving throw or take 2d6\
Cold damage.\
Using a Higher-Level Spell Slot. The Cold damage\
increases by 1d6 for each spell slot level above 1.\

Original layout: [[../../srd-5.2.1/pages/141|page 141]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 1 ` |
| ` school ` | ` conjuration ` |
| ` classes ` | ` ["druid", "sorcerer", "wizard"] ` |
| ` components ` | ` ["somatic", "material"] ` |
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
| ` description ` | ` a drop of water or a piece of ice ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/ice_knife ` |
| ` implementation ` | ` dnd2024:spell_ice_knife ` |

### Parameters

#### Target

<!-- splat:values path=splat_rule/parameters/target -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` count ` | ` 1 ` |
| ` visible_to_caster ` | ` false ` |

#### Attack

<!-- splat:yaml path=splat_rule/parameters/attack -->

```yaml
kind: ranged_spell
hit:
  damage:
    amount:
      dice:
        count: 1
        sides: 10
        add: 0
    type: piercing
```

#### Explosion

<!-- splat:yaml path=splat_rule/parameters/explosion -->

```yaml
after:
- hit
- miss
targets:
- original_target
- creatures_within_5_feet_of_target
save:
  ability: dexterity
  dc:
    binding: save_dc
  failure:
    damage:
      amount:
        dice:
          count: 2
          sides: 6
          add: 0
      type: cold
  success:
    effect: none
```

#### Scaling

<!-- splat:values path=splat_rule/parameters/scaling -->

| Field | Value |
| --- | --- |
| ` per_slot_above ` | ` 1 ` |
| ` cold_damage_dice ` | ` 1 ` |
| ` sides ` | ` 6 ` |

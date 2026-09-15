---
splat_schema: 1
splat_id: 43c6436f-6610-5460-925c-0479374081d0
splat_kind: spell
splat_key: dnd2024:spells/acid_arrow
splat_name: Acid Arrow
splat_origin:
  key: dnd2024:reference/spell/acid_arrow
splat_sources:
- title: SRD 5.2.1
  locator: pp. 107
  url: https://www.dndbeyond.com/srd
---

# Acid Arrow

## Source text

<!-- SRD page 107 -->

Level 2 Evocation (Wizard)\
Casting Time: Action\
Range: 90 feet\
Components: V, S, M (powdered rhubarb leaf)\
Duration: Instantaneous\
A shimmering green arrow streaks toward a target\
within range and bursts in a spray of acid. Make a\
ranged spell attack against the target. On a hit, the\
target takes 4d4 Acid damage and 2d4 Acid dam-\
age at the end of its next turn. On a miss, the arrow\
splashes the target with acid for half as much of the\
initial damage only.\
Using a Higher-Level Spell Slot. The damage\
(both initial and later) increases by 1d4 for each\
spell slot level above 2.\

Original layout: [[../../srd-5.2.1/pages/107|page 107]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 2 ` |
| ` school ` | ` evocation ` |
| ` classes ` | ` ["wizard"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` Instantaneous ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` feet ` | ` 90 ` |

### Material

<!-- splat:values path=splat_spell/material -->

| Field | Value |
| --- | --- |
| ` description ` | ` powdered rhubarb leaf ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/acid_arrow ` |
| ` implementation ` | ` dnd2024:spell_acid_arrow ` |

### Parameters

#### Target

<!-- splat:values path=splat_rule/parameters/target -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature_or_object ` |
| ` count ` | ` 1 ` |
| ` visible_to_caster ` | ` false ` |

#### Attack

<!-- splat:yaml path=splat_rule/parameters/attack -->

```yaml
kind: ranged_spell
hit:
  initial:
    amount:
      dice:
        count: 4
        sides: 4
        add: 0
    type: acid
  later:
    event: turn.end
    subject: target
    turn: next
    damage:
      amount:
        dice:
          count: 2
          sides: 4
          add: 0
      type: acid
miss:
  damage:
    amount:
      dice:
        count: 4
        sides: 4
        add: 0
    type: acid
  scale:
    numerator: 1
    denominator: 2
  round: down
  later_damage: false
```

#### Scaling

<!-- splat:values path=splat_rule/parameters/scaling -->

| Field | Value |
| --- | --- |
| ` per_slot_above ` | ` 2 ` |
| ` initial_dice ` | ` 1 ` |
| ` later_dice ` | ` 1 ` |
| ` sides ` | ` 4 ` |

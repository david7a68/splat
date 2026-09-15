---
splat_schema: 1
splat_id: cdf3eb74-6d96-578f-88ad-9aa3b89bcd14
splat_kind: spell
splat_key: dnd2024:spells/guiding_bolt
splat_name: Guiding Bolt
splat_origin:
  key: dnd2024:reference/spell/guiding_bolt
splat_sources:
- title: SRD 5.2.1
  locator: pp. 138
  url: https://www.dndbeyond.com/srd
---

# Guiding Bolt

## Source text

<!-- SRD page 138 -->

Level 1 Evocation (Cleric)\
Casting Time: Action\
Range: 120 feet\
Components: V, S\
Duration: 1 round\
You hurl a bolt of light toward a creature within\
range. Make a ranged spell attack against the tar-\
get. On a hit, it takes 4d6 Radiant damage, and the\
next attack roll made against it before the end of\
your next turn has Advantage.\
Using a Higher-Level Spell Slot. The damage in-\
creases by 1d6 for each spell slot level above 1.\

Original layout: [[../../srd-5.2.1/pages/138|page 138]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 1 ` |
| ` school ` | ` evocation ` |
| ` classes ` | ` ["cleric"] ` |
| ` components ` | ` ["verbal", "somatic"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` 1 round ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` feet ` | ` 120 ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/guiding_bolt ` |
| ` implementation ` | ` dnd2024:spell_guiding_bolt ` |

### Parameters

#### Targeting

<!-- splat:values path=splat_rule/parameters/targeting -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` count ` | ` 1 ` |
| ` within_range ` | ` true ` |

#### Attack

<!-- splat:yaml path=splat_rule/parameters/attack -->

```yaml
kind: ranged_spell
hit:
  damage:
    dice: 4d6
    type: radiant
  rider:
    next_attack_against_target: advantage
    deadline: caster_next_turn_end
    consumes_on: next_attack_roll
```

#### Slot scaling

<!-- splat:values path=splat_rule/parameters/slot_scaling -->

| Field | Value |
| --- | --- |
| ` above ` | ` 1 ` |
| ` damage_per_level ` | ` 1d6 ` |

#### Spell lifetime

<!-- splat:yaml path=splat_rule/parameters/spell_lifetime -->

```yaml
concentration: false
maximum:
  value: 1
  unit: round
```

---
splat_schema: 1
splat_id: 2f42c7a2-36ab-5b1d-9a9d-968c579a6c4c
splat_kind: spell
splat_key: dnd2024:spells/shocking_grasp
splat_name: Shocking Grasp
splat_origin:
  key: dnd2024:reference/spell/shocking_grasp
splat_sources:
- title: SRD 5.2.1
  locator: pp. 162
  url: https://www.dndbeyond.com/srd
---

# Shocking Grasp

## Source text

<!-- SRD page 162 -->

Evocation Cantrip (Sorcerer, Wizard)\
Casting Time: Action\
Range: Touch\
Components: V, S\
Duration: Instantaneous\
Lightning springs from you to a creature that you\
try to touch. Make a melee spell attack against the\
target. On a hit, the target takes 1d8 Lightning dam-\
age, and it can’t make Opportunity Attacks until the\
start of its next turn.\
Cantrip Upgrade. The damage increases by 1d8\
when you reach levels 5 (2d8), 11 (3d8), and 17\
(4d8).\

Original layout: [[../../srd-5.2.1/pages/162|page 162]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 0 ` |
| ` school ` | ` evocation ` |
| ` classes ` | ` ["sorcerer", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` Instantaneous ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` mode ` | ` touch ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/shocking_grasp ` |
| ` implementation ` | ` dnd2024:spell_shocking_grasp ` |

### Parameters

#### Targets

<!-- splat:values path=splat_rule/parameters/targets -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` maximum ` | ` 1 ` |
| ` within_spell_range ` | ` true ` |

#### Attack

<!-- splat:yaml path=splat_rule/parameters/attack -->

```yaml
kind: melee_spell
hit:
  damage:
    dice:
      count:
        by_caster_level:
        - minimum: 1
          count: 1
        - minimum: 5
          count: 2
        - minimum: 11
          count: 3
        - minimum: 17
          count: 4
      sides: 8
      add: 0
    type: lightning
  prohibit: opportunity_attacks
  until: target_next_turn_start
```

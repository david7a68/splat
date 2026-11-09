---
splat_schema: 2
splat_id: 4cb693a0-2527-531e-952e-cbbbc19e9261
splat_kind: spell
splat_key: dnd2024:spells/phantasmal_killer
splat_name: Phantasmal Killer
splat_origin: dnd2024:reference/spell/phantasmal_killer
splat_sources:
- SRD 5.2.1, pp. 152, https://www.dndbeyond.com/srd
---

# Phantasmal Killer


## Spell

| Field | Value |
| --- | --- |
| Level | 4 |
| School | Illusion |
| Classes | Bard, Wizard |
| Casting Time | Action |
| Range | 120 feet |
| Components | V, S |
| Duration | Concentration, up to 1 minute |
| Ritual | No |
| Concentration | Yes |


## Source text

<!-- SRD page 152 -->

Level 4 Illusion (Bard, Wizard)\
Casting Time: Action\
Range: 120 feet\
Components: V, S\
Duration: Concentration, up to 1 minute

You tap into the nightmares of a creature you can see within range and create an illusion of its deepest fears, visible only to that creature. The target makes a Wisdom saving throw. On a failed save, the target takes 4d10 Psychic damage and has Disadvantage on ability checks and attack rolls for the duration. On a successful save, the target takes half as much damage, and the spell ends.

For the duration, the target makes a Wisdom saving throw at the end of each of its turns. On a failed save, it takes the Psychic damage again. On a successful save, the spell ends.

Using a Higher-Level Spell Slot. The damage increases by 1d10 for each spell slot level above 4.

Original layout: [[../../srd-5.2.1/pages/152|page 152]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/phantasmal_killer ` |
| ` implementation ` | ` dnd2024:spell_phantasmal_killer ` |

### Parameters

#### Targets

<!-- splat:values path=splat_rule/parameters/targets -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` maximum ` | ` 1 ` |
| ` within_spell_range ` | ` true ` |
| ` visible ` | ` true ` |

#### Illusion

<!-- splat:values path=splat_rule/parameters/illusion -->

| Field | Value |
| --- | --- |
| ` depiction ` | ` target_deepest_fears ` |
| ` visible_only_to ` | ` target ` |

#### Initial save

<!-- splat:yaml path=splat_rule/parameters/initial_save -->

```yaml
ability: wisdom
dc:
  binding: save_dc
failure:
  damage:
    dice:
      count:
        op: add
        args:
        - 4
        - op: multiply
          args:
          - 1
          - op: subtract
            args:
            - binding: cast_level
            - 4
      sides: 10
      add: 0
    type: psychic
  disadvantage:
  - ability_checks
  - attack_rolls
  duration: spell_duration
success:
  damage:
    dice:
      count:
        op: add
        args:
        - 4
        - op: multiply
          args:
          - 1
          - op: subtract
            args:
            - binding: cast_level
            - 4
      sides: 10
      add: 0
    type: psychic
  damage_fraction:
    numerator: 1
    denominator: 2
  end_spell: true
```

#### Repeat save

<!-- splat:yaml path=splat_rule/parameters/repeat_save -->

```yaml
event: target_turn_end
save:
  ability: wisdom
  dc:
    binding: save_dc
  failure:
    damage:
      dice:
        count:
          op: add
          args:
          - 4
          - op: multiply
            args:
            - 1
            - op: subtract
              args:
              - binding: cast_level
              - 4
        sides: 10
        add: 0
      type: psychic
  success:
    end_spell: true
```

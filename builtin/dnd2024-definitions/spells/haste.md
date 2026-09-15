---
splat_schema: 1
splat_id: 4402731c-08cc-5dd5-aed6-8f409703fdcc
splat_kind: spell
splat_key: dnd2024:spells/haste
splat_name: Haste
splat_origin:
  key: dnd2024:reference/spell/haste
splat_sources:
- title: SRD 5.2.1
  locator: pp. 139
  url: https://www.dndbeyond.com/srd
---

# Haste

## Source text

<!-- SRD page 139 -->

Level 3 Transmutation (Sorcerer, Wizard)\
Casting Time: Action\
Range: 30 feet\
Components: V, S, M (a shaving of licorice root)\
Duration: Concentration, up to 1 minute\
Choose a willing creature that you can see within\
range. Until the spell ends, the target’s Speed is\
doubled, it gains a +2 bonus to Armor Class, it has\
Advantage on Dexterity saving throws, and it gains\
an additional action on each of its turns. That ac-\
tion can be used to take only the Attack (one attack\
only), Dash, Disengage, Hide, or Utilize action.\
When the spell ends, the target is Incapacitated\
and has a Speed of 0 until the end of its next turn, as\
a wave of lethargy washes over it.\

Original layout: [[../../srd-5.2.1/pages/139|page 139]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 3 ` |
| ` school ` | ` transmutation ` |
| ` classes ` | ` ["sorcerer", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` true ` |
| ` duration ` | ` Concentration, up to 1 minute ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` feet ` | ` 30 ` |

### Material

<!-- splat:values path=splat_spell/material -->

| Field | Value |
| --- | --- |
| ` description ` | ` a shaving of licorice root ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/haste ` |
| ` implementation ` | ` dnd2024:spell_haste ` |

### Parameters

#### Targets

<!-- splat:values path=splat_rule/parameters/targets -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` maximum ` | ` 1 ` |
| ` within_spell_range ` | ` true ` |
| ` visible ` | ` true ` |
| ` willing ` | ` true ` |

#### Benefits

<!-- splat:yaml path=splat_rule/parameters/benefits -->

```yaml
duration: spell_duration
speed_multiplier: 2
ac_bonus: 2
saving_throw_advantage:
- dexterity
additional_action_each_target_turn:
  count: 1
  allowed:
  - attack
  - dash
  - disengage
  - hide
  - utilize
  attack_count_maximum: 1
```

#### On end

<!-- splat:values path=splat_rule/parameters/on_end -->

| Field | Value |
| --- | --- |
| ` condition ` | ` incapacitated ` |
| ` speed ` | ` 0 ` |
| ` until ` | ` target_next_turn_end ` |

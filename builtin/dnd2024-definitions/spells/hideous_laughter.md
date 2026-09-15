---
splat_schema: 1
splat_id: 0b5651a1-6a64-5e9f-89f9-f603fd79b44a
splat_kind: spell
splat_key: dnd2024:spells/hideous_laughter
splat_name: Hideous Laughter
splat_origin:
  key: dnd2024:reference/spell/hideous_laughter
splat_sources:
- title: SRD 5.2.1
  locator: pp. 140
  url: https://www.dndbeyond.com/srd
---

# Hideous Laughter

## Source text

<!-- SRD page 140 -->

Level 1 Enchantment (Bard, Warlock, Wizard)\
Casting Time: Action\
Range: 30 feet\
Components: V, S, M (a tart and a feather)\
Duration: Concentration, up to 1 minute\
One creature of your choice that you can see within\
range makes a Wisdom saving throw. On a failed\
save, it has the Prone and Incapacitated conditions\
for the duration. During that time, it laughs uncon-\
trollably if it’s capable of laughter, and it can’t end\
the Prone condition on itself.\
At the end of each of its turns and each time it\
takes damage, it makes another Wisdom saving\
throw. The target has Advantage on the save if the\
save is triggered by damage. On a successful save,\
the spell ends.\
Using a Higher-Level Spell Slot. You can target\
one additional creature for each spell slot level\
above 1.\

Original layout: [[../../srd-5.2.1/pages/140|page 140]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 1 ` |
| ` school ` | ` enchantment ` |
| ` classes ` | ` ["bard", "warlock", "wizard"] ` |
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
| ` description ` | ` a tart and a feather ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/hideous_laughter ` |
| ` implementation ` | ` dnd2024:spell_hideous_laughter ` |

### Parameters

#### Targets

<!-- splat:yaml path=splat_rule/parameters/targets -->

```yaml
kind: creature
maximum:
  op: add
  args:
  - 1
  - op: multiply
    args:
    - 1
    - op: subtract
      args:
      - binding: cast_level
      - 1
within_spell_range: true
visible: true
```

#### Save

<!-- splat:yaml path=splat_rule/parameters/save -->

```yaml
ability: wisdom
dc:
  binding: save_dc
failure:
  conditions:
  - prone
  - incapacitated
  duration: spell_duration
  cannot_end_own_prone: true
  laugh_uncontrollably_if_capable: true
success:
  effect: none
```

#### Repeat save

<!-- splat:values path=splat_rule/parameters/repeat_save -->

| Field | Value |
| --- | --- |
| ` ability ` | ` wisdom ` |
| ` events ` | ` ["target_turn_end", "target_takes_damage"] ` |
| ` advantage_if ` | ` triggered_by_damage ` |
| ` success ` | ` end_spell ` |

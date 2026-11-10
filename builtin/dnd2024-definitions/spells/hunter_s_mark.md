---
splat_schema: 2
splat_id: 9969e83a-34fe-5bd6-bdc1-f9f4e29bb5a0
splat_kind: spell
splat_key: dnd2024:spells/hunter_s_mark
splat_name: Hunter’s Mark
splat_origin: dnd2024:reference/spell/hunter_s_mark
splat_sources:
- SRD 5.2.1, pp. 141, https://www.dndbeyond.com/srd
---

# Hunter’s Mark


## Spell

| Field | Value |
| --- | --- |
| Level | 1 |
| School | Divination |
| Classes | Ranger |
| Casting Time | Bonus Action |
| Range | 90 feet |
| Components | V |
| Duration | Concentration, up to 1 hour |
| Ritual | No |
| Concentration | Yes |


## Source text

<!-- SRD page 141 -->

Level 1 Divination (Ranger)\
Casting Time: Bonus Action\
Range: 90 feet\
Components: V\
Duration: Concentration, up to 1 hour

You magically mark one creature you can see within range as your quarry. Until the spell ends, you deal an extra 1d6 Force damage to the target whenever you hit it with an attack roll. You also have Advantage on any Wisdom (Perception or Survival) check you make to find it.

If the target drops to 0 Hit Points before this spell ends, you can take a Bonus Action to move the mark to a new creature you can see within range.

Using a Higher-Level Spell Slot. Your Concentration can last longer with a spell slot of level 3–4 (up to 8 hours) or 5+ (up to 24 hours).

Original layout: [[../../srd-5.2.1/pages/141|page 141]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/hunter_s_mark ` |
| ` implementation ` | ` dnd2024:spell_hunter_s_mark ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` concentration ` | ` true ` |

#### Targets

<!-- splat:values path=splat_rule/parameters/targets -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` maximum ` | ` 1 ` |
| ` within_spell_range ` | ` true ` |
| ` visible ` | ` true ` |

#### Mark

<!-- splat:yaml path=splat_rule/parameters/mark -->

```yaml
duration: spell_duration
extra_damage:
  trigger: caster_hits_target_with_attack_roll
  damage:
    dice:
      count: 1
      sides: 6
      add: 0
    type: force
advantage:
  checks_ability: wisdom
  skills:
  - perception
  - survival
  purpose: find_marked_target
```

#### Transfer

<!-- splat:yaml path=splat_rule/parameters/transfer -->

```yaml
requires: marked_target_dropped_to_zero_hp_before_spell_end
cost: bonus_action
new_target:
  kind: creature
  maximum: 1
  within_spell_range: true
  visible: true
```

#### Duration by slot

<!-- splat:yaml path=splat_rule/parameters/duration_by_slot -->

```yaml
'1_2':
  hours: 1
'3_4':
  hours: 8
5_plus:
  hours: 24
```

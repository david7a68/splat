---
splat_schema: 2
splat_id: e8e788a3-9823-5375-99ad-87e814401cce
splat_kind: spell
splat_key: dnd2024:spells/hellish_rebuke
splat_name: Hellish Rebuke
splat_origin: dnd2024:reference/spell/hellish_rebuke
splat_sources:
- SRD 5.2.1, pp. 140, https://www.dndbeyond.com/srd
---

# Hellish Rebuke


## Spell

| Field | Value |
| --- | --- |
| Level | 1 |
| School | Evocation |
| Classes | Warlock |
| Casting Time | Reaction, which you take in response to taking damage from a creature that you can see within 60 feet of yourself |
| Range | 60 feet |
| Components | V, S |
| Duration | Instantaneous |
| Ritual | No |
| Concentration | No |


## Source text

<!-- SRD page 140 -->

Level 1 Evocation (Warlock)\
Casting Time: Reaction, which you take in response to taking damage from a creature that you can see within 60 feet of yourself\
Range: 60 feet\
Components: V, S\
Duration: Instantaneous

The creature that damaged you is momentarily surrounded by green flames. It makes a Dexterity saving throw, taking 2d10 Fire damage on a failed save or half as much damage on a successful one.

Using a Higher-Level Spell Slot. The damage increases by 1d10 for each spell slot level above 1.

Original layout: [[../../srd-5.2.1/pages/140|page 140]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/hellish_rebuke ` |
| ` implementation ` | ` dnd2024:spell_hellish_rebuke ` |

### Parameters

#### Targets

<!-- splat:values path=splat_rule/parameters/targets -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` identity ` | ` triggering_damage_source ` |
| ` visible ` | ` true ` |
| ` maximum_distance_feet ` | ` 60 ` |

#### Save

<!-- splat:yaml path=splat_rule/parameters/save -->

```yaml
ability: dexterity
dc:
  binding: save_dc
failure:
  damage:
    dice:
      count: &id001
        op: add
        args:
        - 2
        - op: multiply
          args:
          - 1
          - op: subtract
            args:
            - binding: cast_level
            - 1
      sides: 10
      add: 0
    type: fire
success:
  damage:
    dice:
      count: *id001
      sides: 10
      add: 0
    type: fire
  damage_fraction:
    numerator: 1
    denominator: 2
```

---
splat_schema: 1
splat_id: e8e788a3-9823-5375-99ad-87e814401cce
splat_kind: spell
splat_key: dnd2024:spells/hellish_rebuke
splat_name: Hellish Rebuke
splat_origin:
  key: dnd2024:reference/spell/hellish_rebuke
splat_sources:
- title: SRD 5.2.1
  locator: pp. 140
  url: https://www.dndbeyond.com/srd
---

# Hellish Rebuke

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

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 1 ` |
| ` school ` | ` evocation ` |
| ` classes ` | ` ["warlock"] ` |
| ` components ` | ` ["verbal", "somatic"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` Instantaneous ` |
| ` casting_time ` | ` Reaction, which you take in response to taking damage from a creature that you can see within 60 feet of yourself ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` feet ` | ` 60 ` |

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

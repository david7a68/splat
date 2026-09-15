---
splat_schema: 1
splat_id: 0a96f42b-3b27-57e4-a112-4553a424129b
splat_kind: spell
splat_key: dnd2024:spells/scorching_ray
splat_name: Scorching Ray
splat_origin:
  key: dnd2024:reference/spell/scorching_ray
splat_sources:
- title: SRD 5.2.1
  locator: pp. 159
  url: https://www.dndbeyond.com/srd
---

# Scorching Ray

## Source text

<!-- SRD page 159 -->

Level 2 Evocation (Sorcerer, Wizard)\
Casting Time: Action\
Range: 120 feet\
Components: V, S\
Duration: Instantaneous\
You hurl three fiery rays. You can hurl them at one\
target within range or at several. Make a ranged\
spell attack for each ray. On a hit, the target takes\
2d6 Fire damage.\
Using a Higher-Level Spell Slot. You create one\
additional ray for each spell slot level above 2.\

Original layout: [[../../srd-5.2.1/pages/159|page 159]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 2 ` |
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
| ` feet ` | ` 120 ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/scorching_ray ` |
| ` implementation ` | ` dnd2024:spell_scorching_ray ` |

### Parameters

#### Rays

<!-- splat:yaml path=splat_rule/parameters/rays -->

```yaml
base_count: 3
target_assignment: same_or_different_targets_in_range
attack_per_ray: ranged_spell
hit:
  damage:
    amount:
      dice:
        count: 2
        sides: 6
        add: 0
    type: fire
```

#### Scaling

<!-- splat:values path=splat_rule/parameters/scaling -->

| Field | Value |
| --- | --- |
| ` per_slot_above ` | ` 2 ` |
| ` additional_rays ` | ` 1 ` |

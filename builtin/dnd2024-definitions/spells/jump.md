---
splat_schema: 1
splat_id: d562d78e-a442-533a-af52-5725303f1251
splat_kind: spell
splat_key: dnd2024:spells/jump
splat_name: Jump
splat_origin:
  key: dnd2024:reference/spell/jump
splat_sources:
- title: SRD 5.2.1
  locator: pp. 143
  url: https://www.dndbeyond.com/srd
---

# Jump

## Source text

<!-- SRD page 143 -->

Level 1 Transmutation (Druid, Ranger, Sorcerer, Wizard)\
Casting Time: Bonus Action\
Range: Touch\
Component: V, S, M (a grasshopper’s hind leg)\
Duration: 1 minute

You touch a willing creature. Once on each of its turns until the spell ends, that creature can jump up to 30 feet by spending 10 feet of movement.

Using a Higher-Level Spell Slot. You can target one additional creature for each spell slot level above 1.

Original layout: [[../../srd-5.2.1/pages/143|page 143]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 1 ` |
| ` school ` | ` transmutation ` |
| ` classes ` | ` ["druid", "ranger", "sorcerer", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` 1 minute ` |
| ` casting_time ` | ` Bonus Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` mode ` | ` touch ` |

### Material

<!-- splat:values path=splat_spell/material -->

| Field | Value |
| --- | --- |
| ` description ` | ` a grasshopper’s hind leg ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/jump ` |
| ` implementation ` | ` dnd2024:spell_jump ` |

### Parameters

#### Spell lifetime

<!-- splat:values path=splat_rule/parameters/spell_lifetime -->

| Field | Value |
| --- | --- |
| ` concentration ` | ` false ` |
| ` maximum_seconds ` | ` 60 ` |

#### Targets

<!-- splat:yaml path=splat_rule/parameters/targets -->

```yaml
kind: creature
willing: true
touch: true
count:
  base: 1
  increment_per_slot_above: 1
  base_slot: 1
```

#### Jump grant

<!-- splat:values path=splat_rule/parameters/jump_grant -->

| Field | Value |
| --- | --- |
| ` timing ` | ` target_turn ` |
| ` count_per_turn ` | ` 1 ` |
| ` maximum_distance_feet ` | ` 30 ` |
| ` movement_cost_feet ` | ` 10 ` |
| ` optional ` | ` true ` |

#### Spatial resolution

<!-- splat:values path=splat_rule/parameters/spatial_resolution -->

| Field | Value |
| --- | --- |
| ` source ` | ` supplied_world_geometry ` |
| ` unknown_facts ` | ` require_resolution ` |
| ` infer_unprovided_geometry ` | ` false ` |

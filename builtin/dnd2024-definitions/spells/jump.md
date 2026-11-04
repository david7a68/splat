---
splat_schema: 2
splat_id: d562d78e-a442-533a-af52-5725303f1251
splat_kind: spell
splat_key: dnd2024:spells/jump
splat_name: Jump
splat_origin: dnd2024:reference/spell/jump
splat_sources:
- SRD 5.2.1, pp. 143, https://www.dndbeyond.com/srd
---

# Jump


## Spell

| Field | Value |
| --- | --- |
| Level | 1 |
| School | Transmutation |
| Classes | Druid, Ranger, Sorcerer, Wizard |
| Casting Time | Bonus Action |
| Range |  |
| Components | V, S, M (a grasshopper’s hind leg) |
| Duration | 1 minute |


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

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> material:
>   consumed: false
> range:
>   mode: touch
> ```


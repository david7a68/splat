---
splat_schema: 1
splat_id: c8f436ab-e351-503b-a543-fafe5e134c0a
splat_kind: spell
splat_key: dnd2024:spells/fly
splat_name: Fly
splat_origin:
  key: dnd2024:reference/spell/fly
splat_sources:
- title: SRD 5.2.1
  locator: pp. 133
  url: https://www.dndbeyond.com/srd
---

# Fly

## Source text

<!-- SRD page 133 -->

Level 3 Transmutation (Sorcerer, Warlock, Wizard)\
Casting Time: Action\
Range: Touch\
Components: V, S, M (a feather)\
Duration: Concentration, up to 10 minutes

You touch a willing creature. For the duration, the target gains a Fly Speed of 60 feet and can hover. When the spell ends, the target falls if it is still aloft unless it can stop the fall.

Using a Higher-Level Spell Slot. You can target one additional creature for each spell slot level above 3.

Original layout: [[../../srd-5.2.1/pages/133|page 133]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 3 ` |
| ` school ` | ` transmutation ` |
| ` classes ` | ` ["sorcerer", "warlock", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` true ` |
| ` duration ` | ` Concentration, up to 10 minutes ` |
| ` casting_time ` | ` Action ` |
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
| ` description ` | ` a feather ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/fly ` |
| ` implementation ` | ` dnd2024:spell_fly ` |

### Parameters

#### Spell lifetime

<!-- splat:values path=splat_rule/parameters/spell_lifetime -->

| Field | Value |
| --- | --- |
| ` concentration ` | ` true ` |
| ` maximum_seconds ` | ` 600 ` |

#### Targets

<!-- splat:yaml path=splat_rule/parameters/targets -->

```yaml
kind: creature
willing: true
touch: true
count:
  base: 1
  increment_per_slot_above: 1
  base_slot: 3
```

#### Grant

<!-- splat:values path=splat_rule/parameters/grant -->

| Field | Value |
| --- | --- |
| ` fly_speed_feet ` | ` 60 ` |
| ` hover ` | ` true ` |

#### On end

<!-- splat:values path=splat_rule/parameters/on_end -->

| Field | Value |
| --- | --- |
| ` if ` | ` target_aloft ` |
| ` effect ` | ` fall ` |
| ` exception ` | ` target_can_stop_fall ` |

#### Spatial resolution

<!-- splat:values path=splat_rule/parameters/spatial_resolution -->

| Field | Value |
| --- | --- |
| ` source ` | ` supplied_world_geometry ` |
| ` unknown_facts ` | ` require_resolution ` |
| ` infer_unprovided_geometry ` | ` false ` |

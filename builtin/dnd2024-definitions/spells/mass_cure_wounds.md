---
splat_schema: 1
splat_id: 65399dfc-e071-52d4-8a0d-953570dfa950
splat_kind: spell
splat_key: dnd2024:spells/mass_cure_wounds
splat_name: Mass Cure Wounds
splat_origin:
  key: dnd2024:reference/spell/mass_cure_wounds
splat_sources:
- title: SRD 5.2.1
  locator: pp. 147
  url: https://www.dndbeyond.com/srd
---

# Mass Cure Wounds

## Source text

<!-- SRD page 147 -->

Level 5 Abjuration (Bard, Cleric, Druid)\
Casting Time: Action\
Range: 60 feet\
Components: V, S\
Duration: Instantaneous

A wave of healing energy washes out from a point you can see within range. Choose up to six creatures in a 30-foot-radius Sphere centered on that point. Each target regains Hit Points equal to 5d8 plus your spellcasting ability modifier.

Using a Higher-Level Spell Slot. The healing increases by 1d8 for each spell slot level above 5.

Original layout: [[../../srd-5.2.1/pages/147|page 147]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 5 ` |
| ` school ` | ` abjuration ` |
| ` classes ` | ` ["bard", "cleric", "druid"] ` |
| ` components ` | ` ["verbal", "somatic"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` Instantaneous ` |
| ` casting_time ` | ` Action ` |
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
| ` mechanic ` | ` dnd2024:spells/mass_cure_wounds ` |
| ` implementation ` | ` dnd2024:spell_mass_cure_wounds ` |

### Parameters

#### Spell lifetime

<!-- splat:values path=splat_rule/parameters/spell_lifetime -->

| Field | Value |
| --- | --- |
| ` concentration ` | ` false ` |
| ` instantaneous ` | ` true ` |

#### Area

<!-- splat:values path=splat_rule/parameters/area -->

| Field | Value |
| --- | --- |
| ` shape ` | ` sphere ` |
| ` radius_feet ` | ` 30 ` |
| ` origin ` | ` chosen_point ` |
| ` point_visible ` | ` true ` |
| ` range_feet ` | ` 60 ` |

#### Targets

<!-- splat:values path=splat_rule/parameters/targets -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` in_area ` | ` true ` |
| ` chooser ` | ` caster ` |
| ` maximum_count ` | ` 6 ` |

#### Healing

<!-- splat:yaml path=splat_rule/parameters/healing -->

```yaml
dice:
  base_count: 5
  sides: 8
  extra_per_slot: 1
  base_slot: 5
add:
  binding: spellcasting_ability_modifier
```

#### Spatial resolution

<!-- splat:values path=splat_rule/parameters/spatial_resolution -->

| Field | Value |
| --- | --- |
| ` source ` | ` supplied_world_geometry ` |
| ` unknown_facts ` | ` require_resolution ` |
| ` infer_unprovided_geometry ` | ` false ` |

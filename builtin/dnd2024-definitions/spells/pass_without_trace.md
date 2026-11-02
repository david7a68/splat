---
splat_schema: 1
splat_id: 463d14d1-fd72-5ea7-9eb9-51797f64ecee
splat_kind: spell
splat_key: dnd2024:spells/pass_without_trace
splat_name: Pass without Trace
splat_origin:
  key: dnd2024:reference/spell/pass_without_trace
splat_sources:
- title: SRD 5.2.1
  locator: pp. 151
  url: https://www.dndbeyond.com/srd
---

# Pass without Trace

## Source text

<!-- SRD page 151 -->

Level 2 Abjuration (Druid, Ranger)\
Casting Time: Action\
Range: Self\
Components: V, S, M (ashes from burned mistletoe)\
Duration: Concentration, up to 1 hour

You radiate a concealing aura in a 30-foot Emanation for the duration. While in the aura, you and each creature you choose have a +10 bonus to Dexterity (Stealth) checks and leave no tracks.

Original layout: [[../../srd-5.2.1/pages/151|page 151]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 2 ` |
| ` school ` | ` abjuration ` |
| ` classes ` | ` ["druid", "ranger"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` true ` |
| ` duration ` | ` Concentration, up to 1 hour ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` mode ` | ` self ` |

### Material

<!-- splat:values path=splat_spell/material -->

| Field | Value |
| --- | --- |
| ` description ` | ` ashes from burned mistletoe ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/pass_without_trace ` |
| ` implementation ` | ` dnd2024:spell_pass_without_trace ` |

### Parameters

#### Spell lifetime

<!-- splat:values path=splat_rule/parameters/spell_lifetime -->

| Field | Value |
| --- | --- |
| ` concentration ` | ` true ` |
| ` maximum_seconds ` | ` 3600 ` |

#### Area

<!-- splat:values path=splat_rule/parameters/area -->

| Field | Value |
| --- | --- |
| ` shape ` | ` emanation ` |
| ` radius_feet ` | ` 30 ` |
| ` origin ` | ` caster ` |
| ` moves_with_origin ` | ` true ` |

#### Beneficiaries

<!-- splat:values path=splat_rule/parameters/beneficiaries -->

| Field | Value |
| --- | --- |
| ` caster ` | ` true ` |
| ` additional_creatures ` | ` caster_choice ` |
| ` requires ` | ` in_aura ` |

#### Benefits

<!-- splat:values path=splat_rule/parameters/benefits -->

| Field | Value |
| --- | --- |
| ` dexterity_stealth_check_bonus ` | ` 10 ` |
| ` leave_tracks ` | ` false ` |

#### Spatial resolution

<!-- splat:values path=splat_rule/parameters/spatial_resolution -->

| Field | Value |
| --- | --- |
| ` source ` | ` supplied_world_geometry ` |
| ` unknown_facts ` | ` require_resolution ` |
| ` infer_unprovided_geometry ` | ` false ` |

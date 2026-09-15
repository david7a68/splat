---
splat_schema: 1
splat_id: b5d274e6-266a-512f-be8e-9db637d17892
splat_kind: spell
splat_key: dnd2024:spells/conjure_minor_elementals
splat_name: Conjure Minor Elementals
splat_origin:
  key: dnd2024:reference/spell/conjure_minor_elementals
splat_sources:
- title: SRD 5.2.1
  locator: pp. 118
  url: https://www.dndbeyond.com/srd
---

# Conjure Minor Elementals

## Source text

<!-- SRD page 118 -->

Level 4 Conjuration (Druid, Wizard)\
Casting Time: Action\
Range: Self\
Components: V, S\
Duration: Concentration, up to 10 minutes\
You conjure spirits from the Elemental Planes that\
flit around you in a 15-foot Emanation for the dura-\
tion. Until the spell ends, any attack you make deals\
an extra 2d8 damage when you hit a creature in the\
Emanation. This damage is Acid, Cold, Fire, or Light-\
ning (your choice when you make the attack).\
In addition, the ground in the Emanation is Diffi-\
cult Terrain for your enemies.\
Using a Higher-Level Spell Slot. The damage in-\
creases by 1d8 for each spell slot level above 4.\

Original layout: [[../../srd-5.2.1/pages/118|page 118]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 4 ` |
| ` school ` | ` conjuration ` |
| ` classes ` | ` ["druid", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic"] ` |
| ` concentration ` | ` true ` |
| ` duration ` | ` Concentration, up to 10 minutes ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` mode ` | ` self ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/conjure_minor_elementals ` |
| ` implementation ` | ` dnd2024:spell_conjure_minor_elementals ` |

### Parameters

#### Spell lifetime

<!-- splat:values path=splat_rule/parameters/spell_lifetime -->

| Field | Value |
| --- | --- |
| ` concentration ` | ` true ` |
| ` maximum_seconds ` | ` 600 ` |

#### Area

<!-- splat:values path=splat_rule/parameters/area -->

| Field | Value |
| --- | --- |
| ` shape ` | ` emanation ` |
| ` radius_feet ` | ` 15 ` |
| ` origin ` | ` caster ` |
| ` moves_with_origin ` | ` true ` |

#### Damage trigger

<!-- splat:values path=splat_rule/parameters/damage_trigger -->

| Field | Value |
| --- | --- |
| ` event ` | ` attack.hit ` |
| ` attacker ` | ` caster ` |
| ` target_kind ` | ` creature ` |
| ` target_in_area ` | ` true ` |

#### Extra damage

<!-- splat:yaml path=splat_rule/parameters/extra_damage -->

```yaml
dice:
  base_count: 2
  sides: 8
  extra_per_slot: 1
  base_slot: 4
type:
  chooser: caster
  timing: attack_made
  options:
  - acid
  - cold
  - fire
  - lightning
```

#### Terrain

<!-- splat:values path=splat_rule/parameters/terrain -->

| Field | Value |
| --- | --- |
| ` scope ` | ` ground_in_area ` |
| ` affected ` | ` caster_enemies ` |
| ` difficult ` | ` true ` |

#### Spatial resolution

<!-- splat:values path=splat_rule/parameters/spatial_resolution -->

| Field | Value |
| --- | --- |
| ` source ` | ` supplied_world_geometry ` |
| ` unknown_facts ` | ` require_resolution ` |
| ` infer_unprovided_geometry ` | ` false ` |

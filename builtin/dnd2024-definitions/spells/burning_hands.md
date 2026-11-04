---
splat_schema: 2
splat_id: 1ded0ab1-4ede-507e-b5f4-833d13a0d3fc
splat_kind: spell
splat_key: dnd2024:spells/burning_hands
splat_name: Burning Hands
splat_origin: dnd2024:reference/spell/burning_hands
splat_sources:
- SRD 5.2.1, pp. 114, https://www.dndbeyond.com/srd
---

# Burning Hands


## Spell

| Field | Value |
| --- | --- |
| Level | 1 |
| School | Evocation |
| Classes | Sorcerer, Wizard |
| Casting Time | Action |
| Range |  |
| Components | V, S |
| Duration | Instantaneous |


## Source text

<!-- SRD page 114 -->

Level 1 Evocation (Sorcerer, Wizard)\
Casting Time: Action\
Range: Self\
Components: V, S\
Duration: Instantaneous

A thin sheet of flames shoots forth from you. Each creature in a 15-foot Cone makes a Dexterity saving throw, taking 3d6 Fire damage on a failed save or half as much damage on a successful one.

Flammable objects in the Cone that aren’t being worn or carried start burning.

Using a Higher-Level Spell Slot. The damage increases by 1d6 for each spell slot level above 1.

Original layout: [[../../srd-5.2.1/pages/114|page 114]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/burning_hands ` |
| ` implementation ` | ` dnd2024:spell_burning_hands ` |

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
| ` shape ` | ` cone ` |
| ` length_feet ` | ` 15 ` |
| ` origin ` | ` caster ` |

#### Creatures

<!-- splat:yaml path=splat_rule/parameters/creatures -->

```yaml
save:
  ability: dexterity
  dc: spell_save_dc
damage:
  dice:
    base_count: 3
    sides: 6
    extra_per_slot: 1
    base_slot: 1
  type: fire
success_damage_scale:
  numerator: 1
  denominator: 2
```

#### Objects

<!-- splat:values path=splat_rule/parameters/objects -->

| Field | Value |
| --- | --- |
| ` flammable ` | ` true ` |
| ` worn ` | ` false ` |
| ` carried ` | ` false ` |
| ` effect ` | ` start_burning ` |

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
> range:
>   mode: self
> ```


---
splat_schema: 2
splat_id: 2453ff76-7894-510c-a00b-a82913d32f92
splat_kind: spell
splat_key: dnd2024:spells/fire_shield
splat_name: Fire Shield
splat_origin: dnd2024:reference/spell/fire_shield
splat_sources:
- SRD 5.2.1, pp. 132, https://www.dndbeyond.com/srd
---

# Fire Shield


## Spell

| Field | Value |
| --- | --- |
| Level | 4 |
| School | Evocation |
| Classes | Druid, Sorcerer, Wizard |
| Casting Time | Action |
| Range |  |
| Components | V, S, M (a bit of phosphorus or a firefly) |
| Duration | 10 minutes |


## Source text

<!-- SRD page 132 -->

Level 4 Evocation (Druid, Sorcerer, Wizard)\
Casting Time: Action\
Range: Self\
Components: V, S, M (a bit of phosphorus or a firefly)\
Duration: 10 minutes

Wispy flames wreathe your body for the duration, shedding Bright Light in a 10-foot radius and Dim Light for an additional 10 feet.

The flames provide you with a warm shield or a chill shield, as you choose. The warm shield grants you Resistance to Cold damage, and the chill shield grants you Resistance to Fire damage.

In addition, whenever a creature within 5 feet of you hits you with a melee attack roll, the shield erupts with flame. The attacker takes 2d8 Fire damage from a warm shield or 2d8 Cold damage from a chill shield.

Original layout: [[../../srd-5.2.1/pages/132|page 132]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/fire_shield ` |
| ` implementation ` | ` dnd2024:spell_fire_shield ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` target ` | ` caster ` |

#### Spell lifetime

<!-- splat:values path=splat_rule/parameters/spell_lifetime -->

| Field | Value |
| --- | --- |
| ` concentration ` | ` false ` |
| ` maximum_seconds ` | ` 600 ` |

#### Light

<!-- splat:values path=splat_rule/parameters/light -->

| Field | Value |
| --- | --- |
| ` bright_radius_feet ` | ` 10 ` |
| ` additional_dim_feet ` | ` 10 ` |

#### Choice

<!-- splat:values path=splat_rule/parameters/choice -->

| Field | Value |
| --- | --- |
| ` chooser ` | ` caster ` |
| ` options ` | ` ["warm", "chill"] ` |

#### Warm

<!-- splat:yaml path=splat_rule/parameters/warm -->

```yaml
resistance: cold
retaliatory_damage:
  dice: 2d8
  type: fire
```

#### Chill

<!-- splat:yaml path=splat_rule/parameters/chill -->

```yaml
resistance: fire
retaliatory_damage:
  dice: 2d8
  type: cold
```

#### Retaliation

<!-- splat:values path=splat_rule/parameters/retaliation -->

| Field | Value |
| --- | --- |
| ` event ` | ` attack.hit ` |
| ` attack_kind ` | ` melee_attack_roll ` |
| ` target ` | ` caster ` |
| ` attacker_kind ` | ` creature ` |
| ` attacker_within_feet ` | ` 5 ` |
| ` damage_recipient ` | ` attacker ` |

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
>   mode: self
> ```


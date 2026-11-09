---
splat_schema: 2
splat_id: 85c3e405-aefe-523e-acdf-0c2ac7c8ec52
splat_kind: spell
splat_key: dnd2024:spells/vitriolic_sphere
splat_name: Vitriolic Sphere
splat_origin: dnd2024:reference/spell/vitriolic_sphere
splat_sources:
- SRD 5.2.1, pp. 172, https://www.dndbeyond.com/srd
---

# Vitriolic Sphere


## Spell

| Field | Value |
| --- | --- |
| Level | 4 |
| School | Evocation |
| Classes | Sorcerer, Wizard |
| Casting Time | Action |
| Range | 150 feet |
| Components | V, S, M (a drop of bile) |
| Duration | Instantaneous |
| Ritual | No |
| Concentration | No |


## Source text

<!-- SRD page 172 -->

Level 4 Evocation (Sorcerer, Wizard)\
Casting Time: Action\
Range: 150 feet\
Components: V, S, M (a drop of bile)\
Duration: Instantaneous

You point at a location within range, and a glowing, 1-foot-diameter ball of acid streaks there and explodes in a 20-foot-radius Sphere. Each creature in that area makes a Dexterity saving throw. On a failed save, a creature takes 10d4 Acid damage and another 5d4 Acid damage at the end of its next turn. On a successful save, a creature takes half the initial damage only.

Using a Higher-Level Spell Slot. The initial damage increases by 2d4 for each spell slot level above 4.

Original layout: [[../../srd-5.2.1/pages/172|page 172]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/vitriolic_sphere ` |
| ` implementation ` | ` dnd2024:spell_vitriolic_sphere ` |

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
| ` radius_feet ` | ` 20 ` |
| ` origin ` | ` chosen_location ` |
| ` range_feet ` | ` 150 ` |
| ` projectile_diameter_feet ` | ` 1 ` |

#### Targets

<!-- splat:values path=splat_rule/parameters/targets -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` selection ` | ` all_in_area ` |

#### Save

<!-- splat:values path=splat_rule/parameters/save -->

| Field | Value |
| --- | --- |
| ` ability ` | ` dexterity ` |
| ` dc ` | ` spell_save_dc ` |

#### Failure

<!-- splat:yaml path=splat_rule/parameters/failure -->

```yaml
initial_damage:
  dice:
    base_count: 10
    sides: 4
    extra_per_slot: 2
    base_slot: 4
  type: acid
delayed_damage:
  event: turn.end
  subject: target
  occurrence: next
  dice: 5d4
  type: acid
```

#### Success

<!-- splat:yaml path=splat_rule/parameters/success -->

```yaml
initial_damage_scale:
  numerator: 1
  denominator: 2
delayed_damage: false
```

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
> ```


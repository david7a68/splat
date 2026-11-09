---
splat_schema: 2
splat_id: 051910fb-59e0-55da-9b69-21b335f12d31
splat_kind: spell
splat_key: dnd2024:spells/wind_wall
splat_name: Wind Wall
splat_origin: dnd2024:reference/spell/wind_wall
splat_sources:
- SRD 5.2.1, pp. 174, https://www.dndbeyond.com/srd
---

# Wind Wall


## Spell

| Field | Value |
| --- | --- |
| Level | 3 |
| School | Evocation |
| Classes | Druid, Ranger |
| Casting Time | Action |
| Range | 120 feet |
| Components | V, S, M (a fan and a feather) |
| Duration | Concentration, up to 1 minute |
| Ritual | No |
| Concentration | Yes |


## Source text

<!-- SRD page 174 -->

Level 3 Evocation (Druid, Ranger)\
Casting Time: Action\
Range: 120 feet\
Components: V, S, M (a fan and a feather)\
Duration: Concentration, up to 1 minute

A wall of strong wind rises from the ground at a point you choose within range. You can make the wall up to 50 feet long, 15 feet high, and 1 foot thick. You can shape the wall in any way you choose so long as it makes one continuous path along the ground. The wall lasts for the duration.

When the wall appears, each creature in its area makes a Strength saving throw, taking 4d8 Bludgeoning damage on a failed save or half as much damage on a successful one.

The strong wind keeps fog, smoke, and other gases at bay. Small or smaller flying creatures or objects can’t pass through the wall. Loose, lightweight materials brought into the wall fly upward. Arrows, bolts, and other ordinary projectiles launched at targets behind the wall are deflected upward and miss automatically. Boulders hurled by Giants or siege engines, and similar projectiles, are unaffected. Creatures in gaseous form can’t pass through it.

Original layout: [[../../srd-5.2.1/pages/174|page 174]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/wind_wall ` |
| ` implementation ` | ` dnd2024:spell_wind_wall ` |

### Parameters

#### Spell lifetime

<!-- splat:values path=splat_rule/parameters/spell_lifetime -->

| Field | Value |
| --- | --- |
| ` concentration ` | ` true ` |
| ` maximum_seconds ` | ` 60 ` |

#### Wall

<!-- splat:values path=splat_rule/parameters/wall -->

| Field | Value |
| --- | --- |
| ` origin ` | ` chosen_point_on_ground ` |
| ` range_feet ` | ` 120 ` |
| ` maximum_length_feet ` | ` 50 ` |
| ` maximum_height_feet ` | ` 15 ` |
| ` shape ` | ` caster_choice ` |
| ` continuous_path_on_ground ` | ` true ` |
| ` wind ` | ` strong ` |
| ` maximum_thickness_feet ` | ` 1 ` |

#### Initial

<!-- splat:yaml path=splat_rule/parameters/initial -->

```yaml
targets: all_creatures_in_area
save:
  ability: strength
  dc: spell_save_dc
failure_damage:
  dice: 4d8
  type: bludgeoning
success_damage_scale:
  numerator: 1
  denominator: 2
```

#### Blocked

<!-- splat:values path=splat_rule/parameters/blocked -->

| Field | Value |
| --- | --- |
| ` gases ` | ` ["fog", "smoke", "other_gases"] ` |
| ` flying_creatures_and_objects_maximum_size ` | ` small ` |
| ` gaseous_form_creatures ` | ` true ` |

#### Loose lightweight material

<!-- splat:values path=splat_rule/parameters/loose_lightweight_material -->

| Field | Value |
| --- | --- |
| ` on_enter ` | ` fly_upward ` |

#### Projectiles

<!-- splat:yaml path=splat_rule/parameters/projectiles -->

```yaml
ordinary:
  examples:
  - arrows
  - bolts
  target_behind_wall:
    deflect: upward
    attack_result: automatic_miss
exceptions_unaffected:
- giant_hurled_boulders
- siege_engine_projectiles
- similar_projectiles
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


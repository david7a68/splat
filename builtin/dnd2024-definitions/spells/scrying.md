---
splat_schema: 2
splat_id: 8174ca4b-590c-5811-a154-1f6821ae809c
splat_kind: spell
splat_key: dnd2024:spells/scrying
splat_name: Scrying
splat_origin: dnd2024:reference/spell/scrying
splat_sources:
- SRD 5.2.1, pp. 160, https://www.dndbeyond.com/srd
---

# Scrying


## Spell

| Field | Value |
| --- | --- |
| Level | 5 |
| School | Divination |
| Classes | Bard, Cleric, Druid, Warlock, Wizard |
| Casting Time | 10 minutes |
| Range |  |
| Components | V, S, M (a focus worth 1,000+ GP, such as a crystal ball, mirror, or water-filled font) |
| Duration | Concentration, up to 10 minutes |
| Ritual | No |
| Concentration | Yes |


## Source text

<!-- SRD page 160 -->

Level 5 Divination (Bard, Cleric, Druid, Warlock, Wizard)\
Casting Time: 10 minutes\
Range: Self\
Components: V, S, M (a focus worth 1,000+ GP, such as a crystal ball, mirror, or water-filled font)\
Duration: Concentration, up to 10 minutes

You can see and hear a creature you choose that is on the same plane of existence as you. The target makes a Wisdom saving throw, which is modified (see the tables below) by how well you know the target and the sort of physical connection you have to it. The target doesn’t know what it is making the save against, only that it feels uneasy.

| Your Knowledge of the Target Is … | Save Modifier |
| --- | --- |
| Secondhand (heard of the target) | +5 |
| Firsthand (met the target) | +0 |
| Extensive (know the target well) | −5 |
| You Have the Target’s … | Save Modifier |
| Picture or other likeness | −2 |
| Garment or other possession | −4 |
| Body part, lock of hair, or bit of nail | −10 |

On a successful save, the target isn’t affected, and you can’t use this spell on it again for 24 hours.

On a failed save, the spell creates an Invisible, intangible sensor within 10 feet of the target. You can see and hear through the sensor as if you were there. The sensor moves with the target, remaining within 10 feet of it for the duration. If something can see the sensor, it appears as a luminous orb about the size of your fist.

Instead of targeting a creature, you can target a location you have seen. When you do so, the sensor appears at that location and doesn’t move.

Original layout: [[../../srd-5.2.1/pages/160|page 160]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/scrying ` |
| ` implementation ` | ` dnd2024:spell_scrying ` |

### Parameters

#### Targeting

##### Options

<!-- splat:table
path: splat_rule/parameters/targeting/options
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Kind:
    path: kind
    type: symbol
  Same plane:
    path: same_plane
    type: boolean
    optional: true
  Previously seen:
    path: previously_seen
    type: boolean
    optional: true
-->

| Key | Kind | Same plane | Previously seen |
| --- | --- | --- | --- |
| ` row_1 ` | ` Creature ` | true |  |
| ` row_2 ` | ` Location ` |  | true |

#### Creature save

<!-- splat:yaml path=splat_rule/parameters/creature_save -->

```yaml
ability: wisdom
knowledge_modifier:
  secondhand: 5
  firsthand: 0
  extensive: -5
connection_modifier:
  none: 0
  likeness: -2
  possession: -4
  body_part_hair_or_nail: -10
combination: knowledge_plus_selected_connection
target_awareness: unease_without_knowing_save_source
success:
  no_effect: true
  cannot_retarget_hours: 24
failure: create_sensor
connection_selection:
  type: choice
  chooser: caster
  choices: available_physical_connections
  count: 1
```

#### Sensor

<!-- splat:yaml path=splat_rule/parameters/sensor -->

```yaml
invisible: true
intangible: true
channels:
- vision
- hearing
perspective: sensor_location
creature_mode:
  distance_max_feet: 10
  moves_with_target: true
location_mode:
  location: chosen_location
  moves: false
  save: none
visible_appearance: fist_sized_luminous_orb
duration: spell_duration
```

#### Spell lifetime

<!-- splat:yaml path=splat_rule/parameters/spell_lifetime -->

```yaml
concentration: true
maximum:
  value: 10
  unit: minute
```

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> material:
>   consumed: false
> range:
>   mode: self
> ```


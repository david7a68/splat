---
splat_schema: 2
splat_id: 7c12a608-4c5d-5a17-b3cc-5e8600636ad5
splat_kind: spell
splat_key: dnd2024:spells/alarm
splat_name: Alarm
splat_origin: dnd2024:reference/spell/alarm
splat_sources:
- SRD 5.2.1, pp. 107, https://www.dndbeyond.com/srd
---

# Alarm


## Spell

| Field | Value |
| --- | --- |
| Level | 1 |
| School | Abjuration |
| Classes | Ranger, Wizard |
| Casting Time | 1 minute or Ritual |
| Range | 30 feet |
| Components | V, S, M (a bell and silver wire) |
| Duration | 8 hours |
| Ritual | Yes |
| Concentration | No |


## Source text

<!-- SRD page 107 -->

Level 1 Abjuration (Ranger, Wizard)\
Casting Time: 1 minute or Ritual\
Range: 30 feet\
Components: V, S, M (a bell and silver wire)\
Duration: 8 hours

You set an alarm against intrusion. Choose a door, a window, or an area within range that is no larger than a 20-foot Cube. Until the spell ends, an alarm alerts you whenever a creature touches or enters the warded area. When you cast the spell, you can designate creatures that won’t set off the alarm. You also choose whether the alarm is audible or mental:

Audible Alarm. The alarm produces the sound of a handbell for 10 seconds within 60 feet of the warded area.

Mental Alarm. You are alerted by a mental ping if you are within 1 mile of the warded area. This ping awakens you if you’re asleep.

Original layout: [[../../srd-5.2.1/pages/107|page 107]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/alarm ` |
| ` implementation ` | ` dnd2024:spell_alarm ` |

### Parameters

#### Spell lifetime

<!-- splat:values path=splat_rule/parameters/spell_lifetime -->

| Field | Value |
| --- | --- |
| ` concentration ` | ` false ` |
| ` maximum_seconds ` | ` 28800 ` |

#### Target

<!-- splat:values path=splat_rule/parameters/target -->

| Field | Value |
| --- | --- |
| ` kinds ` | ` ["door", "window", "area"] ` |
| ` range_feet ` | ` 30 ` |
| ` maximum_cube_side_feet ` | ` 20 ` |

#### Exemptions

<!-- splat:values path=splat_rule/parameters/exemptions -->

| Field | Value |
| --- | --- |
| ` chooser ` | ` caster ` |
| ` selection ` | ` creature_list ` |
| ` timing ` | ` cast ` |

#### Trigger

<!-- splat:values path=splat_rule/parameters/trigger -->

| Field | Value |
| --- | --- |
| ` events ` | ` ["creature.touched_area", "creature.entered_area"] ` |
| ` exclude_designated_creatures ` | ` true ` |
| ` repeat ` | ` every_occurrence ` |

#### Alarm choice

<!-- splat:yaml path=splat_rule/parameters/alarm_choice -->

```yaml
chooser: caster
timing: cast
options:
  audible:
    sound: handbell
    seconds: 10
    audible_distance_feet: 60
    origin: warded_area
  mental:
    recipient: caster
    maximum_distance_miles: 1
    signal: mental_ping
    awakens_sleeping_recipient: true
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


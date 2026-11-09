---
splat_schema: 2
splat_id: bdbdaf92-8dfd-5bc6-8d11-10d4946d054d
splat_kind: spell
splat_key: dnd2024:spells/storm_of_vengeance
splat_name: Storm of Vengeance
splat_origin: dnd2024:reference/spell/storm_of_vengeance
splat_sources:
- SRD 5.2.1, pp. 165, 166, https://www.dndbeyond.com/srd
---

# Storm of Vengeance


## Spell

| Field | Value |
| --- | --- |
| Level | 9 |
| School | Conjuration |
| Classes | Druid |
| Casting Time | Action |
| Range |  |
| Components | V, S |
| Duration | Concentration, up to 1 minute |
| Ritual | No |
| Concentration | Yes |


## Source text

<!-- SRD page 165 -->

Level 9 Conjuration (Druid)\
Casting Time: Action\
Range: 1 mile\
Components: V, S\
Duration: Concentration, up to 1 minute

A churning storm cloud forms for the duration, centered on a point within range and spreading to a radius of 300 feet. Each creature under the cloud when it appears must succeed on a Constitution saving throw or take 2d6 Thunder damage and have the Deafened condition for the duration.

At the start of each of your later turns, the storm produces different effects, as detailed below.

Turn 2. Acidic rain falls. Each creature and object under the cloud takes 4d6 Acid damage.

Turn 3. You call six bolts of lightning from the cloud to strike six different creatures or objects beneath it. Each target makes a Dexterity saving throw, taking 10d6 Lightning damage on a failed save or half as much damage on a successful one.

<!-- SRD page 166 -->

Turn 4. Hailstones rain down. Each creature under the cloud takes 2d6 Bludgeoning damage.

Turns 5–10. Gusts and freezing rain assail the area under the cloud. Each creature there takes 1d6 Cold damage. Until the spell ends, the area is Difficult Terrain and Heavily Obscured, ranged attacks with weapons are impossible there, and strong wind blows through the area.

Original layout: [[../../srd-5.2.1/pages/165|page 165]], [[../../srd-5.2.1/pages/166|page 166]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/storm_of_vengeance ` |
| ` implementation ` | ` dnd2024:spell_storm_of_vengeance ` |

### Parameters

#### Cloud

<!-- splat:values path=splat_rule/parameters/cloud -->

| Field | Value |
| --- | --- |
| ` radius_feet ` | ` 300 ` |
| ` center ` | ` chosen_point_within_range ` |
| ` duration ` | ` spell_duration ` |

#### Initial

<!-- splat:yaml path=splat_rule/parameters/initial -->

```yaml
targets: creatures_under_cloud_at_appearance
save:
  ability: constitution
  dc:
    binding: save_dc
  failure:
    damage:
      dice:
        count: 2
        sides: 6
        add: 0
      type: thunder
    condition: deafened
    duration: spell_duration
  success:
    effect: none
```

#### Later effects

<!-- splat:yaml path=splat_rule/parameters/later_effects -->

```yaml
event: caster_turn_start
turns:
- number: 2
  targets: each_creature_and_object_under_cloud
  damage:
    dice:
      count: 4
      sides: 6
      add: 0
    type: acid
- number: 3
  targets:
    count: 6
    different: true
    kinds:
    - creature
    - object
    under_cloud: true
    chooser: caster
  save:
    ability: dexterity
    dc:
      binding: save_dc
    failure:
      damage:
        dice:
          count: 10
          sides: 6
          add: 0
        type: lightning
    success:
      damage:
        dice:
          count: 10
          sides: 6
          add: 0
        type: lightning
      damage_fraction:
        numerator: 1
        denominator: 2
- number: 4
  targets: each_creature_under_cloud
  damage:
    dice:
      count: 2
      sides: 6
      add: 0
    type: bludgeoning
- numbers:
  - 5
  - 6
  - 7
  - 8
  - 9
  - 10
  targets: each_creature_under_cloud
  damage:
    dice:
      count: 1
      sides: 6
      add: 0
    type: cold
  area_until_spell_ends:
    terrain: difficult
    obscurement: heavy
    ranged_weapon_attacks: impossible
    wind: strong
```

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> range:
>   mode: 1 mile
> ```


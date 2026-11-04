---
splat_schema: 2
splat_id: e5e91d65-5f33-5e24-aa7f-dc5c4154bb28
splat_kind: spell
splat_key: dnd2024:spells/fire_storm
splat_name: Fire Storm
splat_origin: dnd2024:reference/spell/fire_storm
splat_sources:
- SRD 5.2.1, pp. 132, https://www.dndbeyond.com/srd
---

# Fire Storm


## Spell

| Field | Value |
| --- | --- |
| Level | 7 |
| School | Evocation |
| Classes | Cleric, Druid, Sorcerer |
| Casting Time | Action |
| Range | 150 feet |
| Components | V, S |
| Duration | Instantaneous |


## Source text

<!-- SRD page 132 -->

Level 7 Evocation (Cleric, Druid, Sorcerer)\
Casting Time: Action\
Range: 150 feet\
Components: V, S\
Duration: Instantaneous

A storm of fire appears within range. The area of the storm consists of up to ten 10-foot Cubes, which you arrange as you like. Each Cube must be contiguous with at least one other Cube. Each creature in the area makes a Dexterity saving throw, taking 7d10 Fire damage on a failed save or half as much damage on a successful one.

Flammable objects in the area that aren’t being worn or carried start burning.

Original layout: [[../../srd-5.2.1/pages/132|page 132]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/fire_storm ` |
| ` implementation ` | ` dnd2024:spell_fire_storm ` |

### Parameters

#### Area

<!-- splat:values path=splat_rule/parameters/area -->

| Field | Value |
| --- | --- |
| ` shape ` | ` cubes ` |
| ` maximum_count ` | ` 10 ` |
| ` side_feet ` | ` 10 ` |
| ` each_contiguous_with_another ` | ` true ` |
| ` arrangement_chooser ` | ` caster ` |

#### Save

<!-- splat:yaml path=splat_rule/parameters/save -->

```yaml
ability: dexterity
dc:
  binding: save_dc
failure:
  damage:
    amount:
      dice:
        count: 7
        sides: 10
        add: 0
    type: fire
success:
  damage_fraction:
    numerator: 1
    denominator: 2
```

#### Objects

<!-- splat:values path=splat_rule/parameters/objects -->

| Field | Value |
| --- | --- |
| ` flammable ` | ` true ` |
| ` not_worn_or_carried ` | ` true ` |
| ` effect ` | ` burning ` |

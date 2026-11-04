---
splat_schema: 2
splat_id: 9ce28dee-20bd-5fa6-b585-1a86c8c37f08
splat_kind: spell
splat_key: dnd2024:spells/weird
splat_name: Weird
splat_origin: dnd2024:reference/spell/weird
splat_sources:
- SRD 5.2.1, pp. 174, https://www.dndbeyond.com/srd
---

# Weird


## Spell

| Field | Value |
| --- | --- |
| Level | 9 |
| School | Illusion |
| Classes | Warlock, Wizard |
| Casting Time | Action |
| Range | 120 feet |
| Components | V, S |
| Duration | Concentration, up to 1 minute |


## Source text

<!-- SRD page 174 -->

Level 9 Illusion (Warlock, Wizard)\
Casting Time: Action\
Range: 120 feet\
Components: V, S\
Duration: Concentration, up to 1 minute

You try to create illusory terrors in others’ minds. Each creature of your choice in a 30-foot-radius Sphere centered on a point within range makes a Wisdom saving throw. On a failed save, a target takes 10d10 Psychic damage and has the Frightened condition for the duration. On a successful save, a target takes half as much damage only.

A Frightened target makes a Wisdom saving throw at the end of each of its turns. On a failed save, it takes 5d10 Psychic damage. On a successful save, the spell ends on that target.

Original layout: [[../../srd-5.2.1/pages/174|page 174]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/weird ` |
| ` implementation ` | ` dnd2024:spell_weird ` |

### Parameters

#### Targeting

<!-- splat:values path=splat_rule/parameters/targeting -->

| Field | Value |
| --- | --- |
| ` kind ` | ` chosen_creatures_in_area ` |
| ` shape ` | ` sphere ` |
| ` radius_feet ` | ` 30 ` |
| ` origin ` | ` chosen_point_in_range ` |

#### Initial save

<!-- splat:yaml path=splat_rule/parameters/initial_save -->

```yaml
ability: wisdom
failure:
  damage:
    dice: 10d10
    type: psychic
  condition: frightened
  duration: spell_duration
success:
  damage_fraction:
    numerator: 1
    denominator: 2
```

#### Repeat

<!-- splat:yaml path=splat_rule/parameters/repeat -->

```yaml
event: frightened_target.turn_ended
save:
  ability: wisdom
  failure:
    damage:
      dice: 5d10
      type: psychic
  success:
    end_spell_on_target: true
```

#### Spell lifetime

<!-- splat:yaml path=splat_rule/parameters/spell_lifetime -->

```yaml
concentration: true
maximum:
  value: 1
  unit: minute
```

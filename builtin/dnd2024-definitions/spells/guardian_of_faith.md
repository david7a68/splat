---
splat_schema: 2
splat_id: 19de9a9b-8f93-58bd-9a7d-2b61d133bed4
splat_kind: spell
splat_key: dnd2024:spells/guardian_of_faith
splat_name: Guardian of Faith
splat_origin: dnd2024:reference/spell/guardian_of_faith
splat_sources:
- SRD 5.2.1, pp. 137, https://www.dndbeyond.com/srd
---

# Guardian of Faith


## Spell

| Field | Value |
| --- | --- |
| Level | 4 |
| School | Conjuration |
| Classes | Cleric |
| Casting Time | Action |
| Range | 30 feet |
| Components | V |
| Duration | 8 hours |


## Source text

<!-- SRD page 137 -->

Level 4 Conjuration (Cleric)\
Casting Time: Action\
Range: 30 feet\
Components: V\
Duration: 8 hours

A Large spectral guardian appears and hovers for the duration in an unoccupied space that you can see within range. The guardian occupies that space and is invulnerable, and it appears in a form appropriate for your deity or pantheon.

Any enemy that moves to a space within 10 feet of the guardian for the first time on a turn or starts its turn there makes a Dexterity saving throw, taking 20 Radiant damage on a failed save or half as much damage on a successful one. The guardian vanishes when it has dealt a total of 60 damage.

Original layout: [[../../srd-5.2.1/pages/137|page 137]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/guardian_of_faith ` |
| ` implementation ` | ` dnd2024:spell_guardian_of_faith ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` triggers ` | ` ["moves_into_distance_first_time_on_turn", "starts_turn_in_distance"] ` |

#### Creation

<!-- splat:yaml path=splat_rule/parameters/creation -->

```yaml
size: large
kind: spectral_guardian
space:
  unoccupied: true
  visible: true
  within_spell_range: true
hover: true
occupies_space: true
invulnerable: true
duration: spell_duration
appearance_constraint: appropriate_to_caster_deity_or_pantheon
```

#### Targets

<!-- splat:values path=splat_rule/parameters/targets -->

| Field | Value |
| --- | --- |
| ` relation ` | ` enemy ` |
| ` distance_from_guardian_maximum_feet ` | ` 10 ` |

#### Save

<!-- splat:yaml path=splat_rule/parameters/save -->

```yaml
ability: dexterity
dc:
  binding: save_dc
failure:
  damage:
    amount: 20
    type: radiant
success:
  damage:
    amount: 10
    type: radiant
```

#### Vanish when

<!-- splat:values path=splat_rule/parameters/vanish_when -->

| Field | Value |
| --- | --- |
| ` total_damage_dealt_minimum ` | ` 60 ` |

---
splat_schema: 2
splat_id: e41a0eb6-bbbe-527a-a388-db5af40fa96d
splat_kind: spell
splat_key: dnd2024:spells/entangle
splat_name: Entangle
splat_origin: dnd2024:reference/spell/entangle
splat_sources:
- SRD 5.2.1, pp. 128, https://www.dndbeyond.com/srd
---

# Entangle


## Spell

| Field | Value |
| --- | --- |
| Level | 1 |
| School | Conjuration |
| Classes | Druid, Ranger |
| Casting Time | Action |
| Range | 90 feet |
| Components | V, S |
| Duration | Concentration, up to 1 minute |
| Ritual | No |
| Concentration | Yes |


## Source text

<!-- SRD page 128 -->

Level 1 Conjuration (Druid, Ranger)\
Casting Time: Action\
Range: 90 feet\
Components: V, S\
Duration: Concentration, up to 1 minute

Grasping plants sprout from the ground in a 20-foot square within range. For the duration, these plants turn the ground in the area into Difficult Terrain. They disappear when the spell ends.

Each creature (other than you) in the area when you cast the spell must succeed on a Strength saving throw or have the Restrained condition until the spell ends. A Restrained creature can take an action to make a Strength (Athletics) check against your spell save DC. On a success, it frees itself from the grasping plants and is no longer Restrained by them.

Original layout: [[../../srd-5.2.1/pages/128|page 128]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/entangle ` |
| ` implementation ` | ` dnd2024:spell_entangle ` |

### Parameters

#### Area

<!-- splat:values path=splat_rule/parameters/area -->

| Field | Value |
| --- | --- |
| ` shape ` | ` square ` |
| ` side_feet ` | ` 20 ` |
| ` origin ` | ` ground_within_spell_range ` |
| ` terrain ` | ` difficult ` |
| ` duration ` | ` spell_duration ` |
| ` plants_disappear_on_end ` | ` true ` |

#### Targets

<!-- splat:values path=splat_rule/parameters/targets -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` in_area_at_cast ` | ` true ` |
| ` exclude_caster ` | ` true ` |

#### Save

<!-- splat:yaml path=splat_rule/parameters/save -->

```yaml
ability: strength
dc:
  binding: save_dc
failure:
  condition: restrained
  until: spell_ends
success:
  effect: none
```

#### Escape

<!-- splat:yaml path=splat_rule/parameters/escape -->

```yaml
cost: action
check:
  ability: strength
  skill: athletics
  dc:
    binding: save_dc
success: end_restrained_from_spell
```

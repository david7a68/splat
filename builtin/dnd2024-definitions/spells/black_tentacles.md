---
splat_schema: 2
splat_id: 47dc5eea-7370-5391-a655-11e86303c94d
splat_kind: spell
splat_key: dnd2024:spells/black_tentacles
splat_name: Black Tentacles
splat_origin: dnd2024:reference/spell/black_tentacles
splat_sources:
- SRD 5.2.1, pp. 113, https://www.dndbeyond.com/srd
---

# Black Tentacles


## Spell

| Field | Value |
| --- | --- |
| Level | 4 |
| School | Conjuration |
| Classes | Wizard |
| Casting Time | Action |
| Range | 90 feet |
| Components | V, S, M (a tentacle) |
| Duration | Concentration, up to 1 minute |
| Ritual | No |
| Concentration | Yes |


## Source text

<!-- SRD page 113 -->

Level 4 Conjuration (Wizard)\
Casting Time: Action\
Range: 90 feet\
Components: V, S, M (a tentacle)\
Duration: Concentration, up to 1 minute

Squirming, ebony tentacles fill a 20-foot square on ground that you can see within range. For the duration, these tentacles turn the ground in that area into Difficult Terrain.

Each creature in that area makes a Strength saving throw. On a failed save, it takes 3d6 Bludgeoning damage, and it has the Restrained condition until the spell ends. A creature also makes that save if it enters the area or ends it turn there. A creature makes that save only once per turn.

A Restrained creature can take an action to make a Strength (Athletics) check against your spell save DC, ending the condition on itself on a success.

Original layout: [[../../srd-5.2.1/pages/113|page 113]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/black_tentacles ` |
| ` implementation ` | ` dnd2024:spell_black_tentacles ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` triggers ` | ` ["spell.cast_completed", "area.enter", "turn.end"] ` |

#### Area

<!-- splat:values path=splat_rule/parameters/area -->

| Field | Value |
| --- | --- |
| ` shape ` | ` square ` |
| ` side_feet ` | ` 20 ` |
| ` ground ` | ` true ` |
| ` visible ` | ` true ` |
| ` difficult_terrain ` | ` true ` |

#### Limit

<!-- splat:values path=splat_rule/parameters/limit -->

| Field | Value |
| --- | --- |
| ` saves_per_creature_per_turn ` | ` 1 ` |

#### Save

<!-- splat:yaml path=splat_rule/parameters/save -->

```yaml
ability: strength
dc:
  binding: save_dc
failure:
  damage:
    amount:
      dice:
        count: 3
        sides: 6
        add: 0
    type: bludgeoning
  condition: restrained
  condition_duration: spell
success:
  effect: none
```

#### Escape

<!-- splat:yaml path=splat_rule/parameters/escape -->

```yaml
action: action
ability: strength
skill: athletics
dc:
  binding: save_dc
success: end_restrained_on_self
```

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> material:
>   consumed: false
> ```


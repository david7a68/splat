---
splat_schema: 1
splat_id: c1fe9cc5-f0f5-582c-afee-9066245175eb
splat_kind: spell
splat_key: dnd2024:spells/time_stop
splat_name: Time Stop
splat_origin:
  key: dnd2024:reference/spell/time_stop
splat_sources:
- title: SRD 5.2.1
  locator: pp. 169
  url: https://www.dndbeyond.com/srd
---

# Time Stop

## Source text

<!-- SRD page 169 -->

Level 9 Transmutation (Sorcerer, Wizard)\
Casting Time: Action\
Range: Self\
Components: V\
Duration: Instantaneous

You briefly stop the flow of time for everyone but yourself. No time passes for other creatures, while you take 1d4 + 1 turns in a row, during which you can use actions and move as normal.

This spell ends if one of the actions you use during this period, or any effects that you create during it, affects a creature other than you or an object being worn or carried by someone other than you. In addition, the spell ends if you move to a place more than 1,000 feet from the location where you cast it.

Original layout: [[../../srd-5.2.1/pages/169|page 169]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 9 ` |
| ` school ` | ` transmutation ` |
| ` classes ` | ` ["sorcerer", "wizard"] ` |
| ` components ` | ` ["verbal"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` Instantaneous ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` mode ` | ` self ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/time_stop ` |
| ` implementation ` | ` dnd2024:spell_time_stop ` |

### Parameters

#### Time

<!-- splat:yaml path=splat_rule/parameters/time -->

```yaml
other_creatures_elapsed_time: 0
caster_consecutive_turns:
  dice:
    count: 1
    sides: 4
    add: 1
caster_actions_and_movement: normal
```

#### End when

<!-- splat:values path=splat_rule/parameters/end_when -->

| Field | Value |
| --- | --- |
| ` caster_action_or_created_effect_affects_any ` | ` ["other_creature", "object_worn_or_carried_by_other_creature"] ` |
| ` caster_distance_from_cast_location_feet_greater_than ` | ` 1000 ` |

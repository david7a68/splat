---
splat_schema: 1
splat_id: 3372e62c-39f0-5f70-8e07-a95722ffd0a4
splat_kind: spell
splat_key: dnd2024:spells/freedom_of_movement
splat_name: Freedom of Movement
splat_origin:
  key: dnd2024:reference/spell/freedom_of_movement
splat_sources:
- title: SRD 5.2.1
  locator: pp. 134
  url: https://www.dndbeyond.com/srd
---

# Freedom of Movement

## Source text

<!-- SRD page 134 -->

Level 4 Abjuration (Bard, Cleric, Druid, Ranger)\
Casting Time: Action\
Range: Touch\
Components: V, S, M (a leather strap)\
Duration: 1 hour\
You touch a willing creature. For the duration,\
the target’s movement is unaffected by Difficult\
Terrain, and spells and other magical effects can\
neither reduce the target’s Speed nor cause the tar-\
get to have the Paralyzed or Restrained conditions.\
The target also has a Swim Speed equal to its Speed.\
In addition, the target can spend 5 feet of move-\
ment to automatically escape from nonmagical\
restraints, such as manacles or a creature imposing\
the Grappled condition on it.\
Using a Higher-Level Spell Slot. You can target\
one additional creature for each spell slot level\
above 4.\

Original layout: [[../../srd-5.2.1/pages/134|page 134]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 4 ` |
| ` school ` | ` abjuration ` |
| ` classes ` | ` ["bard", "cleric", "druid", "ranger"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` 1 hour ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` mode ` | ` touch ` |

### Material

<!-- splat:values path=splat_spell/material -->

| Field | Value |
| --- | --- |
| ` description ` | ` a leather strap ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/freedom_of_movement ` |
| ` implementation ` | ` dnd2024:spell_freedom_of_movement ` |

### Parameters

#### Target

<!-- splat:values path=splat_rule/parameters/target -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` count ` | ` 1 ` |
| ` visible_to_caster ` | ` false ` |
| ` willing ` | ` true ` |
| ` touch ` | ` true ` |

#### Effects

<!-- splat:yaml path=splat_rule/parameters/effects -->

```yaml
ignore_difficult_terrain: true
magical_speed_reduction: false
magically_imposed_conditions_prevented:
- paralyzed
- restrained
swim_speed:
  stat:
    of: target
    key: speed
```

#### Escape

<!-- splat:values path=splat_rule/parameters/escape -->

| Field | Value |
| --- | --- |
| ` cost_movement_feet ` | ` 5 ` |
| ` automatic ` | ` true ` |
| ` eligible ` | ` ["nonmagical_restraints", "nonmagical_grapple"] ` |

#### Scaling

<!-- splat:values path=splat_rule/parameters/scaling -->

| Field | Value |
| --- | --- |
| ` per_slot_above ` | ` 4 ` |
| ` additional_targets ` | ` 1 ` |

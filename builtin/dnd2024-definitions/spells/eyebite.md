---
splat_schema: 2
splat_id: 4f0f395a-ed54-5d6b-b89b-fd9cc60369b5
splat_kind: spell
splat_key: dnd2024:spells/eyebite
splat_name: Eyebite
splat_origin: dnd2024:reference/spell/eyebite
splat_sources:
- SRD 5.2.1, pp. 129, https://www.dndbeyond.com/srd
---

# Eyebite


## Spell

| Field | Value |
| --- | --- |
| Level | 6 |
| School | Necromancy |
| Classes | Bard, Sorcerer, Warlock, Wizard |
| Casting Time | Action |
| Range |  |
| Components | V, S |
| Duration | Concentration, up to 1 minute |


## Source text

<!-- SRD page 129 -->

Level 6 Necromancy (Bard, Sorcerer, Warlock, Wizard)\
Casting Time: Action\
Range: Self\
Components: V, S\
Duration: Concentration, up to 1 minute

For the duration, your eyes become an inky void. One creature of your choice within 60 feet of you that you can see must succeed on a Wisdom saving throw or be affected by one of the following effects of your choice for the duration.

On each of your turns until the spell ends, you can take a Magic action to target another creature but can’t target a creature again if it has succeeded on a save against this casting of the spell.

Asleep. The target has the Unconscious condition. It wakes up if it takes any damage or if another creature takes an action to shake it awake.

Panicked. The target has the Frightened condition. On each of its turns, the Frightened target must take the Dash action and move away from you by the safest and shortest route available. If the target moves to a space at least 60 feet away from you where it can’t see you, this effect ends.

Sickened. The target has the Poisoned condition.

Original layout: [[../../srd-5.2.1/pages/129|page 129]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/eyebite ` |
| ` implementation ` | ` dnd2024:spell_eyebite ` |

### Parameters

#### Targets

<!-- splat:values path=splat_rule/parameters/targets -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` visible ` | ` true ` |
| ` maximum_range_feet ` | ` 60 ` |
| ` count_per_activation ` | ` 1 ` |
| ` exclude_previous_successful_savers_this_cast ` | ` true ` |

#### Activation

<!-- splat:yaml path=splat_rule/parameters/activation -->

```yaml
initial: spell_cast
subsequent:
  cost: magic_action
  'on': caster_turn
```

#### Save

<!-- splat:yaml path=splat_rule/parameters/save -->

```yaml
ability: wisdom
dc:
  binding: save_dc
failure:
  chosen_effect_duration: spell_duration
success:
  effect: none
```

#### Modes

<!-- splat:yaml path=splat_rule/parameters/modes -->

```yaml
asleep:
  condition: unconscious
  end_if:
  - target_takes_damage
  - another_creature_uses_action_to_shake_awake
panicked:
  condition: frightened
  fear_source: caster
  each_target_turn:
    action: dash
    movement: away_from_caster
    route:
    - safest
    - shortest
  end_if:
    distance_from_caster_feet_minimum: 60
    cannot_see_caster: true
sickened:
  condition: poisoned
```

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> range:
>   mode: self
> ```


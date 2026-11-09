---
splat_schema: 2
splat_id: a51449cb-c485-55a6-86ba-d96a901af1a0
splat_kind: spell
splat_key: dnd2024:spells/animate_dead
splat_name: Animate Dead
splat_origin: dnd2024:reference/spell/animate_dead
splat_sources:
- SRD 5.2.1, pp. 108, https://www.dndbeyond.com/srd
---

# Animate Dead


## Spell

| Field | Value |
| --- | --- |
| Level | 3 |
| School | Necromancy |
| Classes | Cleric, Wizard |
| Casting Time | 1 minute |
| Range | 10 feet |
| Components | V, S, M (a drop of blood, a piece of flesh, and a pinch of bone dust) |
| Duration | Instantaneous |
| Ritual | No |
| Concentration | No |


## Source text

<!-- SRD page 108 -->

Level 3 Necromancy (Cleric, Wizard)\
Casting Time: 1 minute\
Range: 10 feet\
Components: V, S, M (a drop of blood, a piece of flesh, and a pinch of bone dust)\
Duration: Instantaneous

Choose a pile of bones or a corpse of a Medium or Small Humanoid within range. The target becomes an Undead creature: a Skeleton if you chose bones or a Zombie if you chose a corpse (see “Monsters” for the stat blocks).

On each of your turns, you can take a Bonus Action to mentally command any creature you made with this spell if the creature is within 60 feet of you (if you control multiple creatures, you can command any of them at the same time, issuing the same command to each one). You decide what action the creature will take and where it will move on its next turn, or you can issue a general command, such as to guard a chamber or corridor. If you issue no commands, the creature takes the Dodge action and moves only to avoid harm. Once given an order, the creature continues to follow it until its task is complete.

The creature is under your control for 24 hours, after which it stops obeying any command you’ve given it. To maintain control of the creature for another 24 hours, you must cast this spell on the creature again before the current 24-hour period ends. This use of the spell reasserts your control over up to four creatures you have animated with this spell rather than animating a new creature.

Using a Higher-Level Spell Slot. You animate or reassert control over two additional Undead creatures for each spell slot level above 3. Each of the creatures must come from a different corpse or pile of bones.

Original layout: [[../../srd-5.2.1/pages/108|page 108]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/animate_dead ` |
| ` implementation ` | ` dnd2024:spell_animate_dead ` |

### Parameters

#### Target

<!-- splat:values path=splat_rule/parameters/target -->

| Field | Value |
| --- | --- |
| ` kind ` | ` bones_or_humanoid_corpse ` |
| ` count ` | ` 1 ` |
| ` visible_to_caster ` | ` false ` |
| ` size ` | ` ["small", "medium"] ` |
| ` unique_remains_per_creature ` | ` true ` |

#### Animation

<!-- splat:yaml path=splat_rule/parameters/animation -->

```yaml
bones:
  key: dnd2024:creatures/skeleton
corpse:
  key: dnd2024:creatures/zombie
```

#### Control

<!-- splat:yaml path=splat_rule/parameters/control -->

```yaml
hours: 24
command:
  action: bonus_action
  event: turn.current
  range_feet: 60
  telepathic: true
  targets: any_created_creatures_in_range
  same_command_all: true
  options:
  - next_turn_action_and_movement
  - general_task
  persists_until: task_completed
uncommanded:
  action: dodge
  movement: avoid_harm_only
expiry: stop_obeying
```

#### Reassert

<!-- splat:values path=splat_rule/parameters/reassert -->

| Field | Value |
| --- | --- |
| ` target ` | ` own_animated_creatures ` |
| ` before_control_expires ` | ` true ` |
| ` base_count ` | ` 4 ` |
| ` control_hours ` | ` 24 ` |
| ` replaces_animation ` | ` true ` |

#### Scaling

<!-- splat:values path=splat_rule/parameters/scaling -->

| Field | Value |
| --- | --- |
| ` per_slot_above ` | ` 3 ` |
| ` additional_animated ` | ` 2 ` |
| ` additional_reasserted ` | ` 2 ` |

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> material:
>   consumed: false
> ```


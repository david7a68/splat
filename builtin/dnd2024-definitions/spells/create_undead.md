---
splat_schema: 2
splat_id: 7b239c1f-21aa-5184-9215-48564b907c73
splat_kind: spell
splat_key: dnd2024:spells/create_undead
splat_name: Create Undead
splat_origin: dnd2024:reference/spell/create_undead
splat_sources:
- SRD 5.2.1, pp. 121, https://www.dndbeyond.com/srd
---

# Create Undead


## Spell

| Field | Value |
| --- | --- |
| Level | 6 |
| School | Necromancy |
| Classes | Cleric, Warlock, Wizard |
| Casting Time | 1 minute |
| Range | 10 feet |
| Components | V, S, M (one 150+ GP black onyx stone for each corpse) |
| Duration | Instantaneous |


## Source text

<!-- SRD page 121 -->

Level 6 Necromancy (Cleric, Warlock, Wizard)\
Casting Time: 1 minute\
Range: 10 feet\
Components: V, S, M (one 150+ GP black onyx stone for each corpse)\
Duration: Instantaneous

You can cast this spell only at night. Choose up to three corpses of Medium or Small Humanoids within range. Each one becomes a Ghoul under your control (see “Monsters” for its stat block).

As a Bonus Action on each of your turns, you can mentally command any creature you animated with this spell if the creature is within 120 feet of you (if you control multiple creatures, you can command any of them at the same time, issuing the same command to them). You decide what action the creature will take and where it will move on its next turn, or you can issue a general command, such as to guard a particular place. If you issue no commands, the creature takes the Dodge action and moves only to avoid harm. Once given an order, the creature continues to follow the order until its task is complete.

The creature is under your control for 24 hours, after which it stops obeying any command you’ve given it. To maintain control of the creature for another 24 hours, you must cast this spell on the creature before the current 24-hour period ends. This use of the spell reasserts your control over up to three creatures you have animated with this spell rather than animating new ones.

Using a Higher-Level Spell Slot. If you use a level 7 spell slot, you can animate or reassert control over four Ghouls. If you use a level 8 spell slot, you can animate or reassert control over five Ghouls or two Ghasts or Wights. If you use a level 9 spell slot, you can animate or reassert control over six Ghouls, three Ghasts or Wights, or two Mummies. See “Monsters” for these stat blocks.

Original layout: [[../../srd-5.2.1/pages/121|page 121]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/create_undead ` |
| ` implementation ` | ` dnd2024:spell_create_undead ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` modes ` | ` ["animate", "reassert_control"] ` |

#### Requires

<!-- splat:values path=splat_rule/parameters/requires -->

| Field | Value |
| --- | --- |
| ` time ` | ` night ` |

#### Targets

<!-- splat:yaml path=splat_rule/parameters/targets -->

```yaml
animate:
  kind: corpse
  former_type: humanoid
  sizes:
  - medium
  - small
  within_spell_range: true
reassert_control:
  kind: creature_animated_by_this_spell
  control_not_expired: true
  within_spell_range: true
```

#### Slot options

<!-- splat:yaml path=splat_rule/parameters/slot_options -->

```yaml
'6':
- kind: ghoul
  maximum: 3
'7':
- kind: ghoul
  maximum: 4
'8':
- kind: ghoul
  maximum: 5
- kinds:
  - ghast
  - wight
  maximum: 2
'9':
- kind: ghoul
  maximum: 6
- kinds:
  - ghast
  - wight
  maximum: 3
- kind: mummy
  maximum: 2
```

#### Control

<!-- splat:yaml path=splat_rule/parameters/control -->

```yaml
duration_hours: 24
renew_duration_hours: 24
expiry: stop_obeying_orders
command:
  cost: bonus_action
  event: caster_turn
  range_feet: 120
  communication: mental
  multiple_targets_same_command: true
  choices:
  - next_turn_action_and_movement
  - general_task
  persists_until: task_complete
no_command:
  action: dodge
  movement: avoid_harm_only
```

#### Stat blocks

<!-- splat:yaml path=splat_rule/parameters/stat_blocks -->

```yaml
ghoul:
  key: dnd2024:creatures/ghoul
ghast:
  key: dnd2024:creatures/ghast
wight:
  key: dnd2024:creatures/wight
mummy:
  key: dnd2024:creatures/mummy
```

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> material:
>   consumed: false
> ```


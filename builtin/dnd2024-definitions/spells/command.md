---
splat_schema: 2
splat_id: 662dd9da-c049-5604-8398-9c2809fde868
splat_kind: spell
splat_key: dnd2024:spells/command
splat_name: Command
splat_origin: dnd2024:reference/spell/command
splat_sources:
- SRD 5.2.1, pp. 116, https://www.dndbeyond.com/srd
---

# Command


## Spell

| Field | Value |
| --- | --- |
| Level | 1 |
| School | Enchantment |
| Classes | Bard, Cleric, Paladin |
| Casting Time | Action |
| Range | 60 feet |
| Components | V |
| Duration | Instantaneous |


## Source text

<!-- SRD page 116 -->

Level 1 Enchantment (Bard, Cleric, Paladin)\
Casting Time: Action\
Range: 60 feet\
Components: V\
Duration: Instantaneous

You speak a one-word command to a creature you can see within range. The target must succeed on a Wisdom saving throw or follow the command on its next turn. Choose the command from these options:

Approach. The target moves toward you by the shortest and most direct route, ending its turn if it moves within 5 feet of you.

Drop. The target drops whatever it is holding and then ends its turn.

Flee. The target spends its turn moving away from you by the fastest available means.

Grovel. The target has the Prone condition and then ends its turn.

Halt. On its turn, the target doesn’t move and takes no action or Bonus Action.

Using a Higher-Level Spell Slot. You can affect one additional creature for each spell slot level above 1.

Original layout: [[../../srd-5.2.1/pages/116|page 116]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/command ` |
| ` implementation ` | ` dnd2024:spell_command ` |

### Parameters

#### Target

<!-- splat:values path=splat_rule/parameters/target -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` count ` | ` 1 ` |
| ` visible_to_caster ` | ` true ` |

#### Save

<!-- splat:yaml path=splat_rule/parameters/save -->

```yaml
ability: wisdom
dc:
  binding: save_dc
failure:
  execute_command: next_target_turn
success:
  effect: none
```

#### Choice

<!-- splat:values path=splat_rule/parameters/choice -->

| Field | Value |
| --- | --- |
| ` chooser ` | ` caster ` |
| ` words ` | ` 1 ` |
| ` options ` | ` ["approach", "drop", "flee", "grovel", "halt"] ` |

#### Commands

<!-- splat:yaml path=splat_rule/parameters/commands -->

```yaml
approach:
  movement: toward_caster
  route: shortest_most_direct
  end_turn_within_feet: 5
drop:
  drop: all_held_objects
  then: end_turn
flee:
  whole_turn: move_away_from_caster
  means: fastest_available
grovel:
  condition: prone
  then: end_turn
halt:
  movement: false
  action: false
  bonus_action: false
```

#### Scaling

<!-- splat:values path=splat_rule/parameters/scaling -->

| Field | Value |
| --- | --- |
| ` per_slot_above ` | ` 1 ` |
| ` additional_targets ` | ` 1 ` |

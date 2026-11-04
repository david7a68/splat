---
splat_schema: 2
splat_id: 8010b042-e75d-52f8-b45d-25eefc6b2c6f
splat_kind: rule
splat_key: dnd2024:rules/damage_and_healing_healing
splat_name: Healing
splat_origin: dnd2024:reference/pages/017
splat_sources:
- SRD 5.2.1, pp. 17, https://www.dndbeyond.com/srd
---

# Healing

## Source text

<!-- SRD page 17 -->

Hit Points can be restored by magic, such as the Cure Wounds spell or a Potion of Healing, or by a Short or Long Rest (see “Rules Glossary”).

### Knocking Out A Creature

When you would reduce a creature to 0 Hit Points with a melee attack, you can instead reduce the creature to 1 Hit Point and give it the Unconscious condition. It then starts a Short Rest, at the end of which that condition ends on it. The condition ends early if the creature regains any Hit Points or if someone takes an action to administer first aid to it, making a successful DC 10 Wisdom (Medicine) check.

When you receive healing, add the restored Hit Points to your current Hit Points. Your Hit Points can’t exceed your Hit Point maximum, so any Hit Points regained in excess of the maximum are lost. For example, if you receive 8 Hit Points of healing and have 14 Hit Points and a Hit Point maximum of 20, you regain 6 Hit Points, not 8.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/damage_and_healing_healing ` |
| ` implementation ` | ` dnd2024:rules_damage_and_healing_healing ` |

### Parameters

#### Healing

<!-- splat:yaml path=splat_rule/parameters/healing -->

```yaml
new_hp:
  op: min
  args:
  - var: subject.hp_max
  - op: add
    args:
    - var: subject.hp
    - var: healing
discard_excess: true
```

#### Knock out

<!-- splat:yaml path=splat_rule/parameters/knock_out -->

```yaml
optional: true
trigger:
  melee_attack: true
  would_reduce_target_hp_to: 0
replace_hp_with: 1
condition: unconscious
start_short_rest: true
ends:
- short_rest_finished
- any_hp_regained
- successful_first_aid
first_aid:
  action_cost: 1
  ability: wisdom
  skill: medicine
  dc: 10
```

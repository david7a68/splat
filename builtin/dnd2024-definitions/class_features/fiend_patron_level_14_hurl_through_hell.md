---
splat_schema: 1
splat_id: b1148ee6-a4c1-5ce0-b1df-5b1068550c28
splat_kind: rule
splat_key: dnd2024:class_features/fiend_patron_level_14_hurl_through_hell
splat_name: 'Level 14: Hurl Through Hell'
splat_origin:
  key: dnd2024:reference/pages/076
splat_sources:
- title: SRD 5.2.1
  locator: pp. 76
  url: https://www.dndbeyond.com/srd
---

# Level 14: Hurl Through Hell

## Source text

<!-- SRD page 76 -->

Once per turn when you hit a creature with an\
attack roll, you can try to instantly transport the\
target through the Lower Planes. The target must\
succeed on a Charisma saving throw against your\
spell save DC, or the target disappears and hurtles\
through a nightmare landscape. The target takes\
8d10 Psychic damage if it isn’t a Fiend, and it has the\
Incapacitated condition until the end of your next\
turn, when it returns to the space it previously oc-\
cupied or the nearest unoccupied space.\
Once you use this feature, you can’t use it again\
until you finish a Long Rest unless you expend a\
Pact Magic spell slot (no action required) to restore\
your use of it.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/fiend_patron_level_14_hurl_through_hell ` |
| ` implementation ` | ` dnd2024:class_features_fiend_patron_level_14_hurl_through_hell ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` trigger ` | ` attack_roll.hit_creature ` |

#### Limit

<!-- splat:values path=splat_rule/parameters/limit -->

| Field | Value |
| --- | --- |
| ` count ` | ` 1 ` |
| ` period ` | ` turn ` |

#### Save

<!-- splat:yaml path=splat_rule/parameters/save -->

```yaml
ability: charisma
dc:
  stat: spell_save_dc
success: no_effect
failure:
  banish_to: lower_planes
  damage:
    roll:
      dice:
        count: 8
        sides: 10
    type: psychic
    unless_creature_type: fiend
  condition: incapacitated
  until: source.next_turn_end
  return:
    destination: previous_space
    if_occupied: nearest_unoccupied_space
```

#### Uses

<!-- splat:yaml path=splat_rule/parameters/uses -->

```yaml
maximum: 1
recover:
  event: rest.long_rest_completed
  amount: all
```

#### Restore use

<!-- splat:values path=splat_rule/parameters/restore_use -->

| Field | Value |
| --- | --- |
| ` resource ` | ` pact_magic_spell_slot ` |
| ` amount ` | ` 1 ` |
| ` action ` | ` none ` |

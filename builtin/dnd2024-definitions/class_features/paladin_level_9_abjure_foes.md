---
splat_schema: 1
splat_id: be190777-faf7-5c4d-803e-bc4ac916d144
splat_kind: rule
splat_key: dnd2024:class_features/paladin_level_9_abjure_foes
splat_name: 'Level 9: Abjure Foes'
splat_origin:
  key: dnd2024:reference/pages/055
splat_sources:
- title: SRD 5.2.1
  locator: pp. 55
  url: https://www.dndbeyond.com/srd
---

# Level 9: Abjure Foes

## Source text

<!-- SRD page 55 -->

As a Magic action, you can expend one use of this\
class’s Channel Divinity to overwhelm foes with\
awe. As you present your Holy Symbol or weapon,\
you can target a number of creatures equal to your\
Charisma modifier (minimum of one creature) that\
you can see within 60 feet of yourself. Each target\
must succeed on a Wisdom saving throw or have the\
Frightened condition for 1 minute or until it takes\
any damage. While Frightened in this way, a target\
can do only one of the following on its turns: move,\
take an action, or take a Bonus Action.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/paladin_level_9_abjure_foes ` |
| ` implementation ` | ` dnd2024:class_features_paladin_level_9_abjure_foes ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` action ` | ` magic ` |
| ` present_item_any ` | ` ["holy_symbol", "weapon"] ` |

#### Resource cost

<!-- splat:values path=splat_rule/parameters/resource_cost -->

| Field | Value |
| --- | --- |
| ` id ` | ` paladin.channel_divinity ` |
| ` amount ` | ` 1 ` |

#### Targets

<!-- splat:yaml path=splat_rule/parameters/targets -->

```yaml
maximum:
  op: max
  args:
  - 1
  - var: actor.abilities.charisma.modifier
visible: true
range_feet: 60
type: creature
```

#### Save

<!-- splat:yaml path=splat_rule/parameters/save -->

```yaml
ability: wisdom
dc:
  op: add
  args:
  - 8
  - var: actor.abilities.charisma.modifier
  - var: actor.proficiency_bonus
failure:
  condition: frightened
  source: actor
  duration:
    minutes: 1
  ends_on:
  - target.damage_taken
  turn_activity_limit:
    choose_only_one:
    - move
    - action
    - bonus_action
```

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` paladin ` |
| ` minimum_level ` | ` 9 ` |

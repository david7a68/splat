---
splat_schema: 1
splat_id: 9faba9cc-ccd6-51b2-9585-817d222f1256
splat_kind: rule
splat_key: dnd2024:class_features/warrior_of_the_open_hand_level_3_open_hand_technique
splat_name: 'Level 3: Open Hand Technique'
splat_origin:
  key: dnd2024:reference/pages/052
splat_sources:
- title: SRD 5.2.1
  locator: pp. 52
  url: https://www.dndbeyond.com/srd
---

# Level 3: Open Hand Technique

## Source text

<!-- SRD page 52 -->

Whenever you hit a creature with an attack granted by your Flurry of Blows, you can impose one of the following effects on that target.

Addle. The target can’t make Opportunity Attacks until the start of its next turn.

Push. The target must succeed on a Strength saving throw or be pushed up to 15 feet away from you.

Topple. The target must succeed on a Dexterity saving throw or have the Prone condition.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/warrior_of_the_open_hand_level_3_open_hand_technique ` |
| ` implementation ` | ` dnd2024:class_features_warrior_of_the_open_hand_level_3_open_hand_technique ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` trigger ` | ` actor.flurry_of_blows_attack_hits_creature ` |
| ` optional ` | ` true ` |
| ` choose ` | ` 1 ` |

#### Save dc

<!-- splat:yaml path=splat_rule/parameters/save_dc -->

```yaml
op: add
args:
- 8
- var: actor.abilities.wisdom.modifier
- var: actor.proficiency_bonus
```

#### Options

<!-- splat:yaml path=splat_rule/parameters/options -->

```yaml
addle:
  opportunity_attacks: false
  end: target.next_turn.start
push:
  save: strength
  failure:
    push:
      maximum_feet: 15
      direction: away_from_actor
topple:
  save: dexterity
  failure:
    condition: prone
```

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` monk ` |
| ` minimum_level ` | ` 3 ` |

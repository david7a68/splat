---
splat_schema: 1
splat_id: 4ce22736-a27c-56e4-a2f5-8a49d15fa34b
splat_kind: rule
splat_key: dnd2024:class_features/hunter_level_7_defensive_tactics
splat_name: 'Level 7: Defensive Tactics'
splat_origin:
  key: dnd2024:reference/pages/061
splat_sources:
- title: SRD 5.2.1
  locator: pp. 61
  url: https://www.dndbeyond.com/srd
---

# Level 7: Defensive Tactics

## Source text

<!-- SRD page 61 -->

You gain one of the following feature options of your\
choice. Whenever you finish a Short or Long Rest,\
you can replace the chosen option with the other\
one.\
Escape the Horde. Opportunity Attacks have Dis-\
advantage against you.\
Multiattack Defense. When a creature hits you\
with an attack roll, that creature has Disadvantage\
on all other attack rolls against you this turn.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/hunter_level_7_defensive_tactics ` |
| ` implementation ` | ` dnd2024:class_features_hunter_level_7_defensive_tactics ` |

### Parameters

#### Choice

<!-- splat:yaml path=splat_rule/parameters/choice -->

```yaml
count: 1
options:
- id: escape_the_horde
  opportunity_attacks_against_self: disadvantage
- id: multiattack_defense
  trigger: creature.attack_roll_hits_self
  source_further_attacks_against_self: disadvantage
  until: current_turn_end
```

#### Replace choice

<!-- splat:values path=splat_rule/parameters/replace_choice -->

| Field | Value |
| --- | --- |
| ` events ` | ` ["rest.short_rest_completed", "rest.long_rest_completed"] ` |
| ` with ` | ` other_option ` |

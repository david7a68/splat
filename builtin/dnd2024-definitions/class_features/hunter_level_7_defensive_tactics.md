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

<!-- splat:values path=splat_rule/parameters/choice -->

| Field | Value |
| --- | --- |
| ` count ` | ` 1 ` |

##### Options

<!-- splat:table
path: splat_rule/parameters/choice/options
key: Variant
order:
- escape_the_horde
- multiattack_defense
columns:
  Variant:
    path: id
    type: symbol
  Opportunity attacks against self:
    path: opportunity_attacks_against_self
    type: symbol
    optional: true
  Trigger:
    path: trigger
    type: string
    optional: true
  Source further attacks against self:
    path: source_further_attacks_against_self
    type: symbol
    optional: true
  Until:
    path: until
    type: symbol
    optional: true
-->

| Variant | Opportunity attacks against self | Trigger | Source further attacks against self | Until |
| --- | --- | --- | --- | --- |
| ` Escape the horde ` | ` Disadvantage ` |  |  |  |
| ` Multiattack defense ` |  | ` creature.attack_roll_hits_self ` | ` Disadvantage ` | ` Current turn end ` |

#### Replace choice

<!-- splat:values path=splat_rule/parameters/replace_choice -->

| Field | Value |
| --- | --- |
| ` events ` | ` ["rest.short_rest_completed", "rest.long_rest_completed"] ` |
| ` with ` | ` other_option ` |

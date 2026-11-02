---
splat_schema: 1
splat_id: 68549fb5-f176-5a00-b8e4-286e0a53312b
splat_kind: rule
splat_key: dnd2024:class_features/college_of_lore_level_3_cutting_words
splat_name: 'Level 3: Cutting Words'
splat_origin:
  key: dnd2024:reference/pages/035
splat_sources:
- title: SRD 5.2.1
  locator: pp. 35
  url: https://www.dndbeyond.com/srd
---

# Level 3: Cutting Words

## Source text

<!-- SRD page 35 -->

You learn to use your wit to supernaturally distract, confuse, and otherwise sap the confidence and competence of others. When a creature that you can see within 60 feet of yourself makes a damage roll or succeeds on an ability check or attack roll, you can take a Reaction to expend one use of your Bardic Inspiration; roll your Bardic Inspiration die, and subtract the number rolled from the creature’s roll, reducing the damage or potentially turning the success into a failure.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/college_of_lore_level_3_cutting_words ` |
| ` implementation ` | ` dnd2024:class_features_college_of_lore_level_3_cutting_words ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` action ` | ` reaction ` |
| ` trigger_any ` | ` ["visible_creature.damage_rolled", "visible_creature.ability_check_succeeded", "visible_creature.attack_roll_succeeded"] ` |
| ` range_feet ` | ` 60 ` |
| ` modify ` | ` subtract_from_trigger_roll ` |
| ` recompute_success ` | ` true ` |

#### Resource cost

<!-- splat:values path=splat_rule/parameters/resource_cost -->

| Field | Value |
| --- | --- |
| ` id ` | ` bard.bardic_inspiration ` |
| ` amount ` | ` 1 ` |

#### Roll

<!-- splat:values path=splat_rule/parameters/roll -->

| Field | Value |
| --- | --- |
| ` var ` | ` actor.bardic_inspiration_die ` |

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` bard ` |
| ` minimum_level ` | ` 3 ` |

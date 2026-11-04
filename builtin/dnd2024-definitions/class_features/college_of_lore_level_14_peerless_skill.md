---
splat_schema: 2
splat_id: fea3cc65-4c97-55ba-aafc-13fe7e008ff5
splat_kind: rule
splat_key: dnd2024:class_features/college_of_lore_level_14_peerless_skill
splat_name: 'Level 14: Peerless Skill'
splat_origin: dnd2024:reference/pages/035
splat_sources:
- SRD 5.2.1, pp. 35, https://www.dndbeyond.com/srd
---

# Level 14: Peerless Skill

## Source text

<!-- SRD page 35 -->

When you make an ability check or attack roll and fail, you can expend one use of Bardic Inspiration; roll the Bardic Inspiration die, and add the number rolled to the d20, potentially turning a failure into a success. On a failure, the Bardic Inspiration isn’t expended.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/college_of_lore_level_14_peerless_skill ` |
| ` implementation ` | ` dnd2024:class_features_college_of_lore_level_14_peerless_skill ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` trigger ` | ` ["actor.ability_check_failed", "actor.attack_roll_failed"] ` |
| ` modify ` | ` add_to_d20 ` |
| ` recompute_success ` | ` true ` |
| ` refund_on_failure ` | ` true ` |

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
| ` minimum_level ` | ` 14 ` |

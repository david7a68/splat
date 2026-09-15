---
splat_schema: 1
splat_id: 43a06bd9-351e-5d13-b348-9e20aca36ec0
splat_kind: rule
splat_key: dnd2024:class_features/fighter_level_2_tactical_mind
splat_name: 'Level 2: Tactical Mind'
splat_origin:
  key: dnd2024:reference/pages/048
splat_sources:
- title: SRD 5.2.1
  locator: pp. 48
  url: https://www.dndbeyond.com/srd
---

# Level 2: Tactical Mind

## Source text

<!-- SRD page 48 -->

You have a mind for tactics on and off the battle-\
field. When you fail an ability check, you can expend\
a use of your Second Wind to push yourself toward\
success. Rather than regaining Hit Points, you roll\
1d10 and add the number rolled to the ability check,\
potentially turning it into a success. If the check still\
fails, this use of Second Wind isn’t expended.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/fighter_level_2_tactical_mind ` |
| ` implementation ` | ` dnd2024:class_features_fighter_level_2_tactical_mind ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` trigger ` | ` actor.ability_check_failed ` |
| ` modify ` | ` add_to_d20 ` |
| ` recompute_success ` | ` true ` |
| ` refund_on_failure ` | ` true ` |
| ` healing ` | ` false ` |

#### Resource cost

<!-- splat:values path=splat_rule/parameters/resource_cost -->

| Field | Value |
| --- | --- |
| ` id ` | ` fighter.second_wind ` |
| ` amount ` | ` 1 ` |

#### Roll

<!-- splat:values path=splat_rule/parameters/roll -->

| Field | Value |
| --- | --- |
| ` count ` | ` 1 ` |
| ` sides ` | ` 10 ` |

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` fighter ` |
| ` minimum_level ` | ` 2 ` |

---
splat_schema: 1
splat_id: 74a0958a-f9cf-5a51-af2f-783d53c7db38
splat_kind: rule
splat_key: dnd2024:rules/create_your_character_choose_starting_equipment
splat_name: Choose Starting Equipment
splat_origin:
  key: dnd2024:reference/pages/020
splat_sources:
- title: SRD 5.2.1
  locator: pp. 20
  url: https://www.dndbeyond.com/srd
---

# Choose Starting Equipment

## Source text

<!-- SRD page 20 -->

Your background and class both provide starting equipment. Any coins that you gain at this step can be immediately spent on equipment from “Equipment.”

Record your chosen equipment on your character sheet. Equipment is described in “Equipment,” but for now you can just write it all down and look up the specifics in “Equipment” later. Note any coins you have left after purchasing your equipment.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/create_your_character_choose_starting_equipment ` |
| ` implementation ` | ` dnd2024:rules_create_your_character_choose_starting_equipment ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipment_sources ` | ` ["class", "background"] ` |
| ` record ` | ` ["chosen_equipment", "remaining_coins"] ` |

#### Starting coins

<!-- splat:values path=splat_rule/parameters/starting_coins -->

| Field | Value |
| --- | --- |
| ` may_spend_immediately ` | ` true ` |
| ` purchases ` | ` equipment_catalog ` |
| ` deduct_cost ` | ` true ` |

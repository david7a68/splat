---
splat_schema: 1
splat_id: c7026ed5-95d1-58f3-ad3f-d2a9bdb74532
splat_kind: rule
splat_key: dnd2024:rules/parts_of_a_stat_block_equipping_a_monster_with_other_items
splat_name: Equipping a Monster with Other Items
splat_origin:
  key: dnd2024:reference/pages/255
splat_sources:
- title: SRD 5.2.1
  locator: pp. 255, 256
  url: https://www.dndbeyond.com/srd
---

# Equipping a Monster with Other Items

## Source text

<!-- SRD page 255 -->

You may equip monsters with additional gear however you like, using the items in “Equipment” for inspiration. You decide how much of a monster’s equipment is recoverable after the creature is slain and whether any of that equipment is still usable.

Beware of giving a monster combat-oriented magic items, since those might alter the monster’s Challenge Rating. If you do give a monster a magic item, the monster can have Attunement with magic items as noted in “Equipment.” A monster with a class tag after its creature type is considered a member of that class for Attunement purposes.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/parts_of_a_stat_block_equipping_a_monster_with_other_items ` |
| ` implementation ` | ` dnd2024:rules_parts_of_a_stat_block_equipping_a_monster_with_other_items ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` class_tag_qualifies_for_class_attunement ` | ` true ` |
| ` combat_magic_items_may_change_challenge_rating ` | ` true ` |

#### Additional equipment

<!-- splat:values path=splat_rule/parameters/additional_equipment -->

| Field | Value |
| --- | --- |
| ` authority ` | ` gm ` |
| ` selection ` | ` item_list ` |

#### After monster death

<!-- splat:values path=splat_rule/parameters/after_monster_death -->

| Field | Value |
| --- | --- |
| ` recoverable_equipment ` | ` gm_choice ` |
| ` usable_equipment ` | ` gm_choice ` |

#### Magic item attunement

<!-- splat:values path=splat_rule/parameters/magic_item_attunement -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:glossary/attunement ` |

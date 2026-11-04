---
splat_schema: 2
splat_id: 24e141db-586b-563c-9d22-38b7d1148a16
splat_kind: rule
splat_key: dnd2024:rules/parts_of_a_stat_block_gear
splat_name: Gear
splat_origin: dnd2024:reference/pages/255
splat_sources:
- SRD 5.2.1, pp. 255, https://www.dndbeyond.com/srd
---

# Gear

## Source text

<!-- SRD page 255 -->

Monsters have proficiency with their equipment. If a monster has equipment that can be given away or retrieved, the items are listed in the Gear entry. The monster’s stat block might include special flourishes that happen when the monster uses an item, and the stat block might ignore the rules in “Equipment” for that item. When used by someone else, a retrievable item uses its “Equipment” rules, ignoring any special flourishes in the stat block.

The Gear entry doesn’t necessarily list all of a monster’s equipment. For example, a monster that wears clothes is assumed to be dressed appropriately, and those clothes aren’t in this entry.

Equipment mentioned outside the Gear entry is considered to be supernatural or highly specialized, and it is unusable when the monster is defeated.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/parts_of_a_stat_block_gear ` |
| ` implementation ` | ` dnd2024:rule_parts_of_a_stat_block_gear ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` monster_proficient_with_equipment ` | ` true ` |
| ` retrievable_items_source ` | ` gear_entry ` |
| ` monster_special_item_rules_apply_only_to_original_monster ` | ` true ` |
| ` other_user_uses ` | ` equipment_definition_rules ` |
| ` gear_list_not_exhaustive_for_incidental_clothing ` | ` true ` |

#### Equipment outside gear entry

<!-- splat:values path=splat_rule/parameters/equipment_outside_gear_entry -->

| Field | Value |
| --- | --- |
| ` supernatural_or_specialized ` | ` true ` |
| ` usable_after_monster_defeat ` | ` false ` |

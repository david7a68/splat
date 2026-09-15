---
splat_schema: 1
splat_id: 24e141db-586b-563c-9d22-38b7d1148a16
splat_kind: rule
splat_key: dnd2024:rules/parts_of_a_stat_block_gear
splat_name: Gear
splat_origin:
  key: dnd2024:reference/pages/255
splat_sources:
- title: SRD 5.2.1
  locator: pp. 255
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/parts_of_a_stat_block_gear
  parameters:
    monster_proficient_with_equipment: true
    retrievable_items_source: gear_entry
    monster_special_item_rules_apply_only_to_original_monster: true
    other_user_uses: equipment_definition_rules
    gear_list_not_exhaustive_for_incidental_clothing: true
    equipment_outside_gear_entry:
      supernatural_or_specialized: true
      usable_after_monster_defeat: false
  implementation: dnd2024:rule_parts_of_a_stat_block_gear
splat_contracts:
- id: dnd2024:rule_parts_of_a_stat_block_gear
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
  outputs:
  - monster_parameters
  events: []
  capabilities:
  - monsters
  - equipment
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 255
  engine_gaps: &id001
  - dnd2024:rule_parts_of_a_stat_block_gear
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: 43d286ac39275d2d989de91326165c57e7e0161faf0e39f42da1c536e0e647e8
  parent_name: Parts of a Stat Block
  line_span:
    first: 11031
    count: 17
splat_clauses:
- key: source_rules
  text: '# Gear Monsters have proficiency with their equipment. If a monster has equipment that can be
    given away or retrieved, the items are listed in the Gear entry. The monster’s stat block might include
    special flourishes that happen when the monster uses an item, and the stat block might ignore the
    rules in “Equipment” for that item. When used by someone else, a retrievable item uses its “Equipment”
    rules, ignoring any special flourishes in the stat block. The Gear entry doesn’t necessarily list
    all of a monster’s equipment. For example, a monster that wears clothes is assumed to be dressed appropriately,
    and those clothes aren’t in this entry. Equipment mentioned outside the Gear entry is considered to
    be supernatural or highly specialized, and it is unusable when the monster is defeated.'
  sha256: e6524331e02bacd7167bd9bb6d019bb43d0b9217919600f5f78eca1dfe824303
  status: contract_specified
splat_requires_mechanics: *id001
---

# Gear

<!-- SRD page 255 -->

Monsters have proficiency with their equipment. If\
a monster has equipment that can be given away or\
retrieved, the items are listed in the Gear entry. The\
monster’s stat block might include special flour-\
ishes that happen when the monster uses an item,\
and the stat block might ignore the rules in “Equip-\
ment” for that item. When used by someone else, a\
retrievable item uses its “Equipment” rules, ignor-\
ing any special flourishes in the stat block.\
The Gear entry doesn’t necessarily list all of a\
monster’s equipment. For example, a monster that\
wears clothes is assumed to be dressed appropri-\
ately, and those clothes aren’t in this entry.\
Equipment mentioned outside the Gear entry is\
considered to be supernatural or highly specialized,\
and it is unusable when the monster is defeated.\

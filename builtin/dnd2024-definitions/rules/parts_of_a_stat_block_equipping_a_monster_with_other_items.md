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
splat_rule:
  mechanic: dnd2024:rules/parts_of_a_stat_block_equipping_a_monster_with_other_items
  parameters:
    additional_equipment:
      authority: gm
      selection: item_list
    after_monster_death:
      recoverable_equipment: gm_choice
      usable_equipment: gm_choice
    magic_item_attunement:
      key: dnd2024:glossary/attunement
    class_tag_qualifies_for_class_attunement: true
    combat_magic_items_may_change_challenge_rating: true
  implementation: dnd2024:rules_parts_of_a_stat_block_equipping_a_monster_with_other_items
splat_contracts:
- id: dnd2024:rules_parts_of_a_stat_block_equipping_a_monster_with_other_items
  inputs:
    monster:
      type: participant
      required: true
    equipment:
      type: item_list
      required: true
    recovery_choices:
      required: false
      type: resolution
  outputs:
  - equipment
  - recoverable_loot
  - attunement_eligibility
  events: []
  capabilities:
  - monster_equipment
  - attunement
  - loot
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 255
  - 256
  engine_gaps: &id001
  - dnd2024:rules_parts_of_a_stat_block_equipping_a_monster_with_other_items
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 359b99578acc14999ca361fc699ca87c27b8a27c3b7f5faedf960a6d8f3c362f
  parent_name: Parts of a Stat Block
  line_span:
    first: 11051
    count: 13
splat_clauses:
- key: source_rules
  text: '# Equipping a Monster with Other Items You may equip monsters with additional gear however you
    like, using the items in “Equipment” for inspiration. You decide how much of a monster’s equipment
    is recoverable after the creature is slain and whether any of that equipment is still usable. Beware
    of giving a monster combat-oriented magic items, since those might alter the monster’s Challenge Rating.
    If you do give a monster a magic item, the monster can have Attunement with magic items as noted in
    “Equipment.” A monster with a class tag after its creature type is considered a member of that class
    for Attunement purposes.'
  sha256: f376f5fc39544f58e64861c030a685651eebedef9235266e01389284a7e630e2
  status: contract_specified
splat_requires_mechanics: *id001
---

# Equipping a Monster with Other Items

<!-- SRD page 255 -->

You may equip monsters with additional gear how-\
ever you like, using the items in “Equipment” for\
inspiration. You decide how much of a monster’s\
equipment is recoverable after the creature is slain\
and whether any of that equipment is still usable.\
Beware of giving a monster combat-oriented\
magic items, since those might alter the monster’s\
Challenge Rating. If you do give a monster a magic\
item, the monster can have Attunement with magic\
items as noted in “Equipment.” A monster with\

<!-- SRD page 256 -->

a class tag after its creature type is considered a\
member of that class for Attunement purposes.\

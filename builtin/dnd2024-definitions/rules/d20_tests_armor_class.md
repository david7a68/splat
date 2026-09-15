---
splat_schema: 1
splat_id: 0c74fdb5-106a-509f-a01c-2496a34593be
splat_kind: rule
splat_key: dnd2024:rules/d20_tests_armor_class
splat_name: Armor Class
splat_origin:
  key: dnd2024:reference/pages/007
splat_sources:
- title: SRD 5.2.1
  locator: pp. 7
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/d20_tests_armor_class
  parameters:
    default_base_ac:
      op: add
      left: 10
      right: dexterity_modifier
    base_formula_choice:
      count: 1
      from: available_ac_formulas
    maximum_simultaneous_base_formulas: 1
    modifiers_from:
    - armor
    - magic_items
    - spells
    - other_rules
  implementation: dnd2024:rules_d20_tests_armor_class
splat_contracts:
- id: dnd2024:rules_d20_tests_armor_class
  inputs:
    actor:
      type: participant
      required: true
    item:
      type: item_instance
      required: true
    choice:
      type: choice
      required: true
  outputs:
  - item_attributes
  - actor_modifiers
  events: []
  capabilities:
  - equipment
  - armor_class
  - crafting
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 7
  engine_gaps: &id001
  - dnd2024:rules_d20_tests_armor_class
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: 67e381fb0553277e13cbe33187105ab9d7cf4a5a128d57d94465c2cd14cf3660
  parent_name: D20 Tests
  line_span:
    first: 253
    count: 17
splat_clauses:
- key: source_rules
  text: '# Armor Class A creature’s Armor Class represents how well the creature avoids being wounded
    in combat. The AC of a character is determined at character creation (see “Character Creation”), whereas
    the AC of a monster appears in its stat block. Calculating AC. All creatures start with the same base
    AC calculation: Base AC = 10 + the creature’s Dexterity modifier A creature’s AC can then be modified
    by armor, magic items, spells, and more. Only One Base AC. Some spells and class features give characters
    a different way to calculate their AC. A character with multiple features that give different ways
    to calculate AC must choose which one to use; only one base calculation can be in effect for a creature.'
  sha256: 4038171b049b57d20dd212073ecd9f035e88a5602b19facf8ba2c8b0cc446b9e
  status: contract_specified
splat_requires_mechanics: *id001
---

# Armor Class

<!-- SRD page 7 -->

A creature’s Armor Class represents how well the\
creature avoids being wounded in combat. The AC\
of a character is determined at character creation\
(see “Character Creation”), whereas the AC of a\
monster appears in its stat block.\
Calculating AC. All creatures start with the same\
base AC calculation:\
Base AC = 10 + the creature’s Dexterity modifier\
A creature’s AC can then be modified by armor,\
magic items, spells, and more.\
Only One Base AC. Some spells and class features\
give characters a different way to calculate their AC.\
A character with multiple features that give differ-\
ent ways to calculate AC must choose which one to\
use; only one base calculation can be in effect for a\
creature.\

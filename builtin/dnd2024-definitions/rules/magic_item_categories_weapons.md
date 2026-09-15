---
splat_schema: 1
splat_id: 1d55496c-347e-5683-9e18-a8d1419b207f
splat_kind: rule
splat_key: dnd2024:rules/magic_item_categories_weapons
splat_name: Weapons
splat_origin:
  key: dnd2024:reference/pages/205
splat_sources:
- title: SRD 5.2.1
  locator: pp. 205
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/magic_item_categories_weapons
  parameters:
    weapon_type:
      if_specified: item_definition
      otherwise: choose_or_randomize_equipment_weapon_type
    magic_weapon_with_ammunition_property:
      fired_ammunition_counts_as_magical_weapon: true
  implementation: dnd2024:rules_magic_item_categories_weapons
splat_contracts:
- id: dnd2024:rules_magic_item_categories_weapons
  inputs:
    weapon:
      type: item_instance
      required: true
    weapon_type:
      required: false
      type: choice
  outputs:
  - weapon_type
  - ammunition_magic_status
  events: []
  capabilities:
  - magic_items
  - weapons
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 205
  engine_gaps: &id001
  - dnd2024:rules_magic_item_categories_weapons
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 2cdb0513de46cc4a01dff1e6e567844ce62b7c7f4c53390bada792093433e5b8
  parent_name: Magic Item Categories
  line_span:
    first: 10514
    count: 11
splat_clauses:
- key: source_rules
  text: '# Weapons A magic weapon is typically a magical version of a weapon from “Equipment.” Some magic
    weapons specify the type of weapon they are in their descriptions, such as a Longsword or Longbow.
    If no weapon type is specified, you may choose the type or determine it randomly. Ammunition. If a
    magic weapon has the Ammunition property, ammunition fired from it is considered magical for the purpose
    of any rule that cares whether a weapon is magical or not.'
  sha256: 229bcce04e8f8923a2784a007efa9918a1e57bbb3e1003bba550597055017e8c
  status: contract_specified
splat_requires_mechanics: *id001
---

# Weapons

<!-- SRD page 205 -->

A magic weapon is typically a magical version of a\
weapon from “Equipment.” Some magic weapons\
specify the type of weapon they are in their de-\
scriptions, such as a Longsword or Longbow. If no\
weapon type is specified, you may choose the type\
or determine it randomly.\
Ammunition. If a magic weapon has the Ammu-\
nition property, ammunition fired from it is consid-\
ered magical for the purpose of any rule that cares\
whether a weapon is magical or not.\

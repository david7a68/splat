---
splat_schema: 1
splat_id: 8de03d4f-772d-597c-a05a-098df13a53a9
splat_kind: rule
splat_key: dnd2024:rules/magic_items_wearing_and_wielding_items
splat_name: Wearing and Wielding Items
splat_origin:
  key: dnd2024:reference/pages/103
splat_sources:
- title: SRD 5.2.1
  locator: pp. 103
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/magic_items_wearing_and_wielding_items
  parameters:
    required_equipment_positions:
      boots: feet
      gloves: hands
      hat: head
      helmet: head
      ring: finger
      armor: donned
      shield: strapped_to_arm
      cloak: shoulders
      weapon: held
    magic_garment_fit_default: any_size_or_build
    item_specific_overrides: true
  implementation: dnd2024:rules_magic_items_wearing_and_wielding_items
splat_contracts:
- id: dnd2024:rules_magic_items_wearing_and_wielding_items
  inputs:
    subject:
      type: participant
      required: true
    item:
      type: item_instance
      required: true
  outputs:
  - magic_property_access
  events: []
  capabilities:
  - equipment
  - magic_items
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 103
  engine_gaps: &id001
  - dnd2024:rules_magic_items_wearing_and_wielding_items
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 3a83ddac11ac7cb90b7838f77f69716d1ca229e976df6baae85ca347b430b785
  parent_name: Magic Items
  line_span:
    first: 8935
    count: 12
splat_clauses:
- key: source_rules
  text: '# Wearing and Wielding Items Using a magic item’s properties might mean wearing or wielding it.
    A magic item meant to be worn must be donned in the intended fashion: boots go on feet, gloves on
    hands, hats and helmets on a head, and rings on a finger. Magic armor must be donned, a Shield strapped
    to the arm, a cloak fastened about the shoulders. A weapon must be held. In most cases, a magic item
    that’s meant to be worn can fit a creature regardless of size or build. Magic garments are made to
    be easily adjustable, or they magically adjust themselves to the wearer.'
  sha256: 7ac489c736f38bf9f6eedb0b25207ec8e6c20275a64a97d3dcf086ed7834cfa0
  status: contract_specified
splat_requires_mechanics: *id001
---

# Wearing and Wielding Items

<!-- SRD page 103 -->

Using a magic item’s properties might mean wear-\
ing or wielding it. A magic item meant to be worn\
must be donned in the intended fashion: boots go on\
feet, gloves on hands, hats and helmets on a head,\
and rings on a finger. Magic armor must be donned,\
a Shield strapped to the arm, a cloak fastened about\
the shoulders. A weapon must be held.\
In most cases, a magic item that’s meant to be\
worn can fit a creature regardless of size or build.\
Magic garments are made to be easily adjustable, or\
they magically adjust themselves to the wearer.\

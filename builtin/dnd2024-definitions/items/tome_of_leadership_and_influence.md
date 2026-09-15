---
splat_schema: 1
splat_id: 9aaddf99-9737-5c85-9fb7-14adddc17aa0
splat_kind: item
splat_key: dnd2024:items/tome_of_leadership_and_influence
splat_name: Tome of Leadership and Influence
splat_origin:
  key: dnd2024:reference/magic_item/tome_of_leadership_and_influence
splat_sources:
- title: SRD 5.2.1
  locator: pp. 249, 250
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: very_rare
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/tome_of_leadership_and_influence
  implementation: dnd2024:item_tome_of_leadership_and_influence
  parameters:
    study:
      total_hours: 48
      within_days: 6
      practice_required: true
    completion:
      ability: charisma
      increase: 2
      maximum: 30
      item_magic: false
    magic_recovery:
      amount: 100
      unit: years
    attunement_required: false
splat_contracts:
- id: dnd2024:item_tome_of_leadership_and_influence
  inputs:
    actor:
      type: participant
      required: true
    item:
      type: item_instance
      required: true
    targets:
      required: false
      type: participant_list
    objects:
      required: false
      type: object_list
    destination:
      required: false
      type: location
    selection:
      required: false
      type: choice
    context:
      required: false
      type: resolution
  outputs:
  - effects
  - item_state_changes
  events:
  - study.progressed
  - time.elapsed
  capabilities:
  - item_effects
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 51a55a17419d18062a299807c93bc668c303abd4662b66e421b3598ff2cdce64
  source_path: builtin/srd-5.2.1/magic_item/tome_of_leadership_and_influence.md
  pages:
  - 249
  - 250
  engine_gaps: &id001
  - dnd2024:item_tome_of_leadership_and_influence
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_3.md
  specification_sha256: 8aecec32e88849197d875db080811d1418e902eadc510175d43ed0066e76fa04
splat_clauses:
- key: source_rules
  text: '# Tome of Leadership and Influence Wondrous Item, Very Rare This book contains guidelines for
    influencing and charming others, and its words are charged with magic. If you spend 48 hours over
    a period of 6 days or fewer studying the book’s contents and practicing its guidelines, your Charisma
    increases by 2, to a maximum of 30. The manual then loses its magic but regains it in a century.'
  sha256: 60e649bef8c5d6f5704858543524e63955db423552f884feca193b0ccd870baf
  status: contract_specified
splat_requires_mechanics: *id001
---

# Tome of Leadership and Influence

<!-- SRD page 249 -->

Wondrous Item, Very Rare\
This book contains guidelines for influencing and\
charming others, and its words are charged with\
magic. If you spend 48 hours over a period of 6 days\
or fewer studying the book’s contents and practic-\
ing its guidelines, your Charisma increases by 2, to\

<!-- SRD page 250 -->

a maximum of 30. The manual then loses its magic\
but regains it in a century.\

Original layout: [[../../srd-5.2.1/pages/249|page 249]], [[../../srd-5.2.1/pages/250|page 250]].

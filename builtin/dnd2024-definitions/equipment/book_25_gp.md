---
splat_schema: 1
splat_id: 917193ad-7c25-5052-9e02-9827e0ec2427
splat_kind: item
splat_key: dnd2024:equipment/book_25_gp
splat_name: Book (25 GP)
splat_origin:
  key: dnd2024:reference/pages/096
splat_sources:
- title: SRD 5.2.1
  locator: pp. 96
  url: https://www.dndbeyond.com/srd
splat_item:
  category: adventuring_gear
  equipment:
    attunement: none
  price:
    value: 25
    unit: gp
  mass:
    value: 5.0
    unit: lb
splat_rule:
  mechanic: dnd2024:equipment/book_25_gp
  implementation: dnd2024:equipment_book_25_gp
  parameters:
    contents_kind:
      choice:
      - fiction
      - nonfiction
    consult:
      requirements:
        kind: nonfiction
        accurate: true
        topic_matches_check: true
      bonus: 5
      roll: ability_check
      ability: intelligence
      skills:
      - arcana
      - history
      - nature
      - religion
    mass:
      value: 5.0
      unit: lb
splat_contracts:
- id: dnd2024:equipment_book_25_gp
  inputs:
    actor:
      type: participant
      required: true
    item:
      type: item_instance
      required: true
    event:
      required: false
      type: event
    choices:
      required: false
      type: choice
  outputs:
  - item_properties
  - conditional_modifiers
  events: []
  capabilities:
  - equipment
  - conditional_modifiers
splat_conversion:
  status: specified
  category: equipment
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 96
  engine_gaps: &id001
  - dnd2024:equipment_book_25_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 8ea3199992e6d02f43ec71307780ce698b156ff1f332c6d9ac48d9c92860e155
  line_span:
    first: 8308
    count: 5
splat_clauses:
- key: source_rules
  text: '# Book (25 GP) A Book contains fiction or nonfiction. If you consult an accurate nonfiction Book
    about its topic, you gain a +5 bonus to Intelligence (Arcana, History, Nature, or Religion) checks
    you make about that topic.'
  sha256: f3b0368b2b508d887f232b0585f21adedd7b34a4b24d9f09452a47dd34d62f80
  status: contract_specified
splat_requires_mechanics: *id001
---

# Book (25 GP)

<!-- SRD page 96 -->

A Book contains fiction or nonfiction. If you consult\
an accurate nonfiction Book about its topic, you gain\
a +5 bonus to Intelligence (Arcana, History, Nature,\
or Religion) checks you make about that topic.\

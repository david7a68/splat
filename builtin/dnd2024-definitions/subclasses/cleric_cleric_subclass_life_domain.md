---
splat_schema: 1
splat_id: bf2323c0-8261-5c65-8ab4-dc2ad40e1c6c
splat_kind: rule_group
splat_key: dnd2024:subclasses/cleric_cleric_subclass_life_domain
splat_name: 'Cleric Subclass: Life Domain'
splat_origin:
  key: dnd2024:reference/pages/040
splat_sources:
- title: SRD 5.2.1
  locator: pp. 40
  url: https://www.dndbeyond.com/srd
splat_rule_group:
  category: subclass
  members:
  - key: dnd2024:class_features/life_domain_level_17_supreme_healing
  - key: dnd2024:class_features/life_domain_level_3_disciple_of_life
  - key: dnd2024:class_features/life_domain_level_3_life_domain_spells
  - key: dnd2024:class_features/life_domain_level_3_preserve_life
  - key: dnd2024:class_features/life_domain_level_6_blessed_healer
splat_rule:
  mechanic: dnd2024:subclasses/cleric_cleric_subclass_life_domain
  implementation: dnd2024:subclasses_cleric_cleric_subclass_life_domain
  parameters:
    class: cleric
    subclass: life_domain
    feature_grants:
    - feature:
        key: dnd2024:class_features/life_domain_level_17_supreme_healing
      minimum_class_level: 17
    - feature:
        key: dnd2024:class_features/life_domain_level_3_disciple_of_life
      minimum_class_level: 3
    - feature:
        key: dnd2024:class_features/life_domain_level_3_life_domain_spells
      minimum_class_level: 3
    - feature:
        key: dnd2024:class_features/life_domain_level_3_preserve_life
      minimum_class_level: 3
    - feature:
        key: dnd2024:class_features/life_domain_level_6_blessed_healer
      minimum_class_level: 6
splat_contracts:
- id: dnd2024:subclasses_cleric_cleric_subclass_life_domain
  inputs:
    actor:
      type: participant
      required: true
  outputs:
  - feature_state
  events:
  - character.subclass_selected
  - character.class_level_gained
  capabilities:
  - class_progression
splat_conversion:
  status: specified
  category: subclass
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 40
  engine_gaps: &id001
  - dnd2024:subclasses_cleric_cleric_subclass_life_domain
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: 99dcc4f0e807a18ffd827e580d0cb83a6ae6b2ca4afa619bb4ec7a1fde9ac212
  parent_name: Cleric
  line_span:
    first: 3285
    count: 13
splat_clauses:
- key: source_rules
  text: '# Cleric Subclass: Life Domain Soothe the Hurts of the World The Life Domain focuses on the positive
    energy that helps sustain all life in the multiverse. Clerics who tap into this domain are masters
    of healing, using that life force to cure many hurts. Existence itself relies on the positive energy
    associated with this domain, so a Cleric of almost any religious tradition might choose it. This domain
    is particularly associated with agricultural deities, gods of healing or endurance, and gods of home
    and community. Religious orders of healing also seek the magic of this domain.'
  sha256: c536dde7f8fbd14c55d99169bef9eda103659de5da9f207173dcc934827f8dc9
  status: contract_specified
splat_requires_mechanics: *id001
---

# Cleric Subclass: Life Domain

<!-- SRD page 40 -->

Soothe the Hurts of the World\
The Life Domain focuses on the positive energy that\
helps sustain all life in the multiverse. Clerics who\
tap into this domain are masters of healing, using\
that life force to cure many hurts.\
Existence itself relies on the positive energy as-\
sociated with this domain, so a Cleric of almost any\
religious tradition might choose it. This domain is\
particularly associated with agricultural deities,\
gods of healing or endurance, and gods of home and\
community. Religious orders of healing also seek\
the magic of this domain.\

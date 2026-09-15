---
splat_schema: 1
splat_id: 3a669937-792d-537d-bc50-56c3344e7a1f
splat_kind: rule_group
splat_key: dnd2024:subclasses/bard_college_of_lore
splat_name: College of Lore
splat_origin:
  key: dnd2024:reference/pages/035
splat_sources:
- title: SRD 5.2.1
  locator: pp. 35
  url: https://www.dndbeyond.com/srd
splat_rule_group:
  category: subclass
splat_rule:
  mechanic: dnd2024:subclasses/bard_college_of_lore
  implementation: dnd2024:subclasses_bard_college_of_lore
  parameters:
    class: bard
    minimum_class_level: 3
    feature_grants:
    - class_level: 14
      definition:
        key: dnd2024:class_features/college_of_lore_level_14_peerless_skill
    - class_level: 3
      definition:
        key: dnd2024:class_features/college_of_lore_level_3_bonus_proficiencies
    - class_level: 3
      definition:
        key: dnd2024:class_features/college_of_lore_level_3_cutting_words
    - class_level: 6
      definition:
        key: dnd2024:class_features/college_of_lore_level_6_magical_discoveries
    grant_eligible_features:
      'on':
      - subclass.selected
      - character.level_gained
      comparison: class_level_at_least_threshold
      persistent: true
splat_contracts:
- id: dnd2024:subclasses_bard_college_of_lore
  inputs:
    actor:
      type: participant
      required: true
    selection:
      required: false
      type: choice
  outputs:
  - feature_grants
  events:
  - subclass.selected
  - character.level_gained
  capabilities:
  - character_features
  - proficiencies
splat_conversion:
  status: specified
  category: subclass
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 35
  engine_gaps: &id001
  - dnd2024:subclasses_bard_college_of_lore
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 0e12f88aadd8b136b28ae4f4b6db6111ed3583d22e37b05eb23f223ba80a9e1a
  parent_name: Bard
  line_span:
    first: 2872
    count: 10
splat_clauses:
- key: source_rules
  text: '# College of Lore Plumb the Depths of Magical Knowledge Bards of the College of Lore collect
    spells and secrets from diverse sources, such as scholarly tomes, mystical rites, and peasant tales.
    The college’s members gather in libraries and universities to share their lore with one another. They
    also meet at festivals or affairs of state, where they can expose corruption, unravel lies, and poke
    fun at self-important figures of authority.'
  sha256: 534df38a0b7afb91b49baabe7eceadc584a0cd976a017b35ed707219b71bfc93
  status: contract_specified
splat_requires_mechanics: *id001
---

# College of Lore

<!-- SRD page 35 -->

Plumb the Depths of Magical Knowledge\
Bards of the College of Lore collect spells and se-\
crets from diverse sources, such as scholarly tomes,\
mystical rites, and peasant tales. The college’s mem-\
bers gather in libraries and universities to share\
their lore with one another. They also meet at festi-\
vals or affairs of state, where they can expose cor-\
ruption, unravel lies, and poke fun at self-important\
figures of authority.\

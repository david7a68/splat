---
splat_schema: 1
splat_id: ab33196c-b0ce-5509-b8f0-134faa497055
splat_kind: rule_group
splat_key: dnd2024:subclasses/warlock_warlock_subclass_fiend_patron
splat_name: 'Warlock Subclass: Fiend Patron'
splat_origin:
  key: dnd2024:reference/pages/076
splat_sources:
- title: SRD 5.2.1
  locator: pp. 76
  url: https://www.dndbeyond.com/srd
splat_rule_group:
  category: subclass
splat_rule:
  mechanic: dnd2024:subclasses/warlock_warlock_subclass_fiend_patron
  implementation: dnd2024:subclasses_warlock_warlock_subclass_fiend_patron
  parameters:
    class: warlock
    minimum_class_level: 3
    feature_grants:
    - class_level: 10
      definition:
        key: dnd2024:class_features/fiend_patron_level_10_fiendish_resilience
    - class_level: 14
      definition:
        key: dnd2024:class_features/fiend_patron_level_14_hurl_through_hell
    - class_level: 3
      definition:
        key: dnd2024:class_features/fiend_patron_level_3_dark_one_s_blessing
    - class_level: 3
      definition:
        key: dnd2024:class_features/fiend_patron_level_3_fiend_spells
    - class_level: 6
      definition:
        key: dnd2024:class_features/fiend_patron_level_6_dark_one_s_own_luck
    grant_eligible_features:
      'on':
      - subclass.selected
      - character.level_gained
      comparison: class_level_at_least_threshold
      persistent: true
splat_contracts:
- id: dnd2024:subclasses_warlock_warlock_subclass_fiend_patron
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
  - spellcasting
splat_conversion:
  status: specified
  category: subclass
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 76
  engine_gaps: &id001
  - dnd2024:subclasses_warlock_warlock_subclass_fiend_patron
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: c09e850c3c5ac19b6f9af0b4c4aa3c80ef4ea39a3ac29066cb44d6d5a931ca26
  parent_name: Warlock
  line_span:
    first: 6471
    count: 9
splat_clauses:
- key: source_rules
  text: '# Warlock Subclass: Fiend Patron Make a Deal with the Lower Planes Your pact draws on the Lower
    Planes, the realms of perdition. You might forge a bargain with a demon lord, an archdevil, or another
    fiend that is especially mighty. That patron’s aims are evil—the corruption or destruction of all
    things, ultimately including you—and your path is defined by the extent to which you strive against
    those aims.'
  sha256: 5a8cf2d7d76c7bb6f49a4743b0cac11ac8ec1eaab0d8995711faa25c7951c3f0
  status: contract_specified
splat_requires_mechanics: *id001
---

# Warlock Subclass: Fiend Patron

<!-- SRD page 76 -->

Make a Deal with the Lower Planes\
Your pact draws on the Lower Planes, the realms of\
perdition. You might forge a bargain with a demon\
lord, an archdevil, or another fiend that is especially\
mighty. That patron’s aims are evil—the corruption\
or destruction of all things, ultimately including\
you—and your path is defined by the extent to\
which you strive against those aims.\

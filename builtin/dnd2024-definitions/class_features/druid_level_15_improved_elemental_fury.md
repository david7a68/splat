---
splat_schema: 1
splat_id: 51f2a3e4-a78f-5c14-8f17-5186074a5ebe
splat_kind: rule
splat_key: dnd2024:class_features/druid_level_15_improved_elemental_fury
splat_name: 'Level 15: Improved Elemental Fury'
splat_origin:
  key: dnd2024:reference/pages/043
splat_sources:
- title: SRD 5.2.1
  locator: pp. 43
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/druid_level_15_improved_elemental_fury
  parameters:
    by_elemental_fury_choice:
      potent_spellcasting:
        spell_class: druid
        spell_level: 0
        minimum_base_range_feet: 10
        range_bonus_feet: 300
      primal_strike:
        replace_extra_damage:
          dice:
            count: 2
            sides: 8
  implementation: dnd2024:class_features_druid_level_15_improved_elemental_fury
splat_contracts:
- id: dnd2024:class_features_druid_level_15_improved_elemental_fury
  inputs:
    actor:
      type: participant
      required: true
    trigger:
      required: false
      type: event
    targets:
      required: false
      type: participant_list
  outputs:
  - feature_state
  - damage
  - spell_grants
  events:
  - spell.cast_pending
  - attack.hit
  capabilities:
  - spellcasting
  - damage
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 43
  engine_gaps: &id001
  - dnd2024:class_features_druid_level_15_improved_elemental_fury
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: c4eacffa3baeb159f3bb93d759a6ba0e0e5edb2b80d2b019dfc97038ee0309b1
  parent_name: Druid
  line_span:
    first: 3602
    count: 8
splat_clauses:
- key: source_rules
  text: '# Level 15: Improved Elemental Fury The option you chose for Elemental Fury grows more powerful,
    as detailed below. Potent Spellcasting. When you cast a Druid cantrip with a range of 10 feet or greater,
    the spell’s range increases by 300 feet. Primal Strike. The extra damage of your Primal Strike increases
    to 2d8.'
  sha256: 09ae30b41749fcb81c10f3d2fdc7f9fc1311abc245aefa896b54b042d57d543e
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 15: Improved Elemental Fury

<!-- SRD page 43 -->

The option you chose for Elemental Fury grows\
more powerful, as detailed below.\
Potent Spellcasting. When you cast a Druid can-\
trip with a range of 10 feet or greater, the spell’s\
range increases by 300 feet.\
Primal Strike. The extra damage of your Primal\
Strike increases to 2d8.\

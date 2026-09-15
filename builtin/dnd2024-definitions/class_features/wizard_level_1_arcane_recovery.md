---
splat_schema: 1
splat_id: d5b5dd29-2ac9-53a0-83a0-18df525a6d87
splat_kind: rule
splat_key: dnd2024:class_features/wizard_level_1_arcane_recovery
splat_name: 'Level 1: Arcane Recovery'
splat_origin:
  key: dnd2024:reference/pages/078
splat_sources:
- title: SRD 5.2.1
  locator: pp. 78
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/wizard_level_1_arcane_recovery
  parameters:
    requires_activity: study_own_spellbook_during_short_rest
    recover_choice:
      resource: expended_spell_slots
      maximum_slot_level: 5
      maximum_combined_slot_levels:
        op: ceil
        args:
        - op: divide
          args:
          - stat: classes.wizard.level
          - 2
    uses:
      maximum: 1
      recover:
        event: rest.long_rest_completed
        amount: all
  implementation: dnd2024:class_features_wizard_level_1_arcane_recovery
splat_contracts:
- id: dnd2024:class_features_wizard_level_1_arcane_recovery
  inputs:
    actor:
      type: participant
      required: true
  outputs:
  - feature_state
  - resource_changes
  events:
  - rest.short_rest_completed
  capabilities:
  - spell_slots
  - resources
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 78
  engine_gaps: &id001
  - dnd2024:class_features_wizard_level_1_arcane_recovery
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: 404bd3907507e23b3941d1be4d12100df6092e9dfa632c83c76a59ac03f1b143
  parent_name: Wizard
  line_span:
    first: 6675
    count: 13
splat_clauses:
- key: source_rules
  text: '# Level 1: Arcane Recovery You can regain some of your magical energy by studying your spellbook.
    When you finish a Short Rest, you can choose expended spell slots to recover. The spell slots can
    have a combined level equal to no more than half your Wizard level (round up), and none of the slots
    can be level 6 or higher. For example, if you’re a level 4 Wizard, you can recover up to two levels’
    worth of spell slots, regaining either one level 2 spell slot or two level 1 spell slots. Once you
    use this feature, you can’t do so again until you finish a Long Rest.'
  sha256: 69f5423869ca3e46cc4b9fa11ba99b0819f091f0c2a45e8be582b4d726aea34e
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 1: Arcane Recovery

<!-- SRD page 78 -->

You can regain some of your magical energy by\
studying your spellbook. When you finish a Short\
Rest, you can choose expended spell slots to re-\
cover. The spell slots can have a combined level\
equal to no more than half your Wizard level (round\
up), and none of the slots can be level 6 or higher.\
For example, if you’re a level 4 Wizard, you can re-\
cover up to two levels’ worth of spell slots, regain-\
ing either one level 2 spell slot or two level 1 spell\
slots.\
Once you use this feature, you can’t do so again\
until you finish a Long Rest.\

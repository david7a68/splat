---
splat_schema: 1
splat_id: f9b3f7dc-6ffe-5839-9d14-e8ffd8a220ea
splat_kind: rule
splat_key: dnd2024:class_features/college_of_lore_level_6_magical_discoveries
splat_name: 'Level 6: Magical Discoveries'
splat_origin:
  key: dnd2024:reference/pages/035
splat_sources:
- title: SRD 5.2.1
  locator: pp. 35
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/college_of_lore_level_6_magical_discoveries
  parameters:
    spell_choice:
      count: 2
      lists:
      - cleric
      - druid
      - wizard
      mix_lists: true
      allowed_level:
        cantrip: true
        otherwise_has_bard_progression_slot: true
    always_prepared: true
    counts_as_class_spell: bard
    counts_toward_bard_prepared_limit: false
    replace:
      event: character.bard_level_gained
      maximum: 1
      same_eligibility: true
  implementation: dnd2024:class_features_college_of_lore_level_6_magical_discoveries
splat_contracts:
- id: dnd2024:class_features_college_of_lore_level_6_magical_discoveries
  inputs:
    actor:
      type: participant
      required: true
    selections:
      required: false
      type: choice
  outputs:
  - feature_state
  - choices
  - spell_grants
  events:
  - character.feature_gained
  - character.class_level_gained
  capabilities:
  - spellcasting
  - character_choices
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 35
  engine_gaps: &id001
  - dnd2024:class_features_college_of_lore_level_6_magical_discoveries
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: f21f4f784d0de6b58e3a02af60c4928ac5d74f63633efd9cad404287e6f6d753
  parent_name: College of Lore
  line_span:
    first: 2895
    count: 11
splat_clauses:
- key: source_rules
  text: '# Level 6: Magical Discoveries You learn two spells of your choice. These spells can come from
    the Cleric, Druid, or Wizard spell list or any combination thereof (see a class’s section for its
    spell list). A spell you choose must be a cantrip or a spell for which you have spell slots, as shown
    in the Bard Features table. You always have the chosen spells prepared, and whenever you gain a Bard
    level, you can replace one of the spells with another spell that meets these requirements.'
  sha256: f91435c8a0be1d5f7abc637398d39ada351d6e20b25947848339b3b00c1524b3
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 6: Magical Discoveries

<!-- SRD page 35 -->

You learn two spells of your choice. These spells can\
come from the Cleric, Druid, or Wizard spell list or\
any combination thereof (see a class’s section for its\
spell list). A spell you choose must be a cantrip or a\
spell for which you have spell slots, as shown in the\
Bard Features table.\
You always have the chosen spells prepared, and\
whenever you gain a Bard level, you can replace one\
of the spells with another spell that meets these\
requirements.\

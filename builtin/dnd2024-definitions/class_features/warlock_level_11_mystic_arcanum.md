---
splat_schema: 1
splat_id: 39a14f5c-4402-5aac-afc3-30d18aa67992
splat_kind: rule
splat_key: dnd2024:class_features/warlock_level_11_mystic_arcanum
splat_name: 'Level 11: Mystic Arcanum'
splat_origin:
  key: dnd2024:reference/pages/072
splat_sources:
- title: SRD 5.2.1
  locator: pp. 72
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/warlock_level_11_mystic_arcanum
  parameters:
    arcanum_choices:
    - warlock_level: 11
      spell_level: 6
    - warlock_level: 13
      spell_level: 7
    - warlock_level: 15
      spell_level: 8
    - warlock_level: 17
      spell_level: 9
    count_each: 1
    spell_list: warlock
    each_spell:
      spell_slot_cost: 0
      uses:
        maximum: 1
        recover:
          event: rest.long_rest_completed
          amount: all
    replace:
      event: character.warlock_level_gained
      maximum: 1
      same_spell_level: true
      list: warlock
  implementation: dnd2024:class_features_warlock_level_11_mystic_arcanum
splat_contracts:
- id: dnd2024:class_features_warlock_level_11_mystic_arcanum
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
  - resource_changes
  - spell_grants
  events:
  - character.class_level_gained
  - spell.cast_pending
  - rest.long_rest_completed
  capabilities:
  - spellcasting
  - resources
  - character_choices
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 72
  engine_gaps: &id001
  - dnd2024:class_features_warlock_level_11_mystic_arcanum
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: 9f779b78719c8f8fda29f82995b5ea499e50353daa793a02d572809863ac4a0d
  parent_name: Warlock
  line_span:
    first: 6118
    count: 16
splat_clauses:
- key: source_rules
  text: '# Level 11: Mystic Arcanum Your patron grants you a magical secret called an arcanum. Choose
    one level 6 Warlock spell as this arcanum. You can cast your arcanum spell once without expending
    a spell slot, and you must finish a Long Rest before you can cast it in this way again. As shown in
    the Warlock Features table, you gain another Warlock spell of your choice that can be cast in this
    way when you reach Warlock levels 13 (level 7 spell), 15 (level 8 spell), and 17 (level 9 spell).
    You regain all uses of your Mystic Arcanum when you finish a Long Rest. Whenever you gain a Warlock
    level, you can replace one of your arcanum spells with another Warlock spell of the same level.'
  sha256: d06a02bc183e9020ae4813ec263bc4dc63a48fbcf6474738422d04e8922eb8ea
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 11: Mystic Arcanum

<!-- SRD page 72 -->

Your patron grants you a magical secret called an\
arcanum. Choose one level 6 Warlock spell as this\
arcanum.\
You can cast your arcanum spell once without ex-\
pending a spell slot, and you must finish a Long Rest\
before you can cast it in this way again.\
As shown in the Warlock Features table, you gain\
another Warlock spell of your choice that can be\
cast in this way when you reach Warlock levels\
13 (level 7 spell), 15 (level 8 spell), and 17 (level 9\
spell). You regain all uses of your Mystic Arcanum\
when you finish a Long Rest.\
Whenever you gain a Warlock level, you can re-\
place one of your arcanum spells with another War-\
lock spell of the same level.\

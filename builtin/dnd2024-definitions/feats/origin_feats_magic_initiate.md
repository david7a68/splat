---
splat_schema: 1
splat_id: 5b970f1a-1808-5912-bea8-762c21675dee
splat_kind: rule
splat_key: dnd2024:feats/origin_feats_magic_initiate
splat_name: Magic Initiate
splat_origin:
  key: dnd2024:reference/pages/087
splat_sources:
- title: SRD 5.2.1
  locator: pp. 87
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:feats/origin_feats_magic_initiate
  parameters:
    feat_category: origin
    spell_list_choice:
      count: 1
      options:
      - cleric
      - druid
      - wizard
    spellcasting_ability_choice:
      count: 1
      options:
      - intelligence
      - wisdom
      - charisma
    cantrip_choice:
      count: 2
      spell_level: 0
      list: chosen_spell_list
    level_1_spell_choice:
      count: 1
      spell_level: 1
      list: chosen_spell_list
      always_prepared: true
      free_cast:
        spell_slot_cost: 0
        uses:
          maximum: 1
          recover:
            event: rest.long_rest_completed
            amount: all
      can_cast_with_spell_slots: true
    replace_spell:
      event: character.level_gained
      maximum: 1
      same_spell_level: true
      list: chosen_spell_list
    repeatable: true
    repeat_requirement: different_spell_list_each_selection
  implementation: dnd2024:feats_origin_feats_magic_initiate
splat_contracts:
- id: dnd2024:feats_origin_feats_magic_initiate
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
  - character.feat_selected
  - character.level_gained
  - rest.long_rest_completed
  capabilities:
  - feats
  - spellcasting
  - character_choices
  - resources
splat_conversion:
  status: specified
  category: feat
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 87
  engine_gaps: &id001
  - dnd2024:feats_origin_feats_magic_initiate
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: 6472ce0a9297adcba4a5fd45219aa32f80773c8f72cd96528148adea6125302c
  parent_name: Origin Feats
  line_span:
    first: 7487
    count: 21
splat_clauses:
- key: source_rules
  text: '# Magic Initiate Origin Feat You gain the following benefits. Two Cantrips. You learn two cantrips
    of your choice from the Cleric, Druid, or Wizard spell list. Intelligence, Wisdom, or Charisma is
    your spellcasting ability for this feat’s spells (choose when you select this feat). Level 1 Spell.
    Choose a level 1 spell from the same list you selected for this feat’s cantrips. You always have that
    spell prepared. You can cast it once without a spell slot, and you regain the ability to cast it in
    that way when you finish a Long Rest. You can also cast the spell using any spell slots you have.
    Spell Change. Whenever you gain a new level, you can replace one of the spells you chose for this
    feat with a different spell of the same level from the chosen spell list. Repeatable. You can take
    this feat more than once, but you must choose a different spell list each time.'
  sha256: 67b45b80ed21f8f3f8d6a145733b309361eb9dd263c25713e76359224b33d837
  status: contract_specified
splat_requires_mechanics: *id001
---

# Magic Initiate

<!-- SRD page 87 -->

Origin Feat\
You gain the following benefits.\
Two Cantrips. You learn two cantrips of your\
choice from the Cleric, Druid, or Wizard spell list.\
Intelligence, Wisdom, or Charisma is your spellcast-\
ing ability for this feat’s spells (choose when you\
select this feat).\
Level 1 Spell. Choose a level 1 spell from the same\
list you selected for this feat’s cantrips. You always\
have that spell prepared. You can cast it once with-\
out a spell slot, and you regain the ability to cast it\
in that way when you finish a Long Rest. You can\
also cast the spell using any spell slots you have.\
Spell Change. Whenever you gain a new level, you\
can replace one of the spells you chose for this feat\
with a different spell of the same level from the cho-\
sen spell list.\
Repeatable. You can take this feat more than\
once, but you must choose a different spell list each\
time.\

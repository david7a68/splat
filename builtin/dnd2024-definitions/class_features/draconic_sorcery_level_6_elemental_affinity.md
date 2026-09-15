---
splat_schema: 1
splat_id: 6f444684-28e3-5892-aeae-b3686a61aa0f
splat_kind: rule
splat_key: dnd2024:class_features/draconic_sorcery_level_6_elemental_affinity
splat_name: 'Level 6: Elemental Affinity'
splat_origin:
  key: dnd2024:reference/pages/070
splat_sources:
- title: SRD 5.2.1
  locator: pp. 70
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/draconic_sorcery_level_6_elemental_affinity
  parameters:
    damage_type_choice:
      count: 1
      options:
      - acid
      - cold
      - fire
      - lightning
      - poison
    resistance: chosen_damage_type
    spell_damage_bonus:
      requires_spell_deals_chosen_type: true
      maximum_rolls_per_spell: 1
      bonus:
        stat: abilities.charisma.modifier
      optional: true
  implementation: dnd2024:class_features_draconic_sorcery_level_6_elemental_affinity
splat_contracts:
- id: dnd2024:class_features_draconic_sorcery_level_6_elemental_affinity
  inputs:
    actor:
      type: participant
      required: true
    selections:
      required: false
      type: choice
    trigger:
      required: false
      type: event
    targets:
      required: false
      type: participant_list
  outputs:
  - feature_state
  - choices
  - damage
  events:
  - character.feature_gained
  - spell.damage_rolled
  capabilities:
  - damage
  - resistances
  - character_choices
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 70
  engine_gaps: &id001
  - dnd2024:class_features_draconic_sorcery_level_6_elemental_affinity
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: 511fff5e6aea364102a77b05ebef6069267db9bd04ba922aa282121a60fe26cb
  parent_name: Draconic Sorcery
  line_span:
    first: 5907
    count: 8
splat_clauses:
- key: source_rules
  text: '# Level 6: Elemental Affinity Your draconic magic has an affinity with a damage type associated
    with dragons. Choose one of those types: Acid, Cold, Fire, Lightning, or Poison. You have Resistance
    to that damage type, and when you cast a spell that deals damage of that type, you can add your Charisma
    modifier to one damage roll of that spell.'
  sha256: 73574a94a10abc52cc4954ca7c36aa8d636b5ce520619550d98b05878db1c2ea
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 6: Elemental Affinity

<!-- SRD page 70 -->

Your draconic magic has an affinity with a damage\
type associated with dragons. Choose one of those\
types: Acid, Cold, Fire, Lightning, or Poison.\
You have Resistance to that damage type, and\
when you cast a spell that deals damage of that\
type, you can add your Charisma modifier to one\
damage roll of that spell.\

---
splat_schema: 1
splat_id: a8912eaa-95b4-5ee5-b49d-2f254ea1df9d
splat_kind: rule
splat_key: dnd2024:class_features/oath_of_devotion_level_15_smite_of_protection
splat_name: 'Level 15: Smite of Protection'
splat_origin:
  key: dnd2024:reference/pages/057
splat_sources:
- title: SRD 5.2.1
  locator: pp. 57
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/oath_of_devotion_level_15_smite_of_protection
  parameters:
    trigger:
      spell_cast:
        key: dnd2024:spells/divine_smite
      caster: actor
    aura: paladin.aura_of_protection
    targets:
    - actor
    - allies
    effect:
      cover: half
    end: actor.next_turn.start
    class_scope:
      class: paladin
      minimum_level: 15
  implementation: dnd2024:class_features_oath_of_devotion_level_15_smite_of_protection
splat_contracts:
- id: dnd2024:class_features_oath_of_devotion_level_15_smite_of_protection
  inputs:
    actor:
      type: participant
      required: true
    event:
      required: false
      type: event
    selection:
      required: false
      type: choice
  outputs:
  - effects
  events:
  - spell.cast
  - turn.start
  - participant.moved
  capabilities:
  - character_features
  - spellcasting
  - auras
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 57
  engine_gaps: &id001
  - dnd2024:class_features_oath_of_devotion_level_15_smite_of_protection
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 5add76e8a6e2aa7b6899c240cbb50ad43ebce376d84af61501aaca6c1deda434
  parent_name: Oath of Devotion
  line_span:
    first: 4768
    count: 6
splat_clauses:
- key: source_rules
  text: '# Level 15: Smite of Protection Your magical smite now radiates protective energy. Whenever you
    cast Divine Smite, you and your allies have Half Cover while in your Aura of Protection. The aura
    has this benefit until the start of your next turn.'
  sha256: 73ceaff40689a8ee9032c8f9e82a10c3fed067bdd80e8120460bb9edb402a6e5
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 15: Smite of Protection

<!-- SRD page 57 -->

Your magical smite now radiates protective energy.\
Whenever you cast Divine Smite, you and your allies\
have Half Cover while in your Aura of Protection.\
The aura has this benefit until the start of your next\
turn.\

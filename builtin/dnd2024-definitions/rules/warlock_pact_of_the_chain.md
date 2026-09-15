---
splat_schema: 1
splat_id: d926ea81-ff38-59bb-ae90-7c055f174fb9
splat_kind: rule
splat_key: dnd2024:rules/warlock_pact_of_the_chain
splat_name: Pact of the Chain
splat_origin:
  key: dnd2024:reference/pages/074
splat_sources:
- title: SRD 5.2.1
  locator: pp. 74
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/warlock_pact_of_the_chain
  parameters:
    learn_spell:
      key: dnd2024:spells/find_familiar
    casting:
      economy: magic_action
      spell_slot_cost: 0
    familiar_forms:
      allow_normal: true
      additional:
      - key: dnd2024:creatures/imp
      - key: dnd2024:creatures/pseudodragon
      - key: dnd2024:creatures/quasit
      - key: dnd2024:creatures/skeleton
      - key: dnd2024:creatures/sphinx_of_wonder
      - key: dnd2024:creatures/sprite
      - key: dnd2024:creatures/venomous_snake
    attack_exchange:
      trigger: actor.attack_action
      forgo_attacks: 1
      familiar_attacks: 1
      familiar_economy: reaction
  implementation: dnd2024:rules_warlock_pact_of_the_chain
splat_contracts:
- id: dnd2024:rules_warlock_pact_of_the_chain
  inputs:
    actor:
      type: participant
      required: true
    targets:
      type: participant_list
      required: true
    choice:
      type: choice
      required: true
    event:
      required: false
      type: event
  outputs:
  - effects
  - resource_updates
  events: []
  capabilities:
  - class_features
  - effects
  - resources
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 74
  engine_gaps: &id001
  - dnd2024:rules_warlock_pact_of_the_chain
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: bbad850fa66a0cecb44debe314cd88215f434305c20da3b205431fffe05315dd
  parent_name: Warlock
  line_span:
    first: 6314
    count: 12
splat_clauses:
- key: source_rules
  text: '# Pact of the Chain You learn the Find Familiar spell and can cast it as a Magic action without
    expending a spell slot. When you cast the spell, you choose one of the normal forms for your familiar
    or one of the following special forms: Imp, Pseudodragon, Quasit, Skeleton, Sphinx of Wonder, Sprite,
    or Venomous Snake (see “Monsters” for the familiar’s stat block). Additionally, when you take the
    Attack action, you can forgo one of your own attacks to allow your familiar to make one attack of
    its own with its Reaction.'
  sha256: 1812aa653a3a5f1c13577b09f836e2b441b6f04254120c81afa04e9ad16dea09
  status: contract_specified
splat_requires_mechanics: *id001
---

# Pact of the Chain

<!-- SRD page 74 -->

You learn the Find Familiar spell and can cast it as a\
Magic action without expending a spell slot.\
When you cast the spell, you choose one of the\
normal forms for your familiar or one of the follow-\
ing special forms: Imp, Pseudodragon, Quasit,\
Skeleton, Sphinx of Wonder, Sprite, or Venomous\
Snake (see “Monsters” for the familiar’s stat block).\
Additionally, when you take the Attack action,\
you can forgo one of your own attacks to allow\
your familiar to make one attack of its own with its\
Reaction.\

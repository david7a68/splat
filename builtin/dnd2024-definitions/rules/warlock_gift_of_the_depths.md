---
splat_schema: 1
splat_id: 691823bb-24ef-5b0c-a455-5a9cb9598b9e
splat_kind: rule
splat_key: dnd2024:rules/warlock_gift_of_the_depths
splat_name: Gift of the Depths
splat_origin:
  key: dnd2024:reference/pages/073
splat_sources:
- title: SRD 5.2.1
  locator: pp. 73
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/warlock_gift_of_the_depths
  parameters:
    prerequisites:
      warlock_level_min: 5
    breathe_underwater: true
    swim_speed:
      var: subject.speed
    grant_cast:
      spell:
        key: dnd2024:spells/water_breathing
      spell_slot_cost: 0
      uses: 1
      reset: long_rest_finished
  implementation: dnd2024:rules_warlock_gift_of_the_depths
splat_contracts:
- id: dnd2024:rules_warlock_gift_of_the_depths
  inputs:
    subject:
      type: participant
      required: true
  outputs:
  - speeds
  - respiration
  - spell_grant
  events: []
  capabilities:
  - movement
  - spellcasting
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 73
  engine_gaps: &id001
  - dnd2024:rules_warlock_gift_of_the_depths
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 600bd3908a847a07b30230957c1e3ed5c8f27722d70401a9714ee6a762deeb1f
  parent_name: Warlock
  line_span:
    first: 6214
    count: 7
splat_clauses:
- key: source_rules
  text: '# Gift of the Depths Prerequisite: Level 5+ Warlock You can breathe underwater, and you gain
    a Swim Speed equal to your Speed. You can also cast Water Breathing once without expending a spell
    slot. You regain the ability to cast it in this way again when you finish a Long Rest.'
  sha256: d46535710d3a2266b37a79c329137dbc5b380d73a3e38c70c0dbc81aea7af887
  status: contract_specified
splat_requires_mechanics: *id001
---

# Gift of the Depths

<!-- SRD page 73 -->

Prerequisite: Level 5+ Warlock\
You can breathe underwater, and you gain a Swim\
Speed equal to your Speed.\
You can also cast Water Breathing once without\
expending a spell slot. You regain the ability to cast\
it in this way again when you finish a Long Rest.\

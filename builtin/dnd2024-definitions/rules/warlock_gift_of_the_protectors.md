---
splat_schema: 1
splat_id: 5a8679a2-1803-5527-8ffb-ae21f5086537
splat_kind: rule
splat_key: dnd2024:rules/warlock_gift_of_the_protectors
splat_name: Gift of the Protectors
splat_origin:
  key: dnd2024:reference/pages/073
splat_sources:
- title: SRD 5.2.1
  locator: pp. 73
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/warlock_gift_of_the_protectors
  parameters:
    prerequisite:
      warlock_level_minimum: 9
      invocation: pact_of_the_tome
    book_page:
      created_when: book_of_shadows_conjured
      name_capacity:
        op: max
        args:
        - 1
        - variable: charisma_modifier
      add_name:
        action_cost: 1
        writer: named_creature
        warlock_permission: true
      erase_name:
        action: magic
        touch_page: true
    protect:
      trigger: named_creature_reduced_to_zero_hit_points
      exclude: killed_outright
      set_hit_points: 1
      shared_uses: 1
      recharge: warlock_long_rest
  implementation: dnd2024:rule_warlock_gift_of_the_protectors
splat_contracts:
- id: dnd2024:rule_warlock_gift_of_the_protectors
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
    charisma_modifier:
      type: number
      required: true
  outputs:
  - feature_effects
  events:
  - hit_points.reduced_to_zero
  - rest.long_completed
  capabilities:
  - invocations
  - hit_points
  - resources
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 73
  engine_gaps: &id001
  - dnd2024:rule_warlock_gift_of_the_protectors
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: ee07a1021f6f4e8e13978d3149dd56b71e5cbe498c0ff4519f9333e7a7c984ac
  parent_name: Warlock
  line_span:
    first: 6221
    count: 15
splat_clauses:
- key: source_rules
  text: '# Gift of the Protectors Prerequisite: Level 9+ Warlock, Pact of the Tome Invocation A new page
    appears in your Book of Shadows when you conjure it. With your permission, a creature can take an
    action to write its name on that page, which can contain a number of names equal to your Charisma
    modifier (minimum of one name). When any creature whose name is on the page is reduced to 0 Hit Points
    but not killed outright, the creature magically drops to 1 Hit Point instead. Once this magic is triggered,
    no creature can benefit from it until you finish a Long Rest. As a Magic action, you can erase a name
    on the page by touching it.'
  sha256: 8182a61cb51cc539225cbcc8d9fe3f3ed10c7754cc6438f5065001ef0dadec46
  status: contract_specified
splat_requires_mechanics: *id001
---

# Gift of the Protectors

<!-- SRD page 73 -->

Prerequisite: Level 9+ Warlock, Pact of the Tome\
Invocation\
A new page appears in your Book of Shadows when\
you conjure it. With your permission, a creature can\
take an action to write its name on that page, which\
can contain a number of names equal to your Cha-\
risma modifier (minimum of one name).\
When any creature whose name is on the page is\
reduced to 0 Hit Points but not killed outright, the\
creature magically drops to 1 Hit Point instead.\
Once this magic is triggered, no creature can benefit\
from it until you finish a Long Rest.\
As a Magic action, you can erase a name on the\
page by touching it.\

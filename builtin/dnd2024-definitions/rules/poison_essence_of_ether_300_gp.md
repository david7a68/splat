---
splat_schema: 1
splat_id: ef56cdab-838b-584b-99e9-ef935b1d8021
splat_kind: rule
splat_key: dnd2024:rules/poison_essence_of_ether_300_gp
splat_name: Essence of Ether (300 GP)
splat_origin:
  key: dnd2024:reference/pages/197
splat_sources:
- title: SRD 5.2.1
  locator: pp. 197
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/poison_essence_of_ether_300_gp
  parameters:
    poison:
      type: inhaled
      cost_gp: 300
    save:
      ability: constitution
      dc: 15
    failure:
      condition:
        id: poisoned
        duration:
          value: 8
          unit: hours
      unconscious_while_poisoned: true
    wake_triggers:
    - damage_taken
    - other_creature_action_shake_awake
    wake_ends_poisoned: false
  implementation: dnd2024:rules_poison_essence_of_ether_300_gp
splat_contracts:
- id: dnd2024:rules_poison_essence_of_ether_300_gp
  inputs:
    actor:
      type: participant
      required: true
    targets:
      type: participant_list
      required: true
    hazard:
      type: object
      required: true
    event:
      type: event
      required: true
    resolution:
      type: resolution
      required: true
    choice:
      type: choice
      required: true
  outputs:
  - effects
  - hazard_updates
  events: []
  capabilities:
  - hazards
  - effects
  - saving_throws
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 197
  engine_gaps: &id001
  - dnd2024:rules_poison_essence_of_ether_300_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: ce8291ade40839e944b08db96d65627615a7ae7709b500d608884fc6544ee5c4
  parent_name: Poison
  line_span:
    first: 9850
    count: 9
splat_clauses:
- key: source_rules
  text: '# Essence of Ether (300 GP) Inhaled Poison A creature subjected to Essence of Ether must succeed
    on a DC 15 Constitution saving throw or have the Poisoned condition for 8 hours. The creature also
    has the Unconscious condition while Poisoned in this way. The creature wakes up if it takes damage
    or if another creature takes an action to shake it awake.'
  sha256: a0c1e83002c5b0c2cd76847a1664c23c263ea720562b0d970d6a4eee1794fe0e
  status: contract_specified
splat_requires_mechanics: *id001
---

# Essence of Ether (300 GP)

<!-- SRD page 197 -->

Inhaled Poison\
A creature subjected to Essence of Ether must suc-\
ceed on a DC 15 Constitution saving throw or have\
the Poisoned condition for 8 hours. The creature\
also has the Unconscious condition while Poisoned\
in this way. The creature wakes up if it takes dam-\
age or if another creature takes an action to shake\
it awake.\

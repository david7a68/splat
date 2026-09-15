---
splat_schema: 1
splat_id: 838b0882-4007-50ea-b151-1b931815930f
splat_kind: rule
splat_key: dnd2024:glossary/restrained_condition
splat_name: Restrained [Condition]
splat_origin:
  key: dnd2024:reference/glossary/restrained_condition
splat_sources:
- title: SRD 5.2.1
  locator: pp. 187
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/restrained_condition
  parameters:
    condition: restrained
    speed: 0
    speed_can_increase: false
    incoming_attack_rolls: advantage
    own_attack_rolls: disadvantage
    dexterity_saves: disadvantage
  implementation: dnd2024:glossary_restrained_condition
splat_contracts:
- id: dnd2024:glossary_restrained_condition
  inputs:
    subject:
      type: participant
      required: true
  outputs:
  - condition_effects
  events:
  - attack.pending
  - saving_throw.pending
  - speed.calculated
  capabilities:
  - conditions
  - attacks
  - checks
  - movement
splat_conversion:
  status: specified
  category: glossary
  source_sha256: 8be9904c115479aa6ad9ce46355f6727d0e8d80dd2abe0d8e8de64dc3509847f
  source_path: builtin/srd-5.2.1/glossary/restrained_condition.md
  pages:
  - 187
  engine_gaps: &id001
  - dnd2024:glossary_restrained_condition
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 2df7f4e6e096cfbac995dc23237e9771282f6cc0969d127e5e95ddc80d506ca6
splat_clauses:
- key: source_rules
  text: '# Restrained [Condition] While you have the Restrained condition, you experience the following
    effects. Speed 0. Your Speed is 0 and can’t increase. Attacks Affected. Attack rolls against you have
    Advantage, and your attack rolls have Disadvantage. Saving Throws Affected. You have Disadvantage
    on Dexterity saving throws.'
  sha256: ba1e7e620d7f0679c1739e33b8904070806c9425f057acdcaee2dfddc6720200
  status: contract_specified
splat_requires_mechanics: *id001
---

# Restrained [Condition]

<!-- SRD page 187 -->

While you have the Restrained condition, you expe-\
rience the following effects.\
Speed 0. Your Speed is 0 and can’t increase.\
Attacks Affected. Attack rolls against you have\
Advantage, and your attack rolls have Disadvantage.\
Saving Throws Affected. You have Disadvantage\
on Dexterity saving throws.\

Original layout: [[../../srd-5.2.1/pages/187|page 187]].

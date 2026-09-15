---
splat_schema: 1
splat_id: e885cce0-ca1c-502c-8043-9887e57a7c4f
splat_kind: rule
splat_key: dnd2024:glossary/disengage_action
splat_name: Disengage [Action]
splat_origin:
  key: dnd2024:reference/glossary/disengage_action
splat_sources:
- title: SRD 5.2.1
  locator: pp. 181
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/disengage_action
  parameters:
    cost: action
    duration: remainder_of_current_turn
    movement_provokes_opportunity_attacks: false
  implementation: dnd2024:glossary_disengage_action
splat_contracts:
- id: dnd2024:glossary_disengage_action
  inputs:
    subject:
      type: participant
      required: true
  outputs:
  - opportunity_attack_exemption
  events:
  - action.requested
  - movement.pending
  capabilities:
  - action_economy
  - movement
  - attacks
splat_conversion:
  status: specified
  category: glossary
  source_sha256: bb0ced8cf69257ab227d5e4f741e38e50d9dc1b744e782d1f2210e8980022b7a
  source_path: builtin/srd-5.2.1/glossary/disengage_action.md
  pages:
  - 181
  engine_gaps: &id001
  - dnd2024:glossary_disengage_action
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 544a5bdb1ece321d674bdcf95df1e5746cc48afbe58e3da702753e5d6f2b2455
splat_clauses:
- key: source_rules
  text: '# Disengage [Action] If you take the Disengage action, your movement doesn’t provoke Opportunity
    Attacks for the rest of the current turn.'
  sha256: 6772836fefaa7b04fb35772abb509f2e1c7922a90aad6982a729d0d9220afe9f
  status: contract_specified
splat_requires_mechanics: *id001
---

# Disengage [Action]

<!-- SRD page 181 -->

If you take the Disengage action, your movement\
doesn’t provoke Opportunity Attacks for the rest of\
the current turn.\

Original layout: [[../../srd-5.2.1/pages/181|page 181]].

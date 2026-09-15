---
splat_schema: 1
splat_id: 8bb25829-19e0-542a-95b9-e6e909c45197
splat_kind: rule
splat_key: dnd2024:glossary/challenge_rating
splat_name: Challenge Rating
splat_origin:
  key: dnd2024:reference/glossary/challenge_rating
splat_sources:
- title: SRD 5.2.1
  locator: pp. 178
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/challenge_rating
  parameters:
    reference_party_size: 4
    comparison_to_character_level:
      greater: likely_danger
      less: likely_little_threat
    estimate_modifiers:
    - circumstances
    - party_size
    estimate_is_binding: false
  implementation: dnd2024:glossary_challenge_rating
splat_contracts:
- id: dnd2024:glossary_challenge_rating
  inputs:
    monster:
      type: participant
      required: true
    character_level:
      type: integer
      required: true
    party_size:
      type: integer
      required: true
  outputs:
  - threat_estimate
  events: []
  capabilities:
  - encounter_planning
splat_conversion:
  status: specified
  category: glossary
  source_sha256: d3ba601aa3d7a85792cdb89b6d4217ad10a3d02b8272970c0e5757a83f2039d5
  source_path: builtin/srd-5.2.1/glossary/challenge_rating.md
  pages:
  - 178
  engine_gaps: &id001
  - dnd2024:glossary_challenge_rating
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: d4d6df292f4eef03e4e89b6f05e48500875b76f3edc452a22a54ed50257fab7b
splat_clauses:
- key: source_rules
  text: '# Challenge Rating Challenge Rating (CR) summarizes the threat a monster poses to a group of
    four player characters. Compare a monster’s CR to the characters’ level. If the CR is higher, the
    monster is likely a danger. If the CR is lower, the monster likely poses little threat. But circumstances
    and the number of player characters can significantly alter how threatening a monster is in actual
    play. “Gameplay Toolbox” (“Combat Encounters”) provides guidance to the GM on using CR while planning
    potential combat encounters. See also “Stat Block.”'
  sha256: 51d9fb31c030cf483a19f30d2b1a61e338125649c3b7fed017477a93e0a88e98
  status: contract_specified
splat_requires_mechanics: *id001
---

# Challenge Rating

<!-- SRD page 178 -->

Challenge Rating (CR) summarizes the threat a\
monster poses to a group of four player characters.\
Compare a monster’s CR to the characters’ level. If\
the CR is higher, the monster is likely a danger. If the\
CR is lower, the monster likely poses little threat.\
But circumstances and the number of player charac-\
ters can significantly alter how threatening a mon-\
ster is in actual play. “Gameplay Toolbox” (“Combat\
Encounters”) provides guidance to the GM on using\
CR while planning potential combat encounters. See\
also “Stat Block.”\

Original layout: [[../../srd-5.2.1/pages/178|page 178]].

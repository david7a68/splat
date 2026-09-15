---
splat_schema: 1
splat_id: ea893ce1-636c-54cb-8ddd-fde7402372fc
splat_kind: rule
splat_key: dnd2024:glossary/attitude
splat_name: Attitude
splat_origin:
  key: dnd2024:reference/glossary/attitude
splat_sources:
- title: SRD 5.2.1
  locator: pp. 177
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/attitude
  parameters:
    scope: monster_toward_player_character
    states:
    - friendly
    - hostile
    - indifferent
    default: indifferent
  implementation: dnd2024:glossary_attitude
splat_contracts:
- id: dnd2024:glossary_attitude
  inputs:
    subject:
      type: participant
      required: true
    other:
      type: participant
      required: true
    attitude:
      type: choice
      required: true
  outputs:
  - attitude
  events: []
  capabilities:
  - relationships
splat_conversion:
  status: specified
  category: glossary
  source_sha256: bd7453d7e16e812520321147962d24385420f63e7a90a654fece0c858e52a18a
  source_path: builtin/srd-5.2.1/glossary/attitude.md
  pages:
  - 177
  engine_gaps: &id001
  - dnd2024:glossary_attitude
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 7306cd83df1e3633edec922ef3a0eb1f7a58bd308c7000e34b0bec0de5afd2fb
splat_clauses:
- key: source_rules
  text: '# Attitude A monster has a starting attitude toward a player character: Friendly, Hostile, or
    Indifferent. See also “Friendly,” “Hostile,” “Indifferent,” and “Influence.”'
  sha256: a0add0a9b15c5cb0f4a613b4a8f2fdae15e23ed71e140a43fe1962e2b20038d6
  status: contract_specified
splat_requires_mechanics: *id001
---

# Attitude

<!-- SRD page 177 -->

A monster has a starting attitude toward a player\
character: Friendly, Hostile, or Indifferent. See also\
“Friendly,” “Hostile,” “Indifferent,” and “Influence.”\

Original layout: [[../../srd-5.2.1/pages/177|page 177]].

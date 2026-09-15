---
splat_schema: 1
splat_id: afd0c8f1-476b-56c3-8c8a-d37f4f45c93a
splat_kind: rule
splat_key: dnd2024:glossary/heroic_inspiration
splat_name: Heroic Inspiration
splat_origin:
  key: dnd2024:reference/glossary/heroic_inspiration
splat_sources:
- title: SRD 5.2.1
  locator: pp. 183
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/heroic_inspiration
  parameters:
    eligible: player_character
    capacity: 1
    expend:
      timing: immediately_after_rolling_any_die
      reroll_count: 1
      must_use_new_result: true
    gain_while_full:
      default: lost
      optional_transfer:
        recipient: player_character_without_heroic_inspiration
        chooser: subject
  implementation: dnd2024:glossary_heroic_inspiration
splat_contracts:
- id: dnd2024:glossary_heroic_inspiration
  inputs:
    subject:
      type: participant
      required: true
    die_result:
      required: false
      type: resolution
    transfer_target:
      required: false
      type: participant
  outputs:
  - reroll
  - inspiration_state
  events:
  - die.rolled
  - heroic_inspiration.gained
  capabilities:
  - dice
  - resources
splat_conversion:
  status: specified
  category: glossary
  source_sha256: 53272e0e45e9492eef2b1e52cba4316415e3aae3929395d4d5eb670cb4133755
  source_path: builtin/srd-5.2.1/glossary/heroic_inspiration.md
  pages:
  - 183
  engine_gaps: &id001
  - dnd2024:glossary_heroic_inspiration
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 7f44e3edd68419f0a9c784e29b973c1ef593390d7d7ec87e625d489b35398f2c
splat_clauses:
- key: source_rules
  text: '# Heroic Inspiration If you (a player character) have Heroic Inspiration, you can expend it to
    reroll any die immediately after rolling it, and you must use the new roll. If you gain Heroic Inspiration
    but already have it, it’s lost unless you give it to a player character who lacks it.'
  sha256: 0bac4a4e53a3a12347e60a92dad59567d37c6f8fa83fa506406a8e8f2061dcce
  status: contract_specified
splat_requires_mechanics: *id001
---

# Heroic Inspiration

<!-- SRD page 183 -->

If you (a player character) have Heroic Inspiration,\
you can expend it to reroll any die immediately af-\
ter rolling it, and you must use the new roll.\
If you gain Heroic Inspiration but already have it,\
it’s lost unless you give it to a player character who\
lacks it.\

Original layout: [[../../srd-5.2.1/pages/183|page 183]].

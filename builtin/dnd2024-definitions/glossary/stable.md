---
splat_schema: 1
splat_id: df41374b-9104-5a0b-880d-302104d30e2c
splat_kind: rule
splat_key: dnd2024:glossary/stable
splat_name: Stable
splat_origin:
  key: dnd2024:reference/glossary/stable
splat_sources:
- title: SRD 5.2.1
  locator: pp. 188
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/stable
  parameters:
    hit_points: 0
    death_saving_throws_required: false
  implementation: dnd2024:glossary_stable
splat_contracts:
- id: dnd2024:glossary_stable
  inputs:
    subject:
      type: participant
      required: true
  outputs:
  - death_save_exemption
  events:
  - turn.started
  capabilities:
  - death
  - hit_points
splat_conversion:
  status: specified
  category: glossary
  source_sha256: db14d516c7873e655eac007989b6b384fa890532f96dd6c3a6d5779f50f0b3cd
  source_path: builtin/srd-5.2.1/glossary/stable.md
  pages:
  - 188
  engine_gaps: &id001
  - dnd2024:glossary_stable
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: b9033dc7c10f0ec25d8efd044eda4c971be549911225ad7e3690593dae89832f
splat_clauses:
- key: source_rules
  text: '# Stable A creature is Stable if it has 0 Hit Points but isn’t required to make Death Saving
    Throws. See also “Playing the Game” (“Damage and Healing”).'
  sha256: 1b72bae41fb194a2c2f140f428b3b6304990f0de288a0315c7a582c4dd1f2f46
  status: contract_specified
splat_requires_mechanics: *id001
---

# Stable

<!-- SRD page 188 -->

A creature is Stable if it has 0 Hit Points but isn’t\
required to make Death Saving Throws. See also\
“Playing the Game” (“Damage and Healing”).\

Original layout: [[../../srd-5.2.1/pages/188|page 188]].

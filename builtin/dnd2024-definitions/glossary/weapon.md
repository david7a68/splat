---
splat_schema: 1
splat_id: 97f523c3-606a-5245-8ca7-afe5600c74e6
splat_kind: rule
splat_key: dnd2024:glossary/weapon
splat_name: Weapon
splat_origin:
  key: dnd2024:reference/glossary/weapon
splat_sources:
- title: SRD 5.2.1
  locator: pp. 191
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/weapon
  parameters:
    object_categories:
    - simple
    - martial
  implementation: dnd2024:glossary_weapon
splat_contracts:
- id: dnd2024:glossary_weapon
  inputs:
    object:
      type: item_instance
      required: true
  outputs:
  - is_weapon
  events: []
  capabilities:
  - equipment
  - targeting
splat_conversion:
  status: specified
  category: glossary
  source_sha256: dde9817b1f8026ac4a7cc3a214031012281743317c9dd547a01ebbc89ccee639
  source_path: builtin/srd-5.2.1/glossary/weapon.md
  pages:
  - 191
  engine_gaps: &id001
  - dnd2024:glossary_weapon
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: e9855406682134be75ec0944e8e1cc62ad708e35d5c40e467c16a400e0638775
splat_clauses:
- key: source_rules
  text: '# Weapon A weapon is an object that is in the Simple or Martial weapon category. See also “Equipment”
    (“Weapons”).'
  sha256: ede6cfc8766c2069ba92b57ed05318f52234daa624c7551c75b03f961f646d0a
  status: contract_specified
splat_requires_mechanics: *id001
---

# Weapon

<!-- SRD page 191 -->

A weapon is an object that is in the Simple or\
Martial weapon category. See also “Equipment”\
(“Weapons”).\

Original layout: [[../../srd-5.2.1/pages/191|page 191]].

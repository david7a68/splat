---
splat_schema: 1
splat_id: b15ddebc-afe0-5607-bc5d-e8d4ce651a27
splat_kind: rule
splat_key: dnd2024:glossary/armor_class
splat_name: Armor Class
splat_origin:
  key: dnd2024:reference/glossary/armor_class
splat_sources:
- title: SRD 5.2.1
  locator: pp. 177
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/armor_class
  parameters:
    attack_target_number: armor_class
    default_base:
      op: add
      args:
      - 10
      - var: dexterity_modifier
    alternative_base:
      chooser: subject
      options: available_base_ac_calculations
      select_count: 1
      stack: false
  implementation: dnd2024:glossary_armor_class
splat_contracts:
- id: dnd2024:glossary_armor_class
  inputs:
    subject:
      type: participant
      required: true
    selected_base:
      type: choice
      required: true
  outputs:
  - armor_class
  events: []
  capabilities:
  - statistics
  - attacks
splat_conversion:
  status: specified
  category: glossary
  source_sha256: 706ce3b580282fdcfc166d3dccebb6be6af5cfe7c92579d5ea585f85e9b3b559
  source_path: builtin/srd-5.2.1/glossary/armor_class.md
  pages:
  - 177
  engine_gaps: &id001
  - dnd2024:glossary_armor_class
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 756a57f01846a2ac7689ee7d45fad0d09d783af29dc4dd0bfabe2170e58da1ef
splat_clauses:
- key: source_rules
  text: '# Armor Class An Armor Class (AC) is the target number for an attack roll. AC represents how
    difficult it is to hit a target. Your base AC calculation is 10 plus your Dexterity modifier. If a
    rule gives you another base AC calculation, you choose which calculation to use; you can’t use more
    than one. See also “Attack Roll.”'
  sha256: e870db278c7a2491ecf9d44543ae50ebfd223018164d8da01feed8d1cbffb6ec
  status: contract_specified
splat_requires_mechanics: *id001
---

# Armor Class

<!-- SRD page 177 -->

An Armor Class (AC) is the target number for an\
attack roll. AC represents how difficult it is to hit a\
target.\
Your base AC calculation is 10 plus your Dexterity\
modifier. If a rule gives you another base AC calcula-\
tion, you choose which calculation to use; you can’t\
use more than one. See also “Attack Roll.”\

Original layout: [[../../srd-5.2.1/pages/177|page 177]].

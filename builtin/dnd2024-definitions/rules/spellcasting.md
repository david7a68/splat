---
splat_schema: 1
splat_id: 4bb066b4-2070-57bd-abe5-b452534cad3b
splat_kind: rule
splat_key: dnd2024:rules/spellcasting
splat_name: Spellcasting
splat_origin:
  key: dnd2024:reference/pages/102
splat_sources:
- title: SRD 5.2.1
  locator: pp. 102
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/spellcasting
  parameters:
    component_cost_added: true
    service_costs:
    - spell_levels:
      - 0
      - 0
      settlements:
      - village
      - town
      - city
      cost_gp: 30
    - spell_levels:
      - 1
      - 1
      settlements:
      - village
      - town
      - city
      cost_gp: 50
    - spell_levels:
      - 2
      - 2
      settlements:
      - village
      - town
      - city
      cost_gp: 200
    - spell_levels:
      - 3
      - 3
      settlements:
      - town
      - city
      cost_gp: 300
    - spell_levels:
      - 4
      - 5
      settlements:
      - town
      - city
      cost_gp: 2000
    - spell_levels:
      - 6
      - 8
      settlements:
      - city
      cost_gp: 20000
    - spell_levels:
      - 9
      - 9
      settlements:
      - city
      cost_gp: 100000
  implementation: dnd2024:rules_spellcasting
splat_contracts:
- id: dnd2024:rules_spellcasting
  inputs:
    actor:
      type: participant
      required: true
    choice:
      type: choice
      required: true
    days:
      required: false
      type: integer
  outputs:
  - cost
  - availability
  events: []
  capabilities:
  - economy
  - services
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 102
  engine_gaps: &id001
  - dnd2024:rules_spellcasting
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: 45ae263f1414f2ab11e3abb50861660409a28b0669478ee474bdfe90de1dc20e
  parent_name: Spellcasting
  line_span:
    first: 8852
    count: 16
splat_clauses:
- key: source_rules
  text: '# Spellcasting Most settlements contain individuals who are willing to cast spells in exchange
    for payment. If a spell has expensive components, add the cost of those components to the cost listed
    in the Spellcasting Services table. The higher the level of a desired spell, the harder it is to find
    someone to cast it. Spellcasting Services Spell Level Availability Cost Cantrip Village, town, or
    city 30 GP 1 Village, town, or city 50 GP 2 Village, town, or city 200 GP 3 Town or city only 300
    GP 4–5 Town or city only 2,000 GP 6–8 City only 20,000 GP 9 City only 100,000 GP'
  sha256: 7e0ba9104918b2bda10b08aa4f4b8cb4d655640aa44ff95349226f8fd07d8391
  status: contract_specified
splat_requires_mechanics: *id001
---

# Spellcasting

<!-- SRD page 102 -->

Most settlements contain individuals who are will-\
ing to cast spells in exchange for payment. If a spell\
has expensive components, add the cost of those\
components to the cost listed in the Spellcasting\
Services table. The higher the level of a desired\
spell, the harder it is to find someone to cast it.\
Spellcasting Services\
Spell Level Availability Cost\
Cantrip Village, town, or city 30 GP\
1 Village, town, or city 50 GP\
2 Village, town, or city 200 GP\
3 Town or city only 300 GP\
4–5 Town or city only 2,000 GP\
6–8 City only 20,000 GP\
9 City only 100,000 GP\

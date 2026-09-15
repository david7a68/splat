---
splat_schema: 1
splat_id: 54d4c256-6dfc-5616-8d82-80502a91ce39
splat_kind: spell
splat_key: dnd2024:spells/beacon_of_hope
splat_name: Beacon of Hope
splat_origin:
  key: dnd2024:reference/spell/beacon_of_hope
splat_sources:
- title: SRD 5.2.1
  locator: pp. 112
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 3
  school: abjuration
  classes:
  - cleric
  components:
  - verbal
  - somatic
  concentration: true
  duration: Concentration, up to 1 minute
  casting_time: Action
  ritual: false
  range:
    feet: 30
splat_rule:
  mechanic: dnd2024:spells/beacon_of_hope
  implementation: dnd2024:spell_beacon_of_hope
  parameters:
    targeting:
      kind: creatures
      count: any
      within_range: true
    duration: spell_duration
    advantage_on:
    - wisdom_saving_throws
    - death_saving_throws
    healing:
      amount: maximum_possible
      sources: all
    spell_lifetime:
      concentration: true
      maximum:
        value: 1
        unit: minute
splat_contracts:
- id: dnd2024:spell_beacon_of_hope
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    targets:
      type: participant_list
      required: true
  outputs:
  - effects
  - healing_results
  events:
  - spell.cast_completed
  - effect.ended
  capabilities:
  - spellcasting
  - effects
  - healing
splat_conversion:
  status: specified
  category: spell
  source_sha256: 71d310471948fc74057fa65122c6b730ed81e1442adebf497b87e7e09fde5027
  source_path: builtin/srd-5.2.1/spell/beacon_of_hope.md
  pages:
  - 112
  engine_gaps: &id001
  - dnd2024:spell_beacon_of_hope
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_2.md
  specification_sha256: 2cb14ed6d4d8fdc53de5e9c298ca68d035cb71e554d4040f765b253e7f00e184
splat_clauses:
- key: source_rules
  text: '# Beacon of Hope Level 3 Abjuration (Cleric) Casting Time: Action Range: 30 feet Components:
    V, S Duration: Concentration, up to 1 minute Choose any number of creatures within range. For the
    duration, each target has Advantage on Wisdom saving throws and Death Saving Throws and regains the
    maximum number of Hit Points possible from any healing.'
  sha256: bc853683c79da329873d976e61750c246c3da8ed7243a2fa0973789a63d19a11
  status: contract_specified
splat_requires_mechanics: *id001
---

# Beacon of Hope

<!-- SRD page 112 -->

Level 3 Abjuration (Cleric)\
Casting Time: Action\
Range: 30 feet\
Components: V, S\
Duration: Concentration, up to 1 minute\
Choose any number of creatures within range. For\
the duration, each target has Advantage on Wisdom\
saving throws and Death Saving Throws and re-\
gains the maximum number of Hit Points possible\
from any healing.\

Original layout: [[../../srd-5.2.1/pages/112|page 112]].

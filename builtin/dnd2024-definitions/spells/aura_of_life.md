---
splat_schema: 1
splat_id: d3530c90-dbc4-5def-bda2-896fc7190e55
splat_kind: spell
splat_key: dnd2024:spells/aura_of_life
splat_name: Aura of Life
splat_origin:
  key: dnd2024:reference/spell/aura_of_life
splat_sources:
- title: SRD 5.2.1
  locator: pp. 111
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 4
  school: abjuration
  classes:
  - cleric
  - paladin
  components:
  - verbal
  concentration: true
  duration: Concentration, up to 10 minutes
  casting_time: Action
  ritual: false
  range:
    mode: self
splat_rule:
  mechanic: dnd2024:spells/aura_of_life
  implementation: dnd2024:spell_aura_of_life
  parameters:
    area:
      shape: emanation
      radius_feet: 30
      origin: caster
    beneficiaries:
    - caster
    - allies
    while_inside:
      resistance:
      - necrotic
      prevent_hp_maximum_reduction: true
    healing:
      event: turn.start
      subject: ally
      inside_aura: true
      hp_equals: 0
      amount: 1
splat_contracts:
- id: dnd2024:spell_aura_of_life
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    targets:
      required: false
      type: participant_list
    objects:
      required: false
      type: object_list
    locations:
      required: false
      type: location_list
    choices:
      required: false
      type: choice
    resolution:
      required: false
      type: resolution
    campaign:
      required: false
      type: campaign
  outputs:
  - aura_effect
  - resistances
  - hp_maximum_protection
  - healing
  events:
  - spell.cast_completed
  - turn.start
  capabilities:
  - spellcasting
  - effects
  - healing
  - spatial_areas
  - time
  - relationships
splat_conversion:
  status: specified
  category: spell
  source_sha256: 1559da6db8e15139feafe9f4d283255e951605f88fc571cd69a89d2e4f362769
  source_path: builtin/srd-5.2.1/spell/aura_of_life.md
  pages:
  - 111
  engine_gaps: &id001
  - dnd2024:spell_aura_of_life
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_1.md
  specification_sha256: 5e44a470c96ff4f2f943485c951426e035649f6a715264cb4a6d41c5bf2ba243
splat_clauses:
- key: source_rules
  text: '# Aura of Life Level 4 Abjuration (Cleric, Paladin) Casting Time: Action Range: Self Components:
    V Duration: Concentration, up to 10 minutes An aura radiates from you in a 30-foot Emanation for the
    duration. While in the aura, you and your allies have Resistance to Necrotic damage, and your Hit
    Point maximums can’t be reduced. If an ally with 0 Hit Points starts its turn in the aura, that ally
    regains 1 Hit Point.'
  sha256: 9794da94c54fef3e425e92102bbe7367930512a47438d375db49f34368582ec5
  status: contract_specified
splat_requires_mechanics: *id001
---

# Aura of Life

<!-- SRD page 111 -->

Level 4 Abjuration (Cleric, Paladin)\
Casting Time: Action\
Range: Self\
Components: V\
Duration: Concentration, up to 10 minutes\
An aura radiates from you in a 30-foot Emanation\
for the duration. While in the aura, you and your\
allies have Resistance to Necrotic damage, and your\
Hit Point maximums can’t be reduced. If an ally with\
0 Hit Points starts its turn in the aura, that ally re-\
gains 1 Hit Point.\

Original layout: [[../../srd-5.2.1/pages/111|page 111]].

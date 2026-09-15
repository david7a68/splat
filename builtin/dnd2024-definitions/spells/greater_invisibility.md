---
splat_schema: 1
splat_id: b557d649-7130-5904-ad90-1d256cce72cb
splat_kind: spell
splat_key: dnd2024:spells/greater_invisibility
splat_name: Greater Invisibility
splat_origin:
  key: dnd2024:reference/spell/greater_invisibility
splat_sources:
- title: SRD 5.2.1
  locator: pp. 137
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 4
  school: illusion
  classes:
  - bard
  - sorcerer
  - wizard
  components:
  - verbal
  - somatic
  concentration: true
  duration: Concentration, up to 1 minute
  casting_time: Action
  ritual: false
  range:
    mode: touch
splat_rule:
  mechanic: dnd2024:spells/greater_invisibility
  implementation: dnd2024:spell_greater_invisibility
  parameters:
    target:
      kind: creature
      count: 1
      visible_to_caster: false
      touch: true
    condition:
      name: invisible
      duration: spell
splat_contracts:
- id: dnd2024:spell_greater_invisibility
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
  - invisible_effect
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - conditions
  - perception
  - time
splat_conversion:
  status: specified
  category: spell
  source_sha256: 3676e8f4953575cca13ea1b2191f1a045122da5b6dc6083ceb0d7736604818a8
  source_path: builtin/srd-5.2.1/spell/greater_invisibility.md
  pages:
  - 137
  engine_gaps: &id001
  - dnd2024:spell_greater_invisibility
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_1.md
  specification_sha256: 0b70b104487ac08e9d1f4d1d15f2f1524dc9b9977bfceb8e8f9b40686903bea3
splat_clauses:
- key: source_rules
  text: '# Greater Invisibility Level 4 Illusion (Bard, Sorcerer, Wizard) Casting Time: Action Range:
    Touch Components: V, S Duration: Concentration, up to 1 minute A creature you touch has the Invisible
    condition until the spell ends.'
  sha256: f34625b6650b95702ff3c06b85d248a891a5da5502a9ae2f32f940cdbb64891a
  status: contract_specified
splat_requires_mechanics: *id001
---

# Greater Invisibility

<!-- SRD page 137 -->

Level 4 Illusion (Bard, Sorcerer, Wizard)\
Casting Time: Action\
Range: Touch\
Components: V, S\
Duration: Concentration, up to 1 minute\
A creature you touch has the Invisible condition un-\
til the spell ends.\

Original layout: [[../../srd-5.2.1/pages/137|page 137]].

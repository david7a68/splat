---
splat_schema: 1
splat_id: 2de9b05c-d0ae-535b-a710-0af8929e509f
splat_kind: spell
splat_key: dnd2024:spells/stone_shape
splat_name: Stone Shape
splat_origin:
  key: dnd2024:reference/spell/stone_shape
splat_sources:
- title: SRD 5.2.1
  locator: pp. 165
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 4
  school: transmutation
  classes:
  - cleric
  - druid
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: Instantaneous
  casting_time: Action
  ritual: false
  range:
    mode: touch
  material:
    description: soft clay
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/stone_shape
  implementation: dnd2024:spell_stone_shape
  parameters:
    target:
      touch: true
      choice:
      - kind: stone_object
        maximum_size: medium
      - kind: stone_section
        maximum_each_dimension_feet: 5
    shape:
      chooser: caster
      arbitrary_shape: true
      maximum_hinges: 2
      latch_allowed: true
      finer_mechanical_detail: false
splat_contracts:
- id: dnd2024:spell_stone_shape
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
  - reshaped_stone
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - inventory
  - choices
splat_conversion:
  status: specified
  category: spell
  source_sha256: cc531df89f4de2a5011a9bfb7fc9f0c4f28a0472f08919174a4b7645b34fec07
  source_path: builtin/srd-5.2.1/spell/stone_shape.md
  pages:
  - 165
  engine_gaps: &id001
  - dnd2024:spell_stone_shape
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_1.md
  specification_sha256: 7d760c1c4725a8faf44a585caeb467c20f160aa3c226a3bc287cd8fe78dd98b7
splat_clauses:
- key: source_rules
  text: '# Stone Shape Level 4 Transmutation (Cleric, Druid, Wizard) Casting Time: Action Range: Touch
    Components: V, S, M (soft clay) Duration: Instantaneous You touch a stone object of Medium size or
    smaller or a section of stone no more than 5 feet in any dimension and form it into any shape you
    like. For example, you could shape a large rock into a weapon, statue, or coffer, or you could make
    a small passage through a wall that is 5 feet thick. You could also shape a stone door or its frame
    to seal the door shut. The object you create can have up to two hinges and a latch, but finer mechanical
    detail isn’t possible.'
  sha256: 046838134661f7f60b51caaf5fa99c4e66bfec1f6d095d82787ca45e76d30e85
  status: contract_specified
splat_requires_mechanics: *id001
---

# Stone Shape

<!-- SRD page 165 -->

Level 4 Transmutation (Cleric, Druid, Wizard)\
Casting Time: Action\
Range: Touch\
Components: V, S, M (soft clay)\
Duration: Instantaneous\
You touch a stone object of Medium size or smaller\
or a section of stone no more than 5 feet in any di-\
mension and form it into any shape you like. For ex-\
ample, you could shape a large rock into a weapon,\
statue, or coffer, or you could make a small passage\
through a wall that is 5 feet thick. You could also\
shape a stone door or its frame to seal the door shut.\
The object you create can have up to two hinges and\
a latch, but finer mechanical detail isn’t possible.\

Original layout: [[../../srd-5.2.1/pages/165|page 165]].

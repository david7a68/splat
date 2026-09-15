---
splat_schema: 1
splat_id: 0c3a1349-5ee0-5340-a9ab-d57829167f14
splat_kind: spell
splat_key: dnd2024:spells/barkskin
splat_name: Barkskin
splat_origin:
  key: dnd2024:reference/spell/barkskin
splat_sources:
- title: SRD 5.2.1
  locator: pp. 112
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 2
  school: transmutation
  classes:
  - druid
  - ranger
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: 1 hour
  casting_time: Bonus Action
  ritual: false
  range:
    mode: touch
  material:
    description: a handful of bark
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/barkskin
  implementation: dnd2024:spell_barkskin
  parameters:
    target:
      kind: creature
      count: 1
      visible_to_caster: false
      willing: true
      touch: true
    armor_class:
      minimum: 17
    appearance: bark_skin
splat_contracts:
- id: dnd2024:spell_barkskin
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
  - armor_class_effect
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
splat_conversion:
  status: specified
  category: spell
  source_sha256: ad1506d11647507a532bab927b7e743c22340a286a37b8d551885e89046cb81c
  source_path: builtin/srd-5.2.1/spell/barkskin.md
  pages:
  - 112
  engine_gaps: &id001
  - dnd2024:spell_barkskin
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_1.md
  specification_sha256: 05c64831f7d0d3b6be6b9ee9d4b81574306d8f5244e2a3ca062052eb2d9c42d8
splat_clauses:
- key: source_rules
  text: '# Barkskin Level 2 Transmutation (Druid, Ranger) Casting Time: Bonus Action Range: Touch Component:
    V, S, M (a handful of bark) Duration: 1 hour You touch a willing creature. Until the spell ends, the
    target’s skin assumes a bark-like appearance, and the target has an Armor Class of 17 if its AC is
    lower than that.'
  sha256: f7cfdeda94759a48f4a523851d41a5a51fae49a14f7374dc7827e1351f1b9119
  status: contract_specified
splat_requires_mechanics: *id001
---

# Barkskin

<!-- SRD page 112 -->

Level 2 Transmutation (Druid, Ranger)\
Casting Time: Bonus Action\
Range: Touch\
Component: V, S, M (a handful of bark)\
Duration: 1 hour\
You touch a willing creature. Until the spell ends,\
the target’s skin assumes a bark-like appearance,\
and the target has an Armor Class of 17 if its AC is\
lower than that.\

Original layout: [[../../srd-5.2.1/pages/112|page 112]].

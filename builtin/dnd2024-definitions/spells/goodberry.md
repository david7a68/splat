---
splat_schema: 1
splat_id: 36c9d75d-8e19-5ba5-81e0-57c2ffc09e87
splat_kind: spell
splat_key: dnd2024:spells/goodberry
splat_name: Goodberry
splat_origin:
  key: dnd2024:reference/spell/goodberry
splat_sources:
- title: SRD 5.2.1
  locator: pp. 137
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 1
  school: conjuration
  classes:
  - druid
  - ranger
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: 24 hours
  casting_time: Action
  ritual: false
  range:
    mode: self
  material:
    description: a sprig of mistletoe
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/goodberry
  implementation: dnd2024:spell_goodberry
  parameters:
    creation:
      kind: berry
      count: 10
      location: caster_hand
      duration: spell_duration
      uneaten_disappear_on_end: true
    eat:
      cost: bonus_action
      berries_consumed: 1
      healing_hp: 1
      nourishment_creature_days: 1
splat_contracts:
- id: dnd2024:spell_goodberry
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    choices:
      required: false
      type: choice
    items:
      required: false
      type: item_list
  outputs:
  - effects
  - items
  - health
  events:
  - spell.cast_completed
  - effect.ended
  capabilities:
  - spellcasting
  - effects
  - inventory
  - health
splat_conversion:
  status: specified
  category: spell
  source_sha256: ba835fd6005c6dd2b3b537742ecbdc117d28de4140110e0ea7d0cb6e57665d82
  source_path: builtin/srd-5.2.1/spell/goodberry.md
  pages:
  - 137
  engine_gaps: &id001
  - dnd2024:spell_goodberry
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_3.md
  specification_sha256: a273823e0a8f59200fa9b36e2b42d18c784d09436e4f2abbd7ab13a93b27f770
splat_clauses:
- key: source_rules
  text: '# Goodberry Level 1 Conjuration (Druid, Ranger) Casting Time: Action Range: Self Components:
    V, S, M (a sprig of mistletoe) Duration: 24 hours Ten berries appear in your hand and are infused
    with magic for the duration. A creature can take a Bonus Action to eat one berry. Eating a berry restores
    1 Hit Point, and the berry provides enough nourishment to sustain a creature for one day. Uneaten
    berries disappear when the spell ends.'
  sha256: b4cecb9575af639276983a31abec25f2f8a0f644a1f9c499716989b324fd883c
  status: contract_specified
splat_requires_mechanics: *id001
---

# Goodberry

<!-- SRD page 137 -->

Level 1 Conjuration (Druid, Ranger)\
Casting Time: Action\
Range: Self\
Components: V, S, M (a sprig of mistletoe)\
Duration: 24 hours\
Ten berries appear in your hand and are infused\
with magic for the duration. A creature can take a\
Bonus Action to eat one berry. Eating a berry re-\
stores 1 Hit Point, and the berry provides enough\
nourishment to sustain a creature for one day.\
Uneaten berries disappear when the spell ends.\

Original layout: [[../../srd-5.2.1/pages/137|page 137]].

---
splat_schema: 1
splat_id: c6f8f1d2-2402-5e8f-b38a-f42e141b139f
splat_kind: spell
splat_key: dnd2024:spells/nondetection
splat_name: Nondetection
splat_origin:
  key: dnd2024:reference/spell/nondetection
splat_sources:
- title: SRD 5.2.1
  locator: pp. 151
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 3
  school: abjuration
  classes:
  - bard
  - ranger
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: 8 hours
  casting_time: Action
  ritual: false
  range:
    mode: touch
  material:
    description: a pinch of diamond dust worth 25+ GP, which the spell consumes
    consumed: true
splat_rule:
  mechanic: dnd2024:spells/nondetection
  implementation: dnd2024:spell_nondetection
  parameters:
    targets:
      touched: true
      choices:
      - kind: creature
        willing: true
      - kind: place
        maximum_dimension_feet: 10
      - kind: object
        maximum_dimension_feet: 10
    protection:
      duration: spell_duration
      cannot_be_targeted_by_spell_school: divination
      cannot_be_perceived_through: magical_scrying_sensors
splat_contracts:
- id: dnd2024:spell_nondetection
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
    objects:
      required: false
      type: object_list
    locations:
      required: false
      type: location_list
    campaign:
      required: false
      type: campaign
    choices:
      required: false
      type: choice
  outputs:
  - effects
  events:
  - spell.cast_completed
  - movement.completed
  - spell.cast_pending
  capabilities:
  - spellcasting
  - effects
  - spatial_relationships
splat_conversion:
  status: specified
  category: spell
  source_sha256: 562a62b765681e72990811507ba5c84630adba9d7e20051749d94e353c52158d
  source_path: builtin/srd-5.2.1/spell/nondetection.md
  pages:
  - 151
  engine_gaps: &id001
  - dnd2024:spell_nondetection
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_3.md
  specification_sha256: 2621413203001fce1f862bc13063a7c4a5baf6b4dbb016f20b355e6784f496d3
splat_clauses:
- key: source_rules
  text: '# Nondetection Level 3 Abjuration (Bard, Ranger, Wizard) Casting Time: Action Range: Touch Components:
    V, S, M (a pinch of diamond dust worth 25+ GP, which the spell consumes) Duration: 8 hours For the
    duration, you hide a target that you touch from Divination spells. The target can be a willing creature,
    or it can be a place or an object no larger than 10 feet in any dimension. The target can’t be targeted
    by any Divination spell or perceived through magical scrying sensors.'
  sha256: 5f8ab511328832d510888a07f1cef2b7c695d4e0540fc4c67a35c8b8c219f5ce
  status: contract_specified
splat_requires_mechanics: *id001
---

# Nondetection

<!-- SRD page 151 -->

Level 3 Abjuration (Bard, Ranger, Wizard)\
Casting Time: Action\
Range: Touch\
Components: V, S, M (a pinch of diamond dust worth\
25+ GP, which the spell consumes)\
Duration: 8 hours\
For the duration, you hide a target that you touch\
from Divination spells. The target can be a willing\
creature, or it can be a place or an object no larger\
than 10 feet in any dimension. The target can’t\
be targeted by any Divination spell or perceived\
through magical scrying sensors.\

Original layout: [[../../srd-5.2.1/pages/151|page 151]].

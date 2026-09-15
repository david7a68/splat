---
splat_schema: 1
splat_id: 9c0b220a-a3f4-5c12-b884-dc929fe32d70
splat_kind: spell
splat_key: dnd2024:spells/continual_flame
splat_name: Continual Flame
splat_origin:
  key: dnd2024:reference/spell/continual_flame
splat_sources:
- title: SRD 5.2.1
  locator: pp. 119
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 2
  school: evocation
  classes:
  - cleric
  - druid
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: Until dispelled
  casting_time: Action
  ritual: false
  range:
    mode: touch
  material:
    description: ruby dust worth 50+ GP, which the spell consumes
    consumed: true
splat_rule:
  mechanic: dnd2024:spells/continual_flame
  implementation: dnd2024:spell_continual_flame
  parameters:
    target:
      kind: object
      count: 1
      visible_to_caster: false
      touch: true
    light:
      bright_radius_feet: 20
      additional_dim_feet: 20
    flame:
      heat: false
      fuel_consumption: false
      coverable: true
      hideable: true
      smotherable: false
      quenchable: false
splat_contracts:
- id: dnd2024:spell_continual_flame
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
  - light_effect
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - perception
  - inventory
splat_conversion:
  status: specified
  category: spell
  source_sha256: 5aed7e8acec1a665e81803324d2809a5b6ec440e57ce0aa339f1c344e07f6cac
  source_path: builtin/srd-5.2.1/spell/continual_flame.md
  pages:
  - 119
  engine_gaps: &id001
  - dnd2024:spell_continual_flame
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_1.md
  specification_sha256: ddd674e17439a57269fdcb628199bc491528ececd82f4c103ffd111f13284086
splat_clauses:
- key: source_rules
  text: '# Continual Flame Level 2 Evocation (Cleric, Druid, Wizard) Casting Time: Action Range: Touch
    Components: V, S, M (ruby dust worth 50+ GP, which the spell consumes) Duration: Until dispelled A
    flame springs from an object that you touch. The effect casts Bright Light in a 20-foot radius and
    Dim Light for an additional 20 feet. It looks like a regular flame, but it creates no heat and consumes
    no fuel. The flame can be covered or hidden but not smothered or quenched.'
  sha256: 83bdd8afb38eae4cbd590c204368d34c5fa2c0dd33f23f53c7e64bbf512b828b
  status: contract_specified
splat_requires_mechanics: *id001
---

# Continual Flame

<!-- SRD page 119 -->

Level 2 Evocation (Cleric, Druid, Wizard)\
Casting Time: Action\
Range: Touch\
Components: V, S, M (ruby dust worth 50+ GP, which\
the spell consumes)\
Duration: Until dispelled\
A flame springs from an object that you touch. The\
effect casts Bright Light in a 20-foot radius and Dim\
Light for an additional 20 feet. It looks like a regular\
flame, but it creates no heat and consumes no fuel.\
The flame can be covered or hidden but not smoth-\
ered or quenched.\

Original layout: [[../../srd-5.2.1/pages/119|page 119]].

---
splat_schema: 1
splat_id: a179fbfa-9463-5466-9827-e2e0c45259e7
splat_kind: spell
splat_key: dnd2024:spells/light
splat_name: Light
splat_origin:
  key: dnd2024:reference/spell/light
splat_sources:
- title: SRD 5.2.1
  locator: pp. 144
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 0
  school: evocation
  classes:
  - bard
  - cleric
  - sorcerer
  - wizard
  components:
  - verbal
  - material
  concentration: false
  duration: 1 hour
  casting_time: Action
  ritual: false
  range:
    mode: touch
  material:
    description: a firefly or phosphorescent moss
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/light
  implementation: dnd2024:spell_light
  parameters:
    target:
      kind: object
      count: 1
      visible_to_caster: false
      touch: true
      maximum_size: large
      not_worn_or_carried_by_other: true
    light:
      bright_radius_feet: 20
      additional_dim_feet: 20
      color_chooser: caster
      opaque_cover_blocks: true
    end:
      event: caster.recasts_same_spell
splat_contracts:
- id: dnd2024:spell_light
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
  - choices
splat_conversion:
  status: specified
  category: spell
  source_sha256: 4cebf1ca3ed3e5cb5c1df6320ddcdee0df9f1311424478a8c9608035d3fbd492
  source_path: builtin/srd-5.2.1/spell/light.md
  pages:
  - 144
  engine_gaps: &id001
  - dnd2024:spell_light
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_1.md
  specification_sha256: 1340a64aad726ed9eee945abe603c27878cfa29919c7bc06016faa20bf61b10e
splat_clauses:
- key: source_rules
  text: '# Light Evocation Cantrip (Bard, Cleric, Sorcerer, Wizard) Casting Time: Action Range: Touch
    Components: V, M (a firefly or phosphorescent moss) Duration: 1 hour You touch one Large or smaller
    object that isn’t being worn or carried by someone else. Until the spell ends, the object sheds Bright
    Light in a 20-foot radius and Dim Light for an additional 20 feet. The light can be colored as you
    like. Covering the object with something opaque blocks the light. The spell ends if you cast it again.'
  sha256: 4bfe44970232dab098a206c4336544347151694cb940dc9eaf168326e6230de7
  status: contract_specified
splat_requires_mechanics: *id001
---

# Light

<!-- SRD page 144 -->

Evocation Cantrip (Bard, Cleric, Sorcerer, Wizard)\
Casting Time: Action\
Range: Touch\
Components: V, M (a firefly or phosphorescent moss)\
Duration: 1 hour\
You touch one Large or smaller object that isn’t\
being worn or carried by someone else. Until the\
spell ends, the object sheds Bright Light in a 20-foot\
radius and Dim Light for an additional 20 feet. The\
light can be colored as you like.\
Covering the object with something opaque blocks\
the light. The spell ends if you cast it again.\

Original layout: [[../../srd-5.2.1/pages/144|page 144]].

---
splat_schema: 1
splat_id: 25f83e04-8d94-5821-bd9f-8febd5436a55
splat_kind: spell
splat_key: dnd2024:spells/guidance
splat_name: Guidance
splat_origin:
  key: dnd2024:reference/spell/guidance
splat_sources:
- title: SRD 5.2.1
  locator: pp. 138
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 0
  school: divination
  classes:
  - cleric
  - druid
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
  mechanic: dnd2024:spells/guidance
  implementation: dnd2024:spell_guidance
  parameters:
    target:
      kind: creature
      count: 1
      visible_to_caster: false
      willing: true
      touch: true
    skill_choice:
      chooser: caster
      selection: skill
    bonus:
      roll: ability_check
      requires_skill: chosen
      amount:
        dice:
          count: 1
          sides: 4
          add: 0
      all_matching_checks: true
splat_contracts:
- id: dnd2024:spell_guidance
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    save_dc:
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
  - skill_bonus_effect
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - choices
  - checks
splat_conversion:
  status: specified
  category: spell
  source_sha256: ba37f5684bf59cd709200142e16948bc513803b48daa332f93d69186c0a55498
  source_path: builtin/srd-5.2.1/spell/guidance.md
  pages:
  - 138
  engine_gaps: &id001
  - dnd2024:spell_guidance
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_1.md
  specification_sha256: 9708248767dcc54160c9b4f02f98a77c0a19ec49c1eaa392c02cc3302b77a437
splat_clauses:
- key: source_rules
  text: '# Guidance Divination Cantrip (Cleric, Druid) Casting Time: Action Range: Touch Component: V,
    S Duration: Concentration, up to 1 minute You touch a willing creature and choose a skill. Until the
    spell ends, the creature adds 1d4 to any ability check using the chosen skill.'
  sha256: d97a6700c56d7b246d6827e2d41d9ce2abc06c4b64a72a443000a60d795272d4
  status: contract_specified
splat_requires_mechanics: *id001
---

# Guidance

<!-- SRD page 138 -->

Divination Cantrip (Cleric, Druid)\
Casting Time: Action\
Range: Touch\
Component: V, S\
Duration: Concentration, up to 1 minute\
You touch a willing creature and choose a skill. Until\
the spell ends, the creature adds 1d4 to any ability\
check using the chosen skill.\

Original layout: [[../../srd-5.2.1/pages/138|page 138]].

---
splat_schema: 1
splat_id: cd5c201b-e2f5-53a7-8358-133b1488bffa
splat_kind: spell
splat_key: dnd2024:spells/heroes_feast
splat_name: Heroes’ Feast
splat_origin:
  key: dnd2024:reference/spell/heroes_feast
splat_sources:
- title: SRD 5.2.1
  locator: pp. 140
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 6
  school: conjuration
  classes:
  - bard
  - cleric
  - druid
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: Instantaneous
  casting_time: 10 minutes
  ritual: false
  range:
    mode: self
  material:
    description: a gem-encrusted bowl worth 1,000+ GP, which the spell consumes
    consumed: true
splat_rule:
  mechanic: dnd2024:spells/heroes_feast
  implementation: dnd2024:spell_heroes_feast
  parameters:
    spell_lifetime:
      concentration: false
      instantaneous: true
    feast:
      appears_on: surface
      space:
        unoccupied: true
        cube_side_feet: 10
        adjacent_to_caster: true
      consume_seconds: 3600
      disappears_after_seconds: 3600
      maximum_participants: 12
    benefits_start: consumption_hour_completed
    benefits:
      duration_seconds: 86400
      damage_resistance: poison
      condition_immunities:
      - frightened
      - poisoned
      hit_point_maximum_increase:
        dice: 2d10
      hit_points_gained: same_as_maximum_increase
splat_contracts:
- id: dnd2024:spell_heroes_feast
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    origin:
      type: location
      required: true
    participants:
      type: participant_list
      required: true
  outputs:
  - effects
  - healing
  - objects
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - world_objects
  - conditions
  - healing
  - campaign_time
splat_conversion:
  status: specified
  category: spell
  source_sha256: 88c793314401321113f4456540d9524ec640e73f0b85f49e2932d164318b1ce5
  source_path: builtin/srd-5.2.1/spell/heroes_feast.md
  pages:
  - 140
  engine_gaps: &id001
  - dnd2024:spell_heroes_feast
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_4.md
  specification_sha256: 8f025a48a6fd66627459e487bd91d0b8569671fdd10ea3a5828ae88fa161b997
splat_clauses:
- key: source_rules
  text: '# Heroes’ Feast Level 6 Conjuration (Bard, Cleric, Druid) Casting Time: 10 minutes Range: Self
    Components: V, S, M (a gem-encrusted bowl worth 1,000+ GP, which the spell consumes) Duration: Instantaneous
    You conjure a feast that appears on a surface in an unoccupied 10-foot Cube next to you. The feast
    takes 1 hour to consume and disappears at the end of that time, and the beneficial effects don’t set
    in until this hour is over. Up to twelve creatures can partake of the feast. A creature that partakes
    gains several benefits, which last for 24 hours. The creature has Resistance to Poison damage, and
    it has Immunity to the Frightened and Poisoned conditions. Its Hit Point maximum also increases by
    2d10, and it gains the same number of Hit Points.'
  sha256: 1aa3a4f7af27294f14292817817ec8c2d70d8a73d6dc90d78a8766d2c81c2774
  status: contract_specified
splat_requires_mechanics: *id001
---

# Heroes’ Feast

<!-- SRD page 140 -->

Level 6 Conjuration (Bard, Cleric, Druid)\
Casting Time: 10 minutes\
Range: Self\
Components: V, S, M (a gem-encrusted bowl worth\
1,000+ GP, which the spell consumes)\
Duration: Instantaneous\
You conjure a feast that appears on a surface in\
an unoccupied 10-foot Cube next to you. The feast\
takes 1 hour to consume and disappears at the end\
of that time, and the beneficial effects don’t set in\
until this hour is over. Up to twelve creatures can\
partake of the feast.\
A creature that partakes gains several benefits,\
which last for 24 hours. The creature has Resis-\
tance to Poison damage, and it has Immunity to the\
Frightened and Poisoned conditions. Its Hit Point\
maximum also increases by 2d10, and it gains the\
same number of Hit Points.\

Original layout: [[../../srd-5.2.1/pages/140|page 140]].

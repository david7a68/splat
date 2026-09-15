---
splat_schema: 1
splat_id: e263103d-2edf-52c4-a9bd-818245b149c3
splat_kind: spell
splat_key: dnd2024:spells/simulacrum
splat_name: Simulacrum
splat_origin:
  key: dnd2024:reference/spell/simulacrum
splat_sources:
- title: SRD 5.2.1
  locator: pp. 162, 163
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 7
  school: illusion
  classes:
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: Until dispelled
  casting_time: 12 hours
  ritual: false
  range:
    mode: touch
  material:
    description: powdered ruby worth 1,500+ GP, which the spell consumes
    consumed: true
splat_rule:
  mechanic: dnd2024:spells/simulacrum
  implementation: dnd2024:spell_simulacrum
  parameters:
    targeting:
      kind: creature
      count: 1
      types:
      - beast
      - humanoid
      within_caster_feet_for_entire_cast: 10
      final_touch:
      - target
      - same_size_ice_or_snow_pile
    creation:
      transforms: ice_or_snow_pile
      statistics: original_at_casting
      replace_type: construct
      hp_maximum_fraction:
        numerator: 1
        denominator: 2
      cannot_cast: simulacrum
      duration: spell_duration
    behavior:
      attitude: friendly
      friendly_toward: caster_and_designated_creatures
      obeys: caster_commands
      combat_turn: caster_turn
    restrictions:
      gain_levels: false
      short_rest: false
      long_rest: false
    repair:
      only_hp_restoration_method: true
      timing: caster_long_rest
      consumed_component_gp_per_hp: 100
      distance_to_caster_max_feet: 5
      throughout_repair: true
    termination:
      zero_hp: revert_to_snow_and_melt
      caster_recasts_spell: instantly_destroy_previous_simulacra
    spell_lifetime:
      concentration: false
      maximum:
        kind: until_dispelled
splat_contracts:
- id: dnd2024:spell_simulacrum
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
      type: object_list
      required: true
    forms:
      type: definition_list
      required: true
  outputs:
  - effects
  - participants
  events:
  - spell.cast_completed
  - rest.completed
  - time.elapsed
  - health.changed
  - effect.ended
  capabilities:
  - spellcasting
  - effects
  - participant_lifecycle
  - forms
  - rest_lifecycle
  - controlled_participants
splat_conversion:
  status: specified
  category: spell
  source_sha256: 94181e898b1accb1115496079452269124ca6bc1aad5d01156599610d44bebd4
  source_path: builtin/srd-5.2.1/spell/simulacrum.md
  pages:
  - 162
  - 163
  engine_gaps: &id001
  - dnd2024:spell_simulacrum
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_2.md
  specification_sha256: 5218eb9c2655f905ebe0c807b4455699ce484183bcb4f7c8f9a8a0c5dd2c1fe7
splat_clauses:
- key: source_rules
  text: '# Simulacrum Level 7 Illusion (Wizard) Casting Time: 12 hours Range: Touch Components: V, S,
    M (powdered ruby worth 1,500+ GP, which the spell consumes) Duration: Until dispelled You create a
    simulacrum of one Beast or Humanoid that is within 10 feet of you for the entire casting of the spell.
    You finish the casting by touching both the creature and a pile of ice or snow that is the same size
    as that creature, and the pile turns into the simulacrum, which is a creature. It uses the game statistics
    of the original creature at the time of casting, except it is a Construct, its Hit Point maximum is
    half as much, and it can’t cast this spell. The simulacrum is Friendly to you and creatures you designate.
    It obeys your commands and acts on your turn in combat. The simulacrum can’t gain levels, and it can’t
    take Short or Long Rests. If the simulacrum takes damage, the only way to restore its Hit Points is
    to repair it as you take a Long Rest, during which you expend components worth 100 GP per Hit Point
    restored. The simulacrum must stay within 5 feet of you for the repair. The simulacrum lasts until
    it drops to 0 Hit Points, at which point it reverts to snow and melts away. If you cast this spell
    again, any simulacrum you created with this spell is instantly destroyed.'
  sha256: ebb6b7669590c0dc1f783e5bcdc0cbb9386f186bbc8504d903bfffc3f6ae8828
  status: contract_specified
splat_requires_mechanics: *id001
---

# Simulacrum

<!-- SRD page 162 -->

Level 7 Illusion (Wizard)\
Casting Time: 12 hours\
Range: Touch\
Components: V, S, M (powdered ruby worth 1,500+ GP,\
which the spell consumes)\
Duration: Until dispelled\

<!-- SRD page 163 -->

You create a simulacrum of one Beast or Humanoid\
that is within 10 feet of you for the entire casting of\
the spell. You finish the casting by touching both the\
creature and a pile of ice or snow that is the same\
size as that creature, and the pile turns into the\
simulacrum, which is a creature. It uses the game\
statistics of the original creature at the time of cast-\
ing, except it is a Construct, its Hit Point maximum\
is half as much, and it can’t cast this spell.\
The simulacrum is Friendly to you and creatures\
you designate. It obeys your commands and acts on\
your turn in combat. The simulacrum can’t gain lev-\
els, and it can’t take Short or Long Rests.\
If the simulacrum takes damage, the only way\
to restore its Hit Points is to repair it as you take a\
Long Rest, during which you expend components\
worth 100 GP per Hit Point restored. The simula-\
crum must stay within 5 feet of you for the repair.\
The simulacrum lasts until it drops to 0 Hit Points,\
at which point it reverts to snow and melts away. If\
you cast this spell again, any simulacrum you cre-\
ated with this spell is instantly destroyed.\

Original layout: [[../../srd-5.2.1/pages/162|page 162]], [[../../srd-5.2.1/pages/163|page 163]].

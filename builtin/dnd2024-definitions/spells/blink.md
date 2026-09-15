---
splat_schema: 1
splat_id: ed2f014c-4bfa-5ece-a9c2-dee0563e61d9
splat_kind: spell
splat_key: dnd2024:spells/blink
splat_name: Blink
splat_origin:
  key: dnd2024:reference/spell/blink
splat_sources:
- title: SRD 5.2.1
  locator: pp. 113, 114
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 3
  school: transmutation
  classes:
  - sorcerer
  - wizard
  components:
  - verbal
  - somatic
  concentration: false
  duration: 1 minute
  casting_time: Action
  ritual: false
  range:
    mode: self
splat_rule:
  mechanic: dnd2024:spells/blink
  implementation: dnd2024:spell_blink
  parameters:
    targeting:
      kind: self
    duration: spell_duration
    departure:
      event: caster.turn_ended
      roll: 1d6
      on_results:
      - 4
      - 5
      - 6
      destination_plane: ethereal
      already_ethereal: end_spell_immediately
    ethereal_state:
      perceive_original_plane_feet: 60
      original_plane_color: gray
      interaction_creatures: ethereal_only
      visible_from_original_plane: requires_ethereal_perception
    return:
      events:
      - caster.turn_started
      - spell.ended
      when: on_ethereal_plane
      destination_plane: original
      choose_visible_unoccupied_within_departure_feet: 10
      fallback: nearest_unoccupied_space
    spell_lifetime:
      concentration: false
      maximum:
        value: 1
        unit: minute
splat_contracts:
- id: dnd2024:spell_blink
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
  outputs:
  - effects
  events:
  - spell.cast_completed
  - turn.started
  - turn.ended
  - position.changed
  - effect.ended
  capabilities:
  - spellcasting
  - effects
  - spatial_effects
  - planar_travel
  - perception
splat_conversion:
  status: specified
  category: spell
  source_sha256: 43df2f0eeb08df3a9f69c4ce4ed93ab65222158641a154afd5c2847f96861b7f
  source_path: builtin/srd-5.2.1/spell/blink.md
  pages:
  - 113
  - 114
  engine_gaps: &id001
  - dnd2024:spell_blink
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_2.md
  specification_sha256: 47b350bb7d085285fc505e3ba3139b885e9ee410919862e2ff473033a88a11fa
splat_clauses:
- key: source_rules
  text: '# Blink Level 3 Transmutation (Sorcerer, Wizard) Casting Time: Action Range: Self Components:
    V, S Duration: 1 minute Roll 1d6 at the end of each of your turns for the duration. On a roll of 4–6,
    you vanish from your current plane of existence and appear in the Ethereal Plane (the spell ends instantly
    if you are already on that plane). While on the Ethereal Plane, you can perceive the plane you left,
    which is cast in shades of gray, but you can’t see anything there more than 60 feet away. You can
    affect and be affected only by other creatures on the Ethereal Plane, and creatures on the other plane
    can’t perceive you unless they have a special ability that lets them perceive things on the Ethereal
    Plane. You return to the other plane at the start of your next turn and when the spell ends if you
    are on the Ethereal Plane. You return to an unoccupied space of your choice that you can see within
    10 feet of the space you left. If no unoccupied space is available within that range, you appear in
    the nearest unoccupied space.'
  sha256: 00467de4e21e88eddbea862ed0fb579de3c89cb5de1fe62eceb1b1133875fb4a
  status: contract_specified
splat_requires_mechanics: *id001
---

# Blink

<!-- SRD page 113 -->

Level 3 Transmutation (Sorcerer, Wizard)\
Casting Time: Action\
Range: Self\
Components: V, S\
Duration: 1 minute\
Roll 1d6 at the end of each of your turns for the\
duration. On a roll of 4–6, you vanish from your cur-\
rent plane of existence and appear in the Ethereal\
Plane (the spell ends instantly if you are already on\
that plane). While on the Ethereal Plane, you can\

<!-- SRD page 114 -->

perceive the plane you left, which is cast in shades\
of gray, but you can’t see anything there more than\
60 feet away. You can affect and be affected only\
by other creatures on the Ethereal Plane, and crea-\
tures on the other plane can’t perceive you unless\
they have a special ability that lets them perceive\
things on the Ethereal Plane.\
You return to the other plane at the start of your\
next turn and when the spell ends if you are on the\
Ethereal Plane. You return to an unoccupied space\
of your choice that you can see within 10 feet of the\
space you left. If no unoccupied space is available\
within that range, you appear in the nearest unoc-\
cupied space.\

Original layout: [[../../srd-5.2.1/pages/113|page 113]], [[../../srd-5.2.1/pages/114|page 114]].

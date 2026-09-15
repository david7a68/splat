---
splat_schema: 1
splat_id: c193e0c7-d054-5703-8445-d8aec30d9827
splat_kind: spell
splat_key: dnd2024:spells/true_resurrection
splat_name: True Resurrection
splat_origin:
  key: dnd2024:reference/spell/true_resurrection
splat_sources:
- title: SRD 5.2.1
  locator: pp. 170, 171
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 9
  school: necromancy
  classes:
  - cleric
  - druid
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: Instantaneous
  casting_time: 1 hour
  ritual: false
  range:
    mode: touch
  material:
    description: diamonds worth 25,000+ GP, which the spell consumes
    consumed: true
splat_rule:
  mechanic: dnd2024:spells/true_resurrection
  implementation: dnd2024:spell_true_resurrection
  parameters:
    target:
      dead_maximum_years: 200
      death_by_old_age: false
      touch_required_if_body_exists: true
    revive:
      hp: full
      close_all_wounds: true
      neutralize_poison: true
      cure_all_magical_contagions: true
      lift_curses_at_death: true
      replace_damaged_missing:
      - organs
      - limbs
      if_undead: restore_nonundead_form
    missing_body:
      create_new_body: true
      speak_name_required: true
      appearance_space: caster_chosen_unoccupied
      within_caster_feet: 10
splat_contracts:
- id: dnd2024:spell_true_resurrection
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
  - revived_creature
  - restored_body
  - removed_afflictions
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - damage
  - healing
  - forms
  - choices
splat_conversion:
  status: specified
  category: spell
  source_sha256: 2626ee515044b0af367b52d166e5b784af49fb48c16051547201b82e797b2957
  source_path: builtin/srd-5.2.1/spell/true_resurrection.md
  pages:
  - 170
  - 171
  engine_gaps: &id001
  - dnd2024:spell_true_resurrection
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_1.md
  specification_sha256: 465134c8ff16b260cb6d44289482046bae274f1d8c121eb1dbe9e46184ea31c7
splat_clauses:
- key: source_rules
  text: '# True Resurrection Level 9 Necromancy (Cleric, Druid) Casting Time: 1 hour Range: Touch Components:
    V, S, M (diamonds worth 25,000+ GP, which the spell consumes) Duration: Instantaneous You touch a
    creature that has been dead for no longer than 200 years and that died for any reason except old age.
    The creature is revived with all its Hit Points. This spell closes all wounds, neutralizes any poison,
    cures all magical contagions, and lifts any curses affecting the creature when it died. The spell
    replaces damaged or missing organs and limbs. If the creature was Undead, it is restored to its non-Undead
    form. The spell can provide a new body if the original no longer exists, in which case you must speak
    the creature’s name. The creature then appears in an unoccupied space you choose within 10 feet of
    you.'
  sha256: 0370e333d3bb4102f9a87d43ec71e86898ecd7b126b9675beb9abfcfc878f81b
  status: contract_specified
splat_requires_mechanics: *id001
---

# True Resurrection

<!-- SRD page 170 -->

Level 9 Necromancy (Cleric, Druid)\
Casting Time: 1 hour\
Range: Touch\
Components: V, S, M (diamonds worth 25,000+ GP,\
which the spell consumes)\
Duration: Instantaneous\
You touch a creature that has been dead for no\
longer than 200 years and that died for any reason\
except old age. The creature is revived with all its\
Hit Points.\
This spell closes all wounds, neutralizes any\
poison, cures all magical contagions, and lifts any\
curses affecting the creature when it died. The spell\
replaces damaged or missing organs and limbs.\
If the creature was Undead, it is restored to its\
non-Undead form.\
The spell can provide a new body if the original\
no longer exists, in which case you must speak the\

<!-- SRD page 171 -->

creature’s name. The creature then appears in an\
unoccupied space you choose within 10 feet of you.\

Original layout: [[../../srd-5.2.1/pages/170|page 170]], [[../../srd-5.2.1/pages/171|page 171]].

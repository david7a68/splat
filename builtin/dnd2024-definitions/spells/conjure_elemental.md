---
splat_schema: 1
splat_id: bcfa9253-12b1-553a-bc7c-eaf05c6eb8ec
splat_kind: spell
splat_key: dnd2024:spells/conjure_elemental
splat_name: Conjure Elemental
splat_origin:
  key: dnd2024:reference/spell/conjure_elemental
splat_sources:
- title: SRD 5.2.1
  locator: pp. 118
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 5
  school: conjuration
  classes:
  - druid
  - wizard
  components:
  - verbal
  - somatic
  concentration: true
  duration: Concentration, up to 10 minutes
  casting_time: Action
  ritual: false
  range:
    feet: 60
splat_rule:
  mechanic: dnd2024:spells/conjure_elemental
  implementation: dnd2024:spell_conjure_elemental
  parameters:
    targeting:
      kind: location
      within_range: true
      unoccupied: true
    spirit:
      size: large
      intangible: true
      duration: spell_duration
      element_damage_choice:
        air: lightning
        earth: thunder
        fire: fire
        water: cold
    capture:
      events:
      - creature.enters_spirit_space
      - creature.starts_turn_within_5_feet
      target_visible_to_caster: true
      optional: true
      requires: spirit_has_no_restrained_creature
      save:
        ability: dexterity
        failure:
          damage:
            dice: 8d8
            type: chosen_element_damage
          condition: restrained
          until: spell_ends_or_repeat_save_success
    restrained_repeat:
      event: target.turn_started
      save:
        ability: dexterity
        failure:
          damage:
            dice: 4d8
            type: chosen_element_damage
        success:
          remove: spirit_restrained
    slot_scaling:
      above: 5
      capture_damage_per_level: 1d8
      repeat_damage_per_level: 1d8
    spell_lifetime:
      concentration: true
      maximum:
        value: 10
        unit: minute
splat_contracts:
- id: dnd2024:spell_conjure_elemental
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
    locations:
      type: location_list
      required: true
    save_dc:
      type: integer
      required: true
    choices:
      type: choice
      required: true
  outputs:
  - effects
  - damage_results
  - locations
  events:
  - spell.cast_completed
  - turn.started
  - area.entered
  - position.changed
  - effect.ended
  capabilities:
  - spellcasting
  - effects
  - damage
  - conditions
  - spatial_effects
splat_conversion:
  status: specified
  category: spell
  source_sha256: 8c241874cfeb5d3411ddf80fef35474191de3f10ffb0adbb90f80f204bc44cd7
  source_path: builtin/srd-5.2.1/spell/conjure_elemental.md
  pages:
  - 118
  engine_gaps: &id001
  - dnd2024:spell_conjure_elemental
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_2.md
  specification_sha256: 01918ada980d86105f43660d6625f9dbdbf5df446bfa1349640851e047aeac94
splat_clauses:
- key: source_rules
  text: '# Conjure Elemental Level 5 Conjuration (Druid, Wizard) Casting Time: Action Range: 60 feet Components:
    V, S Duration: Concentration, up to 10 minutes You conjure a Large, intangible spirit from the Elemental
    Planes that appears in an unoccupied space within range. Choose the spirit’s element, which determines
    its damage type: air (Lightning), earth (Thunder), fire (Fire), or water (Cold). The spirit lasts
    for the duration. Whenever a creature you can see enters the spirit’s space or starts its turn within
    5 feet of the spirit, you can force that creature to make a Dexterity saving throw if the spirit has
    no creature Restrained. On failed save, the target takes 8d8 damage of the spirit’s type, and the
    target has the Restrained condition until the spell ends. At the start of each of its turns, the Restrained
    target repeats the save. On a failed save, the target takes 4d8 damage of the spirit’s type. On a
    successful save, the target isn’t Restrained by the spirit. Using a Higher-Level Spell Slot. The damage
    increases by 1d8 for each spell slot level above 5.'
  sha256: 512788ce824724659d3404329e876dafb1dd32df269dd5dc427ae5e3f0785801
  status: contract_specified
splat_requires_mechanics: *id001
---

# Conjure Elemental

<!-- SRD page 118 -->

Level 5 Conjuration (Druid, Wizard)\
Casting Time: Action\
Range: 60 feet\
Components: V, S\
Duration: Concentration, up to 10 minutes\
You conjure a Large, intangible spirit from the Ele-\
mental Planes that appears in an unoccupied space\
within range. Choose the spirit’s element, which\
determines its damage type: air (Lightning), earth\
(Thunder), fire (Fire), or water (Cold). The spirit\
lasts for the duration.\
Whenever a creature you can see enters the spir-\
it’s space or starts its turn within 5 feet of the spirit,\
you can force that creature to make a Dexterity sav-\
ing throw if the spirit has no creature Restrained.\
On failed save, the target takes 8d8 damage of the\
spirit’s type, and the target has the Restrained\
condition until the spell ends. At the start of each\
of its turns, the Restrained target repeats the save.\
On a failed save, the target takes 4d8 damage of the\
spirit’s type. On a successful save, the target isn’t\
Restrained by the spirit.\
Using a Higher-Level Spell Slot. The damage in-\
creases by 1d8 for each spell slot level above 5.\

Original layout: [[../../srd-5.2.1/pages/118|page 118]].

---
splat_schema: 1
splat_id: f9a9d4eb-f7e1-58a8-bc34-0dbfcd22c233
splat_kind: spell
splat_key: dnd2024:spells/spiritual_weapon
splat_name: Spiritual Weapon
splat_origin:
  key: dnd2024:reference/spell/spiritual_weapon
splat_sources:
- title: SRD 5.2.1
  locator: pp. 165
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 2
  school: evocation
  classes:
  - cleric
  components:
  - verbal
  - somatic
  concentration: true
  duration: Concentration, up to 1 minute
  casting_time: Bonus Action
  ritual: false
  range:
    feet: 60
splat_rule:
  mechanic: dnd2024:spells/spiritual_weapon
  implementation: dnd2024:spell_spiritual_weapon
  parameters:
    targeting:
      kind: force_location
      within_range: true
      choice: caster
    force:
      floating: true
      spectral: true
      weapon_appearance: caster_choice
      duration: spell_duration
    attack:
      initial: optional_immediately_on_cast
      kind: melee_spell
      target: one_creature
      within_force_feet: 5
      hit:
        damage:
          dice: 1d8
          add:
            stat: caster_spellcasting_ability_modifier
          type: force
    repeat:
      economy: bonus_action
      timing: later_caster_turns
      movement_max_feet: 20
      repeat_attack: true
    slot_scaling:
      above: 2
      damage_per_level: 1d8
    spell_lifetime:
      concentration: true
      maximum:
        value: 1
        unit: minute
splat_contracts:
- id: dnd2024:spell_spiritual_weapon
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
    choices:
      type: choice
      required: true
  outputs:
  - effects
  - damage_results
  - locations
  events:
  - spell.cast_completed
  - effect.ended
  capabilities:
  - spellcasting
  - effects
  - damage
  - attacks
  - spatial_effects
splat_conversion:
  status: specified
  category: spell
  source_sha256: e4a5d86ac03d03156546c61fa437e25a02895c1f6b5d3ce3e4319c44687e4d89
  source_path: builtin/srd-5.2.1/spell/spiritual_weapon.md
  pages:
  - 165
  engine_gaps: &id001
  - dnd2024:spell_spiritual_weapon
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_2.md
  specification_sha256: 1ecc38ee86483c5cfeb74c061a7fa1607977ef1afa496b0a87b195a5e234e25c
splat_clauses:
- key: source_rules
  text: '# Spiritual Weapon Level 2 Evocation (Cleric) Casting Time: Bonus Action Range: 60 feet Components:
    V, S Duration: Concentration, up to 1 minute You create a floating, spectral force that resembles
    a weapon of your choice and lasts for the duration. The force appears within range in a space of your
    choice, and you can immediately make one melee spell attack against one creature within 5 feet of
    the force. On a hit, the target takes Force damage equal to 1d8 plus your spellcasting ability modifier.
    As a Bonus Action on your later turns, you can move the force up to 20 feet and repeat the attack
    against a creature within 5 feet of it. Using a Higher-Level Spell Slot. The damage increases by 1d8
    for every slot level above 2.'
  sha256: 95c2b67383cc382724aab07ad8954292506941b5a4cb73f85dab9d70fe53c13c
  status: contract_specified
splat_requires_mechanics: *id001
---

# Spiritual Weapon

<!-- SRD page 165 -->

Level 2 Evocation (Cleric)\
Casting Time: Bonus Action\
Range: 60 feet\
Components: V, S\
Duration: Concentration, up to 1 minute\
You create a floating, spectral force that resembles\
a weapon of your choice and lasts for the duration.\
The force appears within range in a space of your\
choice, and you can immediately make one melee\
spell attack against one creature within 5 feet of the\
force. On a hit, the target takes Force damage equal\
to 1d8 plus your spellcasting ability modifier.\
As a Bonus Action on your later turns, you can\
move the force up to 20 feet and repeat the attack\
against a creature within 5 feet of it.\
Using a Higher-Level Spell Slot. The damage in-\
creases by 1d8 for every slot level above 2.\

Original layout: [[../../srd-5.2.1/pages/165|page 165]].

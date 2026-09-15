---
splat_schema: 1
splat_id: b2e653d1-62be-5c31-b6cd-e16c63e1ced0
splat_kind: spell
splat_key: dnd2024:spells/finger_of_death
splat_name: Finger of Death
splat_origin:
  key: dnd2024:reference/spell/finger_of_death
splat_sources:
- title: SRD 5.2.1
  locator: pp. 131
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 7
  school: necromancy
  classes:
  - sorcerer
  - warlock
  - wizard
  components:
  - verbal
  - somatic
  concentration: false
  duration: Instantaneous
  casting_time: Action
  ritual: false
  range:
    feet: 60
splat_rule:
  mechanic: dnd2024:spells/finger_of_death
  implementation: dnd2024:spell_finger_of_death
  parameters:
    targeting:
      kind: creature
      count: 1
      visible: true
      within_range: true
    save:
      ability: constitution
      failure:
        damage:
          dice: 7d8
          add: 30
          type: necrotic
      success:
        damage_fraction:
          numerator: 1
          denominator: 2
    killed_humanoid:
      event: caster.next_turn_started
      form: zombie
      obeys: caster_verbal_orders
    spell_lifetime:
      concentration: false
      maximum:
        kind: instantaneous
splat_contracts:
- id: dnd2024:spell_finger_of_death
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
    forms:
      type: definition_list
      required: true
    save_dc:
      type: integer
      required: true
  outputs:
  - effects
  - damage_results
  - participants
  events:
  - spell.cast_completed
  - turn.started
  - health.changed
  capabilities:
  - spellcasting
  - effects
  - damage
  - participant_lifecycle
  - forms
splat_conversion:
  status: specified
  category: spell
  source_sha256: 54b97320b9ca24b86f6d5439139ac99477524b61ce317f27d445e7cf4f0be9f7
  source_path: builtin/srd-5.2.1/spell/finger_of_death.md
  pages:
  - 131
  engine_gaps: &id001
  - dnd2024:spell_finger_of_death
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_2.md
  specification_sha256: 334a65186e833886b381baef8c54ed0517509832f50e195324061e2123366d10
splat_clauses:
- key: source_rules
  text: '# Finger of Death Level 7 Necromancy (Sorcerer, Warlock, Wizard) Casting Time: Action Range:
    60 feet Components: V, S Duration: Instantaneous You unleash negative energy toward a creature you
    can see within range. The target makes a Constitution saving throw, taking 7d8 + 30 Necrotic damage
    on a failed save or half as much damage on a successful one. A Humanoid killed by this spell rises
    at the start of your next turn as a Zombie (see “Monsters”) that follows your verbal orders.'
  sha256: 7ef5083a00525112c160cd6ea014f8567f0a290347057040289a01fb4c9bbd37
  status: contract_specified
splat_requires_mechanics: *id001
---

# Finger of Death

<!-- SRD page 131 -->

Level 7 Necromancy (Sorcerer, Warlock, Wizard)\
Casting Time: Action\
Range: 60 feet\
Components: V, S\
Duration: Instantaneous\
You unleash negative energy toward a creature you\
can see within range. The target makes a Constitu-\
tion saving throw, taking 7d8 + 30 Necrotic damage\
on a failed save or half as much damage on a suc-\
cessful one.\
A Humanoid killed by this spell rises at the start\
of your next turn as a Zombie (see “Monsters”) that\
follows your verbal orders.\

Original layout: [[../../srd-5.2.1/pages/131|page 131]].

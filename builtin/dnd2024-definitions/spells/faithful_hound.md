---
splat_schema: 1
splat_id: 8de80790-d19b-520d-8f3a-3e6c7a91eba8
splat_kind: spell
splat_key: dnd2024:spells/faithful_hound
splat_name: Faithful Hound
splat_origin:
  key: dnd2024:reference/spell/faithful_hound
splat_sources:
- title: SRD 5.2.1
  locator: pp. 129
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 4
  school: conjuration
  classes:
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: 8 hours
  casting_time: Action
  ritual: false
  range:
    feet: 30
  material:
    description: a silver whistle
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/faithful_hound
  implementation: dnd2024:spell_faithful_hound
  parameters:
    targeting:
      kind: location
      unoccupied: true
      visible: true
      within_range: true
    hound:
      visible_to: caster_only
      intangible: true
      invulnerable: true
      truesight_feet: 30
      until_any:
      - spell_duration
      - caster_distance_exceeds_300_feet
    alarm:
      creature_size_min: small
      proximity_feet: 30
      exemption: previously_spoken_caster_specified_password
      effect: loud_barking
    bite:
      event: caster.turn_started
      target: one_enemy
      reach_feet: 5
      save:
        ability: dexterity
        failure:
          damage:
            dice: 4d8
            type: force
        success: no_effect
    move:
      economy: magic_action
      timing: later_caster_turns
      max_feet: 30
    spell_lifetime:
      concentration: false
      maximum:
        value: 8
        unit: hour
splat_contracts:
- id: dnd2024:spell_faithful_hound
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
  - spatial_effects
splat_conversion:
  status: specified
  category: spell
  source_sha256: 2e911f80affcb705d0bf616a0b72cd2bed9867f044cb6477bc63d6b0f529e0ca
  source_path: builtin/srd-5.2.1/spell/faithful_hound.md
  pages:
  - 129
  engine_gaps: &id001
  - dnd2024:spell_faithful_hound
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_2.md
  specification_sha256: af2a452b9bd3f53d2da5c8af8ff3734d67f63caf69d301a0300c62b4c32ae194
splat_clauses:
- key: source_rules
  text: '# Faithful Hound Level 4 Conjuration (Wizard) Casting Time: Action Range: 30 feet Components:
    V, S, M (a silver whistle) Duration: 8 hours You conjure a phantom watchdog in an unoccupied space
    that you can see within range. The hound remains for the duration or until the two of you are more
    than 300 feet apart from each other. No one but you can see the hound, and it is intangible and invulnerable.
    When a Small or larger creature comes within 30 feet of it without first speaking the password that
    you specify when you cast this spell, the hound starts barking loudly. The hound has Truesight with
    a range of 30 feet. At the start of each of your turns, the hound attempts to bite one enemy within
    5 feet of it. That enemy must succeed on a Dexterity saving throw or take 4d8 Force damage. On your
    later turns, you can take a Magic action to move the hound up to 30 feet.'
  sha256: 3056d8a3813ac18730f336d58f5d385984c9c0b2b1721378dcccf2923557bf6a
  status: contract_specified
splat_requires_mechanics: *id001
---

# Faithful Hound

<!-- SRD page 129 -->

Level 4 Conjuration (Wizard)\
Casting Time: Action\
Range: 30 feet\
Components: V, S, M (a silver whistle)\
Duration: 8 hours\
You conjure a phantom watchdog in an unoccupied\
space that you can see within range. The hound\
remains for the duration or until the two of you are\
more than 300 feet apart from each other.\
No one but you can see the hound, and it is in-\
tangible and invulnerable. When a Small or larger\
creature comes within 30 feet of it without first\
speaking the password that you specify when you\
cast this spell, the hound starts barking loudly. The\
hound has Truesight with a range of 30 feet.\
At the start of each of your turns, the hound at-\
tempts to bite one enemy within 5 feet of it. That\
enemy must succeed on a Dexterity saving throw or\
take 4d8 Force damage.\
On your later turns, you can take a Magic action to\
move the hound up to 30 feet.\

Original layout: [[../../srd-5.2.1/pages/129|page 129]].

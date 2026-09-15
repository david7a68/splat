---
splat_schema: 1
splat_id: 7bdf42b5-f9d6-58f3-adaf-7c46a198babb
splat_kind: spell
splat_key: dnd2024:spells/arcane_sword
splat_name: Arcane Sword
splat_origin:
  key: dnd2024:reference/spell/arcane_sword
splat_sources:
- title: SRD 5.2.1
  locator: pp. 110
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 7
  school: evocation
  classes:
  - bard
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: true
  duration: Concentration, up to 1 minute
  casting_time: Action
  ritual: false
  range:
    feet: 90
  material:
    description: a miniature sword worth 250+ GP
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/arcane_sword
  implementation: dnd2024:spell_arcane_sword
  parameters:
    creation:
      kind: spectral_sword
      hover: true
      location: within_spell_range
    attack:
      event: spell.cast_completed
      kind: melee_spell
      target:
        kind: creature_or_object
        count: 1
        visible_to_caster: false
        distance_from_sword_feet: 5
      damage:
        amount:
          dice:
            count: 4
            sides: 12
            add:
              binding: spellcasting_modifier
        type: force
    repeat:
      action: bonus_action
      timing: later_caster_turns
      move_feet: 30
      destination_visible: true
      attack: same_or_different_target
splat_contracts:
- id: dnd2024:spell_arcane_sword
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    spellcasting_modifier:
      type: integer
      required: true
    spell_attack_modifier:
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
  - sword_effect
  - attacks
  - damage
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - damage
  - attacks
  - movement
  - inventory
  - time
splat_conversion:
  status: specified
  category: spell
  source_sha256: b8c92330a48bcd1f4d09132bbc3e28b84cdbde98cacf3ace4ed3b1da1476aebf
  source_path: builtin/srd-5.2.1/spell/arcane_sword.md
  pages:
  - 110
  engine_gaps: &id001
  - dnd2024:spell_arcane_sword
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_1.md
  specification_sha256: 28d94c49640ce0108a5d481804892398e1f794401aedcfb8bccdc00052d5c2dc
splat_clauses:
- key: source_rules
  text: '# Arcane Sword Level 7 Evocation (Bard, Wizard) Casting Time: Action Range: 90 feet Components:
    V, S, M (a miniature sword worth 250+ GP) Duration: Concentration, up to 1 minute You create a spectral
    sword that hovers within range. It lasts for the duration. When the sword appears, you make a melee
    spell attack against a target within 5 feet of the sword. On a hit, the target takes Force damage
    equal to 4d12 plus your spellcasting ability modifier. On your later turns, you can take a Bonus Action
    to move the sword up to 30 feet to a spot you can see and repeat the attack against the same target
    or a different one.'
  sha256: b4453a3ff5d5bacfc7ed6a7a199aaa3a75fbff793a896cdd145f9f4c6c8252cb
  status: contract_specified
splat_requires_mechanics: *id001
---

# Arcane Sword

<!-- SRD page 110 -->

Level 7 Evocation (Bard, Wizard)\
Casting Time: Action\
Range: 90 feet\
Components: V, S, M (a miniature sword worth\
250+ GP)\
Duration: Concentration, up to 1 minute\
You create a spectral sword that hovers within\
range. It lasts for the duration.\
When the sword appears, you make a melee spell\
attack against a target within 5 feet of the sword.\
On a hit, the target takes Force damage equal to\
4d12 plus your spellcasting ability modifier.\
On your later turns, you can take a Bonus Action\
to move the sword up to 30 feet to a spot you can\
see and repeat the attack against the same target or\
a different one.\

Original layout: [[../../srd-5.2.1/pages/110|page 110]].

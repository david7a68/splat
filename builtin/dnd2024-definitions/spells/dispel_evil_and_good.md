---
splat_schema: 1
splat_id: 98c98622-a138-5960-a9f2-d491794a5a79
splat_kind: spell
splat_key: dnd2024:spells/dispel_evil_and_good
splat_name: Dispel Evil and Good
splat_origin:
  key: dnd2024:reference/spell/dispel_evil_and_good
splat_sources:
- title: SRD 5.2.1
  locator: pp. 124
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 5
  school: abjuration
  classes:
  - cleric
  - paladin
  components:
  - verbal
  - somatic
  - material
  concentration: true
  duration: Concentration, up to 1 minute
  casting_time: Action
  ritual: false
  range:
    mode: self
  material:
    description: powdered silver and iron
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/dispel_evil_and_good
  implementation: dnd2024:spell_dispel_evil_and_good
  parameters:
    spell_lifetime:
      concentration: true
      maximum_seconds: 60
    creature_types:
    - celestial
    - elemental
    - fey
    - fiend
    - undead
    protection:
      disadvantage: attack_rolls
      attacker_types: configured_types
      target: caster
    special_functions:
      economy: magic_action
      end_spell_on_use: true
      choose_one:
      - break_enchantment
      - dismissal
    break_enchantment:
      target:
        kind: creature
        touch: true
      remove_effects_from_configured_types:
      - possession
      - charmed
      - frightened
    dismissal:
      target:
        kind: creature
        visible: true
        range_feet: 5
        types: configured_types
      save:
        ability: charisma
        dc: spell_save_dc
      failure:
        if_not_on_home_plane:
          transport_to: home_plane
          overrides:
            undead: shadowfell
            fey: feywild
        if_on_home_plane: unaffected
splat_contracts:
- id: dnd2024:spell_dispel_evil_and_good
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    option:
      required: false
      type: choice
    target:
      required: false
      type: participant
    save_dc:
      type: integer
      required: true
  outputs:
  - effects
  - transports
  events:
  - concentration.ended
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - planar_travel
  - conditions
  - tests
  - concentration
splat_conversion:
  status: specified
  category: spell
  source_sha256: c4929d1fcfa7e46a0dc5f2ecc78c5cd446a8d57c8105d8f227481e403222306b
  source_path: builtin/srd-5.2.1/spell/dispel_evil_and_good.md
  pages:
  - 124
  engine_gaps: &id001
  - dnd2024:spell_dispel_evil_and_good
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_4.md
  specification_sha256: 802d7fdbfc471c6439fdfa2cbbd1799e7b5cec8986ae893a67a202d0040d4acd
splat_clauses:
- key: source_rules
  text: '# Dispel Evil and Good Level 5 Abjuration (Cleric, Paladin) Casting Time: Action Range: Self
    Components: V, S, M (powdered silver and iron) Duration: Concentration, up to 1 minute For the duration,
    Celestials, Elementals, Fey, Fiends, and Undead have Disadvantage on attack rolls against you. You
    can end the spell early by using either of the following special functions. Break Enchantment. As
    a Magic action, you touch a creature that is possessed by or has the Charmed or Frightened condition
    from one or more creatures of the types above. The target is no longer possessed, Charmed, or Frightened
    by such creatures. Dismissal. As a Magic action, you target one creature you can see within 5 feet
    of you that has one of the creature types above. The target must succeed on a Charisma saving throw
    or be sent back to its home plane if it isn’t there already. If they aren’t on their home plane, Undead
    are sent to the Shadowfell, and Fey are sent to the Feywild.'
  sha256: 9a576e43e7bcd8caae87b1eddc831147c76d28938471822e0596fbe23dfe7998
  status: contract_specified
splat_requires_mechanics: *id001
---

# Dispel Evil and Good

<!-- SRD page 124 -->

Level 5 Abjuration (Cleric, Paladin)\
Casting Time: Action\
Range: Self\
Components: V, S, M (powdered silver and iron)\
Duration: Concentration, up to 1 minute\
For the duration, Celestials, Elementals, Fey, Fiends,\
and Undead have Disadvantage on attack rolls\
against you. You can end the spell early by using ei-\
ther of the following special functions.\
Break Enchantment. As a Magic action, you touch\
a creature that is possessed by or has the Charmed\
or Frightened condition from one or more creatures\
of the types above. The target is no longer pos-\
sessed, Charmed, or Frightened by such creatures.\
Dismissal. As a Magic action, you target one crea-\
ture you can see within 5 feet of you that has one of\
the creature types above. The target must succeed\
on a Charisma saving throw or be sent back to its\
home plane if it isn’t there already. If they aren’t on\
their home plane, Undead are sent to the Shadow-\
fell, and Fey are sent to the Feywild.\

Original layout: [[../../srd-5.2.1/pages/124|page 124]].

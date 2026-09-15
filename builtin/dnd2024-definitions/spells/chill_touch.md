---
splat_schema: 1
splat_id: 828fb932-2962-5fb5-928f-2fd5e0ea624b
splat_kind: spell
splat_key: dnd2024:spells/chill_touch
splat_name: Chill Touch
splat_origin:
  key: dnd2024:reference/spell/chill_touch
splat_sources:
- title: SRD 5.2.1
  locator: pp. 115
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 0
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
    mode: touch
splat_rule:
  mechanic: dnd2024:spells/chill_touch
  implementation: dnd2024:spell_chill_touch
  parameters:
    targeting:
      kind: target
      within_reach: true
    attack:
      kind: melee_spell
      hit:
        damage:
          dice: 1d10
          type: necrotic
        cannot_regain_hp:
          until: caster_next_turn_end
    cantrip_scaling:
      character_level_dice:
        '1': 1d10
        '5': 2d10
        '11': 3d10
        '17': 4d10
    spell_lifetime:
      concentration: false
      maximum:
        kind: instantaneous
splat_contracts:
- id: dnd2024:spell_chill_touch
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
      required: false
      type: object_list
  outputs:
  - effects
  - damage_results
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - damage
  - attacks
splat_conversion:
  status: specified
  category: spell
  source_sha256: a711182fb98c509f7674ece443afc6564c5e72a212e349859a2c9602c6b27cb2
  source_path: builtin/srd-5.2.1/spell/chill_touch.md
  pages:
  - 115
  engine_gaps: &id001
  - dnd2024:spell_chill_touch
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_2.md
  specification_sha256: 6e7852592de8255781d1c0070abfcdad0baa00e42ad3941db3a30baea58492f9
splat_clauses:
- key: source_rules
  text: '# Chill Touch Necromancy Cantrip (Sorcerer, Warlock, Wizard) Casting Time: Action Range: Touch
    Components: V, S Duration: Instantaneous Channeling the chill of the grave, make a melee spell attack
    against a target within reach. On a hit, the target takes 1d10 Necrotic damage, and it can’t regain
    Hit Points until the end of your next turn. Cantrip Upgrade. The damage increases by 1d10 when you
    reach levels 5 (2d10), 11 (3d10), and 17 (4d10).'
  sha256: 770367d66ae2d00e6ce021b93d459630d52ef82583ed91d07feaf0865c202b73
  status: contract_specified
splat_requires_mechanics: *id001
---

# Chill Touch

<!-- SRD page 115 -->

Necromancy Cantrip (Sorcerer, Warlock, Wizard)\
Casting Time: Action\
Range: Touch\
Components: V, S\
Duration: Instantaneous\
Channeling the chill of the grave, make a melee spell\
attack against a target within reach. On a hit, the\
target takes 1d10 Necrotic damage, and it can’t re-\
gain Hit Points until the end of your next turn.\
Cantrip Upgrade. The damage increases by 1d10\
when you reach levels 5 (2d10), 11 (3d10), and 17\
(4d10).\

Original layout: [[../../srd-5.2.1/pages/115|page 115]].

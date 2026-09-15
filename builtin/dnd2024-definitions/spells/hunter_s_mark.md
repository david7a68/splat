---
splat_schema: 1
splat_id: 9969e83a-34fe-5bd6-bdc1-f9f4e29bb5a0
splat_kind: spell
splat_key: dnd2024:spells/hunter_s_mark
splat_name: Hunter’s Mark
splat_origin:
  key: dnd2024:reference/spell/hunter_s_mark
splat_sources:
- title: SRD 5.2.1
  locator: pp. 141
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 1
  school: divination
  classes:
  - ranger
  components:
  - verbal
  concentration: true
  duration: Concentration, up to 1 hour
  casting_time: Bonus Action
  ritual: false
  range:
    feet: 90
splat_rule:
  mechanic: dnd2024:spells/hunter_s_mark
  implementation: dnd2024:spell_hunter_s_mark
  parameters:
    targets:
      kind: creature
      maximum: 1
      within_spell_range: true
      visible: true
    mark:
      duration: spell_duration
      extra_damage:
        trigger: caster_hits_target_with_attack_roll
        damage:
          dice:
            count: 1
            sides: 6
            add: 0
          type: force
      advantage:
        checks_ability: wisdom
        skills:
        - perception
        - survival
        purpose: find_marked_target
    transfer:
      requires: marked_target_dropped_to_zero_hp_before_spell_end
      cost: bonus_action
      new_target:
        kind: creature
        maximum: 1
        within_spell_range: true
        visible: true
    duration_by_slot:
      '1_2':
        hours: 1
      '3_4':
        hours: 8
      5_plus:
        hours: 24
    concentration: true
splat_contracts:
- id: dnd2024:spell_hunter_s_mark
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
      required: false
      type: location_list
    campaign:
      required: false
      type: campaign
    choices:
      required: false
      type: choice
    trigger:
      required: false
      type: event
  outputs:
  - effects
  - damage
  events:
  - spell.cast_completed
  - attack.roll
  - attack.hit
  - movement.completed
  - hp.changed
  capabilities:
  - spellcasting
  - effects
  - dice
  - damage
  - spatial_relationships
  - roll_modifiers
splat_conversion:
  status: specified
  category: spell
  source_sha256: 8bc42023ab99e36554f72cff02c11249ff25490cf5b90d2b2856f374a1d6aed7
  source_path: builtin/srd-5.2.1/spell/hunter_s_mark.md
  pages:
  - 141
  engine_gaps: &id001
  - dnd2024:spell_hunter_s_mark
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_3.md
  specification_sha256: 4cb6ca9c3455e7da585229b4db10d19be9be8e821d11ec496913987025deff69
splat_clauses:
- key: source_rules
  text: '# Hunter’s Mark Level 1 Divination (Ranger) Casting Time: Bonus Action Range: 90 feet Components:
    V Duration: Concentration, up to 1 hour You magically mark one creature you can see within range as
    your quarry. Until the spell ends, you deal an extra 1d6 Force damage to the target whenever you hit
    it with an attack roll. You also have Advantage on any Wisdom (Perception or Survival) check you make
    to find it. If the target drops to 0 Hit Points before this spell ends, you can take a Bonus Action
    to move the mark to a new creature you can see within range. Using a Higher-Level Spell Slot. Your
    Concentration can last longer with a spell slot of level 3–4 (up to 8 hours) or 5+ (up to 24 hours).'
  sha256: 99282017a4e4465733829759fa0b57c092f5365f98c28c544a655feea5f45ddd
  status: contract_specified
splat_requires_mechanics: *id001
---

# Hunter’s Mark

<!-- SRD page 141 -->

Level 1 Divination (Ranger)\
Casting Time: Bonus Action\
Range: 90 feet\
Components: V\
Duration: Concentration, up to 1 hour\
You magically mark one creature you can see within\
range as your quarry. Until the spell ends, you deal\
an extra 1d6 Force damage to the target whenever\
you hit it with an attack roll. You also have Advan-\
tage on any Wisdom (Perception or Survival) check\
you make to find it.\
If the target drops to 0 Hit Points before this spell\
ends, you can take a Bonus Action to move the mark\
to a new creature you can see within range.\
Using a Higher-Level Spell Slot. Your Concentra-\
tion can last longer with a spell slot of level 3–4 (up\
to 8 hours) or 5+ (up to 24 hours).\

Original layout: [[../../srd-5.2.1/pages/141|page 141]].

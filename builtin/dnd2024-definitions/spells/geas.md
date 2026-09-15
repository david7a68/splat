---
splat_schema: 1
splat_id: 98dbb838-e5ce-5565-9713-e103b8a8d837
splat_kind: spell
splat_key: dnd2024:spells/geas
splat_name: Geas
splat_origin:
  key: dnd2024:reference/spell/geas
splat_sources:
- title: SRD 5.2.1
  locator: pp. 135
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 5
  school: enchantment
  classes:
  - bard
  - cleric
  - druid
  - paladin
  - wizard
  components:
  - verbal
  concentration: false
  duration: 30 days
  casting_time: 1 minute
  ritual: false
  range:
    feet: 60
splat_rule:
  mechanic: dnd2024:spells/geas
  implementation: dnd2024:spell_geas
  parameters:
    target:
      kind: creature
      count: 1
      visible_to_caster: true
    command:
      verbal: true
      choice:
      - perform_service
      - refrain_action_or_activity
      certain_death_command: end_spell
    save:
      ability: wisdom
      dc:
        binding: save_dc
      failure:
        condition: charmed
      success:
        effect: none
    automatic_success_if: cannot_understand_command
    violation:
      event: target.acts_directly_counter_to_command
      damage:
        amount:
          dice:
            count: 5
            sides: 10
            add: 0
        type: psychic
      maximum_per_day: 1
      requires: charmed_by_spell
    end_by_spells:
    - remove_curse
    - greater_restoration
    - wish
    duration_by_slot:
      '5':
        days: 30
      '6':
        days: 30
      '7':
        days: 365
      '8':
        days: 365
      '9':
        until: named_ending_spell
splat_contracts:
- id: dnd2024:spell_geas
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    save_dc:
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
  - save
  - charmed
  - command
  - violation_damage
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - damage
  - saving_throws
  - conditions
  - movement
  - forms
  - choices
  - time
  - relationships
splat_conversion:
  status: specified
  category: spell
  source_sha256: bbb69d9886f2df42e07167a14d0ea6047e285b15e58e8ea6792ed8d4358c635f
  source_path: builtin/srd-5.2.1/spell/geas.md
  pages:
  - 135
  engine_gaps: &id001
  - dnd2024:spell_geas
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_1.md
  specification_sha256: 842395d393f9043d2a06043a241ef872dda448960eff0408058a0798951aaa61
splat_clauses:
- key: source_rules
  text: '# Geas Level 5 Enchantment (Bard, Cleric, Druid, Paladin, Wizard) Casting Time: 1 minute Range:
    60 feet Components: V Duration: 30 days You give a verbal command to a creature that you can see within
    range, ordering it to carry out some service or refrain from an action or a course of activity as
    you decide. The target must succeed on a Wisdom saving throw or have the Charmed condition for the
    duration. The target automatically succeeds if it can’t understand your command. While Charmed, the
    creature takes 5d10 Psychic damage if it acts in a manner directly counter to your command. It takes
    this damage no more than once each day. You can issue any command you choose, short of an activity
    that would result in certain death. Should you issue a suicidal command, the spell ends. A Remove
    Curse, Greater Restoration, or Wish spell ends this spell. Using a Higher-Level Spell Slot. If you
    use a level 7 or 8 spell slot, the duration is 365 days. If you use a level 9 spell slot, the spell
    lasts until it is ended by one of the spells mentioned above.'
  sha256: 07f2c63f064230c4a3defe7dcc95452d89f8bee2528a99d21d3ecd9e9b173bd6
  status: contract_specified
splat_requires_mechanics: *id001
---

# Geas

<!-- SRD page 135 -->

Level 5 Enchantment (Bard, Cleric, Druid, Paladin,\
Wizard)\
Casting Time: 1 minute\
Range: 60 feet\
Components: V\
Duration: 30 days\
You give a verbal command to a creature that you\
can see within range, ordering it to carry out some\
service or refrain from an action or a course of ac-\
tivity as you decide. The target must succeed on a\
Wisdom saving throw or have the Charmed condi-\
tion for the duration. The target automatically suc-\
ceeds if it can’t understand your command.\
While Charmed, the creature takes 5d10 Psychic\
damage if it acts in a manner directly counter to\
your command. It takes this damage no more than\
once each day.\
You can issue any command you choose, short\
of an activity that would result in certain death.\
Should you issue a suicidal command, the spell\
ends.\
A Remove Curse, Greater Restoration, or Wish spell\
ends this spell.\
Using a Higher-Level Spell Slot. If you use a level\
7 or 8 spell slot, the duration is 365 days. If you use\
a level 9 spell slot, the spell lasts until it is ended by\
one of the spells mentioned above.\

Original layout: [[../../srd-5.2.1/pages/135|page 135]].

---
splat_schema: 1
splat_id: 071dc943-e60f-5646-9efe-315208184a24
splat_kind: rule
splat_key: dnd2024:class_features/oath_of_devotion_level_20_holy_nimbus
splat_name: 'Level 20: Holy Nimbus'
splat_origin:
  key: dnd2024:reference/pages/057
splat_sources:
- title: SRD 5.2.1
  locator: pp. 57
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/oath_of_devotion_level_20_holy_nimbus
  parameters:
    action: bonus_action
    duration:
      minutes: 10
    voluntary_end_action: none
    uses:
      maximum: 1
      recover:
        event: rest.long_rest_completed
        amount: all
    restore_use:
      spell_slot_level: 5
      spell_slot_cost: 1
      action: none
    benefits:
      saving_throw_advantage:
        forced_by_creature_types:
        - fiend
        - undead
      aura_damage:
        aura: own_aura_of_protection
        trigger: enemy.turn_started_in_aura
        amount:
          op: add
          args:
          - stat: abilities.charisma.modifier
          - stat: proficiency_bonus
        type: radiant
      aura_light:
        brightness: bright
        sunlight: true
  implementation: dnd2024:class_features_oath_of_devotion_level_20_holy_nimbus
splat_contracts:
- id: dnd2024:class_features_oath_of_devotion_level_20_holy_nimbus
  inputs:
    actor:
      type: participant
      required: true
    trigger:
      required: false
      type: event
    targets:
      required: false
      type: participant_list
  outputs:
  - feature_state
  - resource_changes
  - damage
  events:
  - action.bonus
  - saving_throw.pending
  - turn.started
  capabilities:
  - auras
  - damage
  - resources
  - light
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 57
  engine_gaps: &id001
  - dnd2024:class_features_oath_of_devotion_level_20_holy_nimbus
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: 99f68a94f9fccbf6bd169159cf7012934c00be6645f7d49f1b56440bd51322ed
  parent_name: Oath of Devotion
  line_span:
    first: 4774
    count: 17
splat_clauses:
- key: source_rules
  text: '# Level 20: Holy Nimbus As a Bonus Action, you can imbue your Aura of Protection with holy power,
    granting the benefits below for 10 minutes or until you end them (no action required). Once you use
    this feature, you can’t use it again until you finish a Long Rest. You can also restore your use of
    it by expending a level 5 spell slot (no action required). Holy Ward. You have Advantage on any saving
    throw you are forced to make by a Fiend or an Undead. Radiant Damage. Whenever an enemy starts its
    turn in the aura, that creature takes Radiant damage equal to your Charisma modifier plus your Proficiency
    Bonus. Sunlight. The aura is filled with Bright Light that is sunlight.'
  sha256: 36bf6772af7e701f97383eb2c1ae8832236543c47cedbffece055589e924a29f
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 20: Holy Nimbus

<!-- SRD page 57 -->

As a Bonus Action, you can imbue your Aura of Pro-\
tection with holy power, granting the benefits be-\
low for 10 minutes or until you end them (no action\
required). Once you use this feature, you can’t use it\
again until you finish a Long Rest. You can also re-\
store your use of it by expending a level 5 spell slot\
(no action required).\
Holy Ward. You have Advantage on any saving\
throw you are forced to make by a Fiend or an\
Undead.\
Radiant Damage. Whenever an enemy starts its\
turn in the aura, that creature takes Radiant dam-\
age equal to your Charisma modifier plus your Pro-\
ficiency Bonus.\
Sunlight. The aura is filled with Bright Light that\
is sunlight.\

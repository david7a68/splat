---
splat_schema: 1
splat_id: 104ffb52-c4d5-50cf-8ef5-4bc801b8a714
splat_kind: spell
splat_key: dnd2024:spells/unseen_servant
splat_name: Unseen Servant
splat_origin:
  key: dnd2024:reference/spell/unseen_servant
splat_sources:
- title: SRD 5.2.1
  locator: pp. 171
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 1
  school: conjuration
  classes:
  - bard
  - warlock
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: 1 hour
  casting_time: Action or Ritual
  ritual: true
  range:
    feet: 60
  material:
    description: a bit of string and of wood
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/unseen_servant
  implementation: dnd2024:spell_unseen_servant
  parameters:
    force:
      size: medium
      invisible: true
      mindless: true
      shapeless: true
      initial_space: unoccupied_ground_in_range
      ac: 10
      hp: 1
      strength: 2
      can_attack: false
    end:
      hp_zero: true
      command_requires_move_beyond_caster_feet: 60
    command:
      cost: bonus_action
      maximum_per_caster_turn: 1
      mental: true
      move_maximum_feet: 15
      object_interaction: true
      task_scope: simple_human_tasks
      examples:
      - fetch
      - clean
      - mend
      - fold_clothes
      - light_fire
      - serve_food
      - pour_drink
      continue: best_ability_until_complete
      after_complete: wait_for_command
splat_contracts:
- id: dnd2024:spell_unseen_servant
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    spell_attack_modifier:
      required: false
      type: integer
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
  - servant
  - commands
  - object_interactions
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - attacks
  - movement
  - perception
  - inventory
  - time
  - relationships
splat_conversion:
  status: specified
  category: spell
  source_sha256: be9383f9c85c3f225340323a71b5a486ce7e351af68be33ded80331fa9f2daba
  source_path: builtin/srd-5.2.1/spell/unseen_servant.md
  pages:
  - 171
  engine_gaps: &id001
  - dnd2024:spell_unseen_servant
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_1.md
  specification_sha256: 17ad775ddb135fb0a22d19ba4d55255853ddf0e98ce5dd8186d88197523ccdae
splat_clauses:
- key: source_rules
  text: '# Unseen Servant Level 1 Conjuration (Bard, Warlock, Wizard) Casting Time: Action or Ritual Range:
    60 feet Components: V, S, M (a bit of string and of wood) Duration: 1 hour This spell creates an Invisible,
    mindless, shapeless, Medium force that performs simple tasks at your command until the spell ends.
    The servant springs into existence in an unoccupied space on the ground within range. It has AC 10,
    1 Hit Point, and a Strength of 2, and it can’t attack. If it drops to 0 Hit Points, the spell ends.
    Once on each of your turns as a Bonus Action, you can mentally command the servant to move up to 15
    feet and interact with an object. The servant can perform simple tasks that a human could do, such
    as fetching things, cleaning, mending, folding clothes, lighting fires, serving food, and pouring
    drinks. Once you give the command, the servant performs the task to the best of its ability until
    it completes the task, then waits for your next command. If you command the servant to perform a task
    that would move it more than 60 feet away from you, the spell ends.'
  sha256: 3c901073b63655a9f2124357433ee3b94d2215bff802485285a7a68879183e5c
  status: contract_specified
splat_requires_mechanics: *id001
---

# Unseen Servant

<!-- SRD page 171 -->

Level 1 Conjuration (Bard, Warlock, Wizard)\
Casting Time: Action or Ritual\
Range: 60 feet\
Components: V, S, M (a bit of string and of wood)\
Duration: 1 hour\
This spell creates an Invisible, mindless, shapeless,\
Medium force that performs simple tasks at your\
command until the spell ends. The servant springs\
into existence in an unoccupied space on the\
ground within range. It has AC 10, 1 Hit Point, and a\
Strength of 2, and it can’t attack. If it drops to 0 Hit\
Points, the spell ends.\
Once on each of your turns as a Bonus Action, you\
can mentally command the servant to move up to\
15 feet and interact with an object. The servant can\
perform simple tasks that a human could do, such as\
fetching things, cleaning, mending, folding clothes,\
lighting fires, serving food, and pouring drinks.\
Once you give the command, the servant performs\
the task to the best of its ability until it completes\
the task, then waits for your next command.\
If you command the servant to perform a task that\
would move it more than 60 feet away from you, the\
spell ends.\

Original layout: [[../../srd-5.2.1/pages/171|page 171]].

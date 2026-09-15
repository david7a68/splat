---
splat_schema: 1
splat_id: a51449cb-c485-55a6-86ba-d96a901af1a0
splat_kind: spell
splat_key: dnd2024:spells/animate_dead
splat_name: Animate Dead
splat_origin:
  key: dnd2024:reference/spell/animate_dead
splat_sources:
- title: SRD 5.2.1
  locator: pp. 108
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 3
  school: necromancy
  classes:
  - cleric
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: Instantaneous
  casting_time: 1 minute
  ritual: false
  range:
    feet: 10
  material:
    description: a drop of blood, a piece of flesh, and a pinch of bone dust
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/animate_dead
  implementation: dnd2024:spell_animate_dead
  parameters:
    target:
      kind: bones_or_humanoid_corpse
      count: 1
      visible_to_caster: false
      size:
      - small
      - medium
      unique_remains_per_creature: true
    animation:
      bones:
        key: dnd2024:creatures/skeleton
      corpse:
        key: dnd2024:creatures/zombie
    control:
      hours: 24
      command:
        action: bonus_action
        event: turn.current
        range_feet: 60
        telepathic: true
        targets: any_created_creatures_in_range
        same_command_all: true
        options:
        - next_turn_action_and_movement
        - general_task
        persists_until: task_completed
      uncommanded:
        action: dodge
        movement: avoid_harm_only
      expiry: stop_obeying
    reassert:
      target: own_animated_creatures
      before_control_expires: true
      base_count: 4
      control_hours: 24
      replaces_animation: true
    scaling:
      per_slot_above: 3
      additional_animated: 2
      additional_reasserted: 2
splat_contracts:
- id: dnd2024:spell_animate_dead
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
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
  - spawned_creatures
  - control_effect
  - commands
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - movement
  - summoning
  - time
  - relationships
splat_conversion:
  status: specified
  category: spell
  source_sha256: ecdcd4ab8fc1785220e7c7441f513210767e992abd59ed25cfadc0b973701bb0
  source_path: builtin/srd-5.2.1/spell/animate_dead.md
  pages:
  - 108
  engine_gaps: &id001
  - dnd2024:spell_animate_dead
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_1.md
  specification_sha256: 9c6f5820227713caaf4f346b5817a2aa5a84127c1db1edbd8387a24f78b2900d
splat_clauses:
- key: source_rules
  text: '# Animate Dead Level 3 Necromancy (Cleric, Wizard) Casting Time: 1 minute Range: 10 feet Components:
    V, S, M (a drop of blood, a piece of flesh, and a pinch of bone dust) Duration: Instantaneous Choose
    a pile of bones or a corpse of a Medium or Small Humanoid within range. The target becomes an Undead
    creature: a Skeleton if you chose bones or a Zombie if you chose a corpse (see “Monsters” for the
    stat blocks). On each of your turns, you can take a Bonus Action to mentally command any creature
    you made with this spell if the creature is within 60 feet of you (if you control multiple creatures,
    you can command any of them at the same time, issuing the same command to each one). You decide what
    action the creature will take and where it will move on its next turn, or you can issue a general
    command, such as to guard a chamber or corridor. If you issue no commands, the creature takes the
    Dodge action and moves only to avoid harm. Once given an order, the creature continues to follow it
    until its task is complete. The creature is under your control for 24 hours, after which it stops
    obeying any command you’ve given it. To maintain control of the creature for another 24 hours, you
    must cast this spell on the creature again before the current 24-hour period ends. This use of the
    spell reasserts your control over up to four creatures you have animated with this spell rather than
    animating a new creature. Using a Higher-Level Spell Slot. You animate or reassert control over two
    additional Undead creatures for each spell slot level above 3. Each of the creatures must come from
    a different corpse or pile of bones.'
  sha256: 23b4bf6d16b3ac9937fedcce31bdb5470e9acc99faee4be50beda9d79a93ac43
  status: contract_specified
splat_requires_mechanics: *id001
---

# Animate Dead

<!-- SRD page 108 -->

Level 3 Necromancy (Cleric, Wizard)\
Casting Time: 1 minute\
Range: 10 feet\
Components: V, S, M (a drop of blood, a piece of flesh,\
and a pinch of bone dust)\
Duration: Instantaneous\
Choose a pile of bones or a corpse of a Medium or\
Small Humanoid within range. The target becomes\
an Undead creature: a Skeleton if you chose bones\
or a Zombie if you chose a corpse (see “Monsters”\
for the stat blocks).\
On each of your turns, you can take a Bonus Ac-\
tion to mentally command any creature you made\
with this spell if the creature is within 60 feet of\
you (if you control multiple creatures, you can\
command any of them at the same time, issuing the\
same command to each one). You decide what action\
the creature will take and where it will move on\
its next turn, or you can issue a general command,\
such as to guard a chamber or corridor. If you issue\
no commands, the creature takes the Dodge action\
and moves only to avoid harm. Once given an order,\
the creature continues to follow it until its task is\
complete.\
The creature is under your control for 24 hours,\
after which it stops obeying any command you’ve\
given it. To maintain control of the creature for an-\
other 24 hours, you must cast this spell on the crea-\
ture again before the current 24-hour period ends.\
This use of the spell reasserts your control over up\
to four creatures you have animated with this spell\
rather than animating a new creature.\
Using a Higher-Level Spell Slot. You animate or\
reassert control over two additional Undead crea-\
tures for each spell slot level above 3. Each of the\
creatures must come from a different corpse or pile\
of bones.\

Original layout: [[../../srd-5.2.1/pages/108|page 108]].

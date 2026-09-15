---
splat_schema: 1
splat_id: 78708bf7-1722-50ee-b63e-52553566fb9a
splat_kind: spell
splat_key: dnd2024:spells/giant_insect
splat_name: Giant Insect
splat_origin:
  key: dnd2024:reference/spell/giant_insect
splat_sources:
- title: SRD 5.2.1
  locator: pp. 135, 136
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 4
  school: conjuration
  classes:
  - druid
  components:
  - verbal
  - somatic
  concentration: true
  duration: Concentration, up to 10 minutes
  casting_time: Action
  ritual: false
  range:
    feet: 60
splat_rule:
  mechanic: dnd2024:spells/giant_insect
  implementation: dnd2024:spell_giant_insect
  parameters:
    summon:
      form_choice:
      - centipede
      - spider
      - wasp
      space:
        unoccupied: true
        visible: true
        within_spell_range: true
      ally_to:
      - caster
      - caster_allies
      disappear:
      - hp_zero
      - spell_ends
    combat:
      initiative: caster_count
      turn: immediately_after_caster
      verbal_commands_cost: none
      without_command:
        action: dodge
        movement: avoid_danger
    statistics:
      size: large
      type: beast
      alignment: unaligned
      ac:
        op: add
        args:
        - 11
        - binding: cast_level
      hp:
        op: add
        args:
        - 30
        - op: multiply
          args:
          - 10
          - op: subtract
            args:
            - binding: cast_level
            - 4
      speed_feet: 40
      climb_speed_feet: 40
      fly:
        form: wasp
        speed_feet: 40
      abilities:
        strength: 17
        dexterity: 13
        constitution: 15
        intelligence: 4
        wisdom: 14
        charisma: 3
      save_modifiers:
        strength: 3
        dexterity: 1
        constitution: 2
        intelligence: -3
        wisdom: 2
        charisma: -4
      darkvision_feet: 60
      passive_perception: 12
      understood_languages: caster_known_languages
      challenge_rating: null
      xp: 0
      proficiency_bonus:
        binding: caster_proficiency_bonus
    traits:
      spider_climb:
        climb_difficult_surfaces: true
        ceilings: true
        requires_ability_check: false
    actions:
      multiattack:
        attack_count:
          op: floor
          args:
          - op: multiply
            args:
            - binding: cast_level
            - numerator: 1
              denominator: 2
        choices:
        - poison_jab
        - web_bolt_if_spider
      poison_jab:
        attack: melee
        bonus:
          binding: spell_attack_modifier
        reach_feet: 10
        damage:
        - dice:
            count: 1
            sides: 6
            add:
              op: add
              args:
              - 3
              - binding: cast_level
          type: piercing
        - dice:
            count: 1
            sides: 4
            add: 0
          type: poison
      web_bolt:
        form: spider
        attack: ranged
        bonus:
          binding: spell_attack_modifier
        range_feet: 60
        damage:
          dice:
            count: 1
            sides: 10
            add:
              op: add
              args:
              - 3
              - binding: cast_level
          type: bludgeoning
        hit_effect:
          speed: 0
          until: insect_next_turn_start
    bonus_actions:
      venomous_spew:
        form: centipede
        target:
          kind: creature
          count: 1
          visible_to_insect: true
          range_feet: 10
        save:
          ability: constitution
          dc:
            binding: save_dc
          failure:
            condition: poisoned
            until: insect_next_turn_start
          success:
            effect: none
splat_contracts:
- id: dnd2024:spell_giant_insect
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    locations:
      required: false
      type: location_list
    campaign:
      required: false
      type: campaign
    caster_proficiency_bonus:
      type: integer
      required: true
    save_dc:
      type: integer
      required: true
    spell_attack_modifier:
      type: integer
      required: true
    choices:
      required: false
      type: choice
  outputs:
  - effects
  - damage
  - attacks
  - participants
  events:
  - spell.cast_completed
  - effect.ended
  - movement.completed
  - turn.start
  - save.before
  - save.after
  - hp.changed
  capabilities:
  - spellcasting
  - effects
  - dice
  - damage
  - attacks
  - spatial_relationships
  - conditions
  - saving_throws
  - participant_lifecycle
splat_conversion:
  status: specified
  category: spell
  source_sha256: 629eb9c4f08ca3cbbba778f9b711fcb43b26ae438db6cdd1714bde4b2a4fe9fe
  source_path: builtin/srd-5.2.1/spell/giant_insect.md
  pages:
  - 135
  - 136
  engine_gaps: &id001
  - dnd2024:spell_giant_insect
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_3.md
  specification_sha256: aee69f231dab20a47352d6b61543fa426aa16ec91e3cff203f6f416a2944a8ea
splat_clauses:
- key: source_rules
  text: '# Giant Insect Level 4 Conjuration (Druid) Casting Time: Action Range: 60 feet Components: V,
    S Duration: Concentration, up to 10 minutes You summon a giant centipede, spider, or wasp (chosen
    when you cast the spell). It manifests in an unoccupied space you can see within range and uses the
    Giant Insect stat block. The form you choose determines certain details in its stat block. The creature
    disappears when it drops to 0 Hit Points or when the spell ends. The creature is an ally to you and
    your allies. In combat, the creature shares your Initiative count, but it takes its turn immediately
    after yours. It obeys your verbal commands (no action required by you). If you don’t issue any, it
    takes the Dodge action and uses its movement to avoid danger. Using a Higher-Level Spell Slot. Use
    the spell slot’s level for the spell’s level in the stat block. ### Giant Insect Large Beast, Unaligned
    AC 11 + the spell’s level HP 30 + 10 for each spell level above 4 Speed 40 ft., Climb 40 ft., Fly
    40 ft. (Wasp only) MOD SAVE MOD SAVE MOD SAVE Str 17 +3 +3 Dex 13 +1 +1 Con15 +2 +2 Int 4 -3 -3 Wis
    14 +2 +2 Cha 3 -4 -4 Senses Darkvision 60 ft.; Passive Perception 12 Languages Understands the languages
    you know CR None (XP 0; PB equals your Proficiency Bonus) ### Traits Spider Climb. The insect can
    climb difficult surfaces, including along ceilings, without needing to make an ability check. ###
    Actions Multiattack. The insect makes a number of attacks equal to half this spell’s level (round
    down). Poison Jab. Melee Attack Roll: Bonus equals your spell attack modifier, reach 10 ft. Hit: 1d6
    + 3 plus the spell’s level Piercing damage plus 1d4 Poison damage. Web Bolt (Spider Only). Ranged
    Attack Roll: Bonus equals your spell attack modifier, range 60 ft. Hit: 1d10 + 3 plus the spell’s
    level Bludgeoning damage, and the target’s Speed is reduced to 0 until the start of the insect’s next
    turn. ### Bonus Actions Venomous Spew (Centipede Only). Constitution Saving Throw: Your spell save
    DC, one creature the insect can see within 10 feet. Failure: The target has the Poisoned condition
    until the start of the insect’s next turn.'
  sha256: b1aedb6f36a741aa5a22db7f023ce2326b6a4783be73e49072f87a9ae79ff5a4
  status: contract_specified
splat_requires_mechanics: *id001
---

# Giant Insect

<!-- SRD page 135 -->

Level 4 Conjuration (Druid)\
Casting Time: Action\
Range: 60 feet\
Components: V, S\
Duration: Concentration, up to 10 minutes\
You summon a giant centipede, spider, or wasp (cho-\
sen when you cast the spell). It manifests in an un-\
occupied space you can see within range and uses\
the Giant Insect stat block. The form you choose\
determines certain details in its stat block. The\
creature disappears when it drops to 0 Hit Points or\
when the spell ends.\
The creature is an ally to you and your allies. In\
combat, the creature shares your Initiative count,\
but it takes its turn immediately after yours. It\
obeys your verbal commands (no action required by\
you). If you don’t issue any, it takes the Dodge action\
and uses its movement to avoid danger.\

<!-- SRD page 136 -->

Using a Higher-Level Spell Slot. Use the spell\
slot’s level for the spell’s level in the stat block.\

### Giant Insect

Large Beast, Unaligned\
AC 11 + the spell’s level\
HP 30 + 10 for each spell level above 4\
Speed 40 ft., Climb 40 ft., Fly 40 ft. (Wasp only)\
MOD SAVE MOD SAVE MOD SAVE\
Str 17 +3 +3 Dex 13 +1 +1 Con15 +2 +2\
Int 4 −3 −3 Wis 14 +2 +2 Cha 3 −4 −4\
Senses Darkvision 60 ft.; Passive Perception 12\
Languages Understands the languages you know\
CR None (XP 0; PB equals your Proficiency Bonus)\

### Traits

Spider Climb. The insect can climb difficult surfaces,\
including along ceilings, without needing to make an\
ability check.\

### Actions

Multiattack. The insect makes a number of attacks\
equal to half this spell’s level (round down).\
Poison Jab. Melee Attack Roll: Bonus equals your spell\
attack modifier, reach 10 ft. Hit: 1d6 + 3 plus the spell’s\
level Piercing damage plus 1d4 Poison damage.\
Web Bolt (Spider Only). Ranged Attack Roll: Bonus\
equals your spell attack modifier, range 60 ft. Hit: 1d10\
+ 3 plus the spell’s level Bludgeoning damage, and the\
target’s Speed is reduced to 0 until the start of the in-\
sect’s next turn.\

### Bonus Actions

Venomous Spew (Centipede Only). Constitution Saving\
Throw: Your spell save DC, one creature the insect can\
see within 10 feet. Failure: The target has the Poisoned\
condition until the start of the insect’s next turn.\

Original layout: [[../../srd-5.2.1/pages/135|page 135]], [[../../srd-5.2.1/pages/136|page 136]].

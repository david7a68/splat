---
splat_schema: 1
splat_id: ae18da0f-a055-5ce5-be8f-dffb5019d6c9
splat_kind: creature
splat_key: dnd2024:creatures/sphinx_of_lore
splat_name: Sphinx of Lore
splat_origin:
  key: dnd2024:reference/creature/sphinx_of_lore
splat_sources:
- title: SRD 5.2.1
  locator: pp. 327, 328
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: celestial
  alignment: Lawful Neutral
  movement_text: 40 ft., Fly 60 ft.
  skills_text: Arcana +12, History +12, Perception +8, Religion +12
  resistances_text: Necrotic, Radiant
  immunities_text: Psychic; Charmed, Frightened
  senses_text: Truesight 120 ft.; Passive Perception 18
  languages_text: Celestial, Common
  cr_text: 11 (XP 7,200, or 8,400 in lair; PB +4)
  hit_points_text: 170 (20d10 + 60)
splat_stats:
  strength:
    value: 18
  save_strength:
    value: 4
  dexterity:
    value: 15
  save_dexterity:
    value: 2
  constitution:
    value: 16
  save_constitution:
    value: 3
  intelligence:
    value: 18
  save_intelligence:
    value: 4
  wisdom:
    value: 18
  save_wisdom:
    value: 4
  charisma:
    value: 18
  save_charisma:
    value: 4
  armor_class:
    value: 17
  max_hp:
    value: 170
  initiative:
    value: 10
  speed:
    value: 40
    unit: ft
  fly_speed:
    value: 60
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/sphinx_of_lore
  implementation: dnd2024:creature_sphinx_of_lore
  parameters:
    features:
      legendary_resistance:
        economy: passive
        resource:
          capacity: 3
          capacity_in_lair: 4
          recovery: long_rest
          cost: 1
        trigger: saving_throw.failed
        optional: true
        op: replace_save_outcome
        outcome: success
      claw:
        economy: action
        op: attack
        attack_modes:
        - melee
        attack_bonus: 8
        targets:
          count: 1
        hit:
        - op: damage
          average: 14
          amount:
            dice:
              count: 3
              sides: 6
            add: 4
          damage_type: slashing
        reach_feet: 5
      multiattack:
        economy: action
        op: sequence
        steps:
        - op: use_feature
          feature: claw
          count: 3
      spellcasting:
        economy: action
        op: cast_spell_choice
        ability: intelligence
        spells:
        - spell:
            key: dnd2024:spells/detect_magic
          frequency: at_will
          cast_level: spell_minimum
        - spell:
            key: dnd2024:spells/identify
          frequency: at_will
          cast_level: spell_minimum
        - spell:
            key: dnd2024:spells/mage_hand
          frequency: at_will
          cast_level: spell_minimum
        - spell:
            key: dnd2024:spells/minor_illusion
          frequency: at_will
          cast_level: spell_minimum
        - spell:
            key: dnd2024:spells/prestidigitation
          frequency: at_will
          cast_level: spell_minimum
        - spell:
            key: dnd2024:spells/dispel_magic
          frequency: daily
          uses: 1
          resource_scope: per_spell
          recovery: long_rest
          cast_level: spell_minimum
        - spell:
            key: dnd2024:spells/legend_lore
          frequency: daily
          uses: 1
          resource_scope: per_spell
          recovery: long_rest
          cast_level: spell_minimum
        - spell:
            key: dnd2024:spells/locate_object
          frequency: daily
          uses: 1
          resource_scope: per_spell
          recovery: long_rest
          cast_level: spell_minimum
        - spell:
            key: dnd2024:spells/plane_shift
          frequency: daily
          uses: 1
          resource_scope: per_spell
          recovery: long_rest
          cast_level: spell_minimum
        - spell:
            key: dnd2024:spells/remove_curse
          frequency: daily
          uses: 1
          resource_scope: per_spell
          recovery: long_rest
          cast_level: spell_minimum
        - spell:
            key: dnd2024:spells/tongues
          frequency: daily
          uses: 1
          resource_scope: per_spell
          recovery: long_rest
          cast_level: spell_minimum
        components_waived:
        - material
        save_dc: 16
      inscrutable:
        economy: passive
        remote_magical_observation:
          requires_permission: true
        magical_thought_detection:
          requires_permission: true
        insight_disadvantage:
          purpose:
          - ascertain_intentions
          - ascertain_sincerity
      mind_rending_roar:
        economy: action
        resource:
          capacity: 1
          initial: 1
          recharge:
            event: turn.start
            participant: self
            roll:
              dice:
                count: 1
                sides: 6
              add: 0
            succeeds_on:
            - 5
            - 6
          cost: 1
          recovery_also:
          - short_rest
          - long_rest
        op: save
        ability: wisdom
        dc: 16
        targets:
          kind: creature
          count: all
          area:
            shape: emanation
            size_feet: 300
            origin: self
          relation: enemy
        failure:
        - op: damage
          average: 35
          amount:
            dice:
              count: 10
              sides: 6
            add: 0
          damage_type: psychic
        - op: condition
          condition: incapacitated
          ends:
            event: turn.start
            participant: self
            occurrence: next
        success: []
      weight_of_years:
        economy: legendary_action
        cooldown_ends:
          event: turn.start
          participant: self
          occurrence: next
        op: save
        ability: constitution
        dc: 16
        targets:
          kind: creature
          count: 1
          range_feet: 120
          visible_to: self
        failure:
        - op: exhaustion
          levels: 1
        - op: apparent_age_increase
          years:
            dice:
              count: 3
              sides: 10
            add: 0
          ends:
            target_exhaustion_levels: 0
        success: []
      arcane_prowl:
        economy: legendary_action
        op: sequence
        steps:
        - op: optional
          effect:
            op: teleport
            range_feet: 30
            destination:
              unoccupied: true
              visible_to: self
        - op: use_feature
          feature: claw
          count: 1
    skill_totals:
      arcana: 12
      history: 12
      perception: 8
      religion: 12
    defenses:
      resistances:
        damage_types:
        - necrotic
        - radiant
      immunities:
        damage_types:
        - psychic
        conditions:
        - charmed
        - frightened
    senses:
      passive_perception: 18
      special:
      - sense: truesight
        range_feet: 120
    communication:
      speaks:
      - celestial
      - common
      understands:
      - celestial
      - common
    legendary_actions:
      uses: 3
      uses_in_lair: 4
      cost_per_action: 1
      trigger: another_creature.turn.end
      recovery:
        event: turn.start
        participant: self
        amount: all
      maximum_per_trigger: 1
      requires:
        self_incapacitated: false
        self_can_take_actions: true
    stat_block_conventions:
      attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
      save_dc: supplied_total
      damage_mode_choice:
      - fixed_average
      - rolled_dice
      condition_source: self
      multiattack_child_economy_cost: 0
      multiattack_child_resource_costs_retained: true
      spellcasting:
        default_level: spell_minimum
        spell_slots_expended: false
        required_materials: available
        casting_time_at_least_one_minute:
          uses_normal_casting_duration: true
          magic_action_each_turn: true
          concentration_required: true
      geometry_assumptions:
        chooser: gm
        supplies:
        - range
        - visibility
        - line_of_sight
        - area_membership
        - reachable_locations
        - unoccupied_locations
        - terrain
        - plane
        - form_appearance
splat_contracts:
- id: dnd2024:creature_sphinx_of_lore
  inputs:
    self:
      type: participant
      required: true
    targets:
      required: false
      type: participant_list
    feature:
      type: choice
      required: true
    event:
      required: false
      type: event
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
  - resolved_feature
  - effects
  - resource_changes
  events:
  - another_creature.turn.end
  - saving_throw.failed
  - turn.start
  capabilities:
  - action_economy
  - creature_rules
  - curses
  - dice
  - effects
  - legendary_actions
  - movement
  - resources
  - spellcasting
  - targeting
splat_conversion:
  status: specified
  category: creature
  source_sha256: 0466a4d544dba8f0c2e6d441bfa50f5f516a01685666b7ad4dc51a757ee3d8fe
  source_path: builtin/srd-5.2.1/creature/sphinx_of_lore.md
  pages:
  - 327
  - 328
  engine_gaps: &id001
  - dnd2024:creature_sphinx_of_lore
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: d91eeed21e39389219ad4af759b7fd29ec0340a9b7ab39bd285396ce53a681cc
splat_clauses:
- key: source_rules
  text: '# Sphinx of Lore Large Celestial, Lawful Neutral AC 17 Initiative +10 (20) HP 170 (20d10 + 60)
    Speed 40 ft., Fly 60 ft. MOD SAVE MOD SAVE MOD SAVE Str 18 +4 +4 Dex 15 +2 +2 Con 16 +3 +3 Int 18
    +4 +4 Wis 18 +4 +4 Cha 18 +4 +4 Skills Arcana +12, History +12, Perception +8, Religion +12 Resistances
    Necrotic, Radiant Immunities Psychic; Charmed, Frightened Senses Truesight 120 ft.; Passive Perception
    18 Languages Celestial, Common CR 11 (XP 7,200, or 8,400 in lair; PB +4) ### Traits Inscrutable. No
    magic can observe the sphinx remotely or detect its thoughts without its permission. Wisdom (Insight)
    checks made to ascertain its intentions or sincerity are made with Disadvantage. Legendary Resistance
    (3/Day, or 4/Day in Lair). If the sphinx fails a saving throw, it can choose to succeed instead. ###
    Actions Multiattack. The sphinx makes three Claw attacks. Claw. Melee Attack Roll: +8, reach 5 ft.
    Hit: 14 (3d6 + 4) Slashing damage. Mind-Rending Roar (Recharge 5–6). Wisdom Saving Throw: DC 16, each
    enemy in a 300-foot Emanation originating from the sphinx. Failure: 35 (10d6) Psychic damage, and
    the target has the Incapacitated condition until the start of the sphinx’s next turn. Spellcasting.
    The sphinx casts one of the following spells, requiring no Material components and using Intelligence
    as the spellcasting ability (spell save DC 16): At Will: Detect Magic, Identify, Mage Hand, Minor
    Illusion, Prestidigitation 1/Day Each: Dispel Magic, Legend Lore, Locate Object, Plane Shift, Remove
    Curse, Tongues ### Legendary Actions Legendary Action Uses: 3 (4 in Lair). Immediately after another
    creature’s turn, the sphinx can expend a use to take one of the following actions. The sphinx regains
    all expended uses at the start of each of its turns. Arcane Prowl. The sphinx can teleport up to 30
    feet to an unoccupied space it can see, and it makes one Claw attack. Weight of Years. Constitution
    Saving Throw: DC 16, one creature the sphinx can see within 120 feet. Failure: The target gains 1
    Exhaustion level. While the target has any Exhaustion levels, it appears 3d10 years older. Failure
    or Success: The sphinx can’t take this action again until the start of its next turn.'
  sha256: 5ada7ecc5dde7566d7f0d1f6788b4c066a30c969f09e45823d8411d9fe95137d
  status: contract_specified
splat_requires_mechanics: *id001
---

# Sphinx of Lore

<!-- SRD page 327 -->

Large Celestial, Lawful Neutral\
AC 17 Initiative +10 (20)\
HP 170 (20d10 + 60)\
Speed 40 ft., Fly 60 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 18 +4 +4 Dex 15 +2 +2 Con 16 +3 +3\
Int 18 +4 +4 Wis 18 +4 +4 Cha 18 +4 +4\
Skills Arcana +12, History +12, Perception +8,\
Religion +12\
Resistances Necrotic, Radiant\
Immunities Psychic; Charmed, Frightened\
Senses Truesight 120 ft.; Passive Perception 18\
Languages Celestial, Common\
CR 11 (XP 7,200, or 8,400 in lair; PB +4)\

### Traits

Inscrutable. No magic can observe the sphinx remotely\
or detect its thoughts without its permission. Wisdom\
(Insight) checks made to ascertain its intentions or sin-\
cerity are made with Disadvantage.\
Legendary Resistance (3/Day, or 4/Day in Lair). If\
the sphinx fails a saving throw, it can choose to suc-\
ceed instead.\

<!-- SRD page 328 -->


### Actions

Multiattack. The sphinx makes three Claw attacks.\
Claw. Melee Attack Roll: +8, reach 5 ft. Hit: 14 (3d6\
+ 4) Slashing damage.\
Mind-Rending Roar (Recharge 5–6). Wisdom Saving\
Throw: DC 16, each enemy in a 300-foot Emanation\
originating from the sphinx. Failure: 35 (10d6) Psychic\
damage, and the target has the Incapacitated condition\
until the start of the sphinx’s next turn.\
Spellcasting. The sphinx casts one of the following\
spells, requiring no Material components and using In-\
telligence as the spellcasting ability (spell save DC 16):\
At Will: Detect Magic, Identify, Mage Hand, Minor Illu-\
sion, Prestidigitation\
1/Day Each: Dispel Magic, Legend Lore, Locate Object,\
Plane Shift, Remove Curse, Tongues\

### Legendary Actions

Legendary Action Uses: 3 (4 in Lair). Immediately after\
another creature’s turn, the sphinx can expend a use to\
take one of the following actions. The sphinx regains all\
expended uses at the start of each of its turns.\
Arcane Prowl. The sphinx can teleport up to 30 feet\
to an unoccupied space it can see, and it makes one\
Claw attack.\
Weight of Years. Constitution Saving Throw: DC 16,\
one creature the sphinx can see within 120 feet. Failure:\
The target gains 1 Exhaustion level. While the target has\
any Exhaustion levels, it appears 3d10 years older. Fail-\
ure or Success: The sphinx can’t take this action again\
until the start of its next turn.\

Original layout: [[../../srd-5.2.1/pages/327|page 327]], [[../../srd-5.2.1/pages/328|page 328]].

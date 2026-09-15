---
splat_schema: 1
splat_id: 7e17aeb6-7f20-5e76-96a2-9d5f6c0e9fd1
splat_kind: spell
splat_key: dnd2024:spells/wish
splat_name: Wish
splat_origin:
  key: dnd2024:reference/spell/wish
splat_sources:
- title: SRD 5.2.1
  locator: pp. 175
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 9
  school: conjuration
  classes:
  - sorcerer
  - wizard
  components:
  - verbal
  concentration: false
  duration: Instantaneous
  casting_time: Action
  ritual: false
  range:
    mode: self
splat_rule:
  mechanic: dnd2024:spells/wish
  implementation: dnd2024:spell_wish
  parameters:
    choice:
      chooser: caster
      options:
      - duplicate_spell
      - object_creation
      - instant_health
      - resistance
      - spell_immunity
      - sudden_learning
      - roll_redo
      - reshape_reality
    effects:
      duplicate_spell:
        maximum_spell_level: 8
        other_spell_only: true
        casting_requirements_waived: all_including_costly_components
        spell_takes_effect: true
        stress: false
      object_creation:
        count: 1
        maximum_gp_value: 25000
        magic_item: false
        maximum_each_dimension_feet: 300
        destination: visible_unoccupied_ground
      instant_health:
        caster: true
        additional_visible_creatures_maximum: 20
        heal: full_hp
        end_all_effects:
        - exhaustion
        - charmed
        - petrified
        - curses
        - cursed_item_attunement
        - ability_score_reductions
        - hp_maximum_reductions
      resistance:
        maximum_visible_creatures: 10
        damage_type_choice_count: 1
        duration: permanent
      spell_immunity:
        maximum_visible_creatures: 10
        single_choice:
        - spell
        - magical_effect
        duration_hours: 8
      sudden_learning:
        replace_own_feat_count: 1
        new_feat_eligibility_required: true
        old_feat_cannot_be_prerequisite_for:
        - other_feat
        - feature
        remove_old_benefits: true
        grant_new_benefits: true
      roll_redo:
        roll_count: 1
        lookback: last_round_including_caster_last_turn
        reroll: true
        roll_mode_choice:
        - normal
        - advantage
        - disadvantage
        result_choice:
        - original
        - reroll
        reshape_prior_consequences: true
      reshape_reality:
        request: caster_precise_statement
        adjudicator: gm
        resolution_choice:
        - success
        - partial_success
        - failure
        - unforeseen_consequence
        risk_increases_with_wish_magnitude: true
        possible_consequences:
        - time_travel
        - transport_to_item_owner
        - powerful_foes_for_community_region_world_effects
        - divine_servants_intervention_for_god_effects
        undo_multiverse: automatic_failure
    stress:
      applies_to: all_modes_except_duplicate_spell
      spell_cast_damage:
        until: caster.long_rest.completed
        event: spell.cast_completed
        amount:
          dice:
            count:
              binding: cast_spell_level
            sides: 10
            add: 0
        type: necrotic
        reducible: false
        preventable: false
        cantrip_damage: 0
      strength:
        set: 3
        duration_days:
          dice:
            count: 2
            sides: 4
            add: 0
        rest_light_activity_day_recovery_reduction_days: 2
      permanent_loss_of_wish_percent: 33
splat_contracts:
- id: dnd2024:spell_wish
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
    cast_spell_level:
      type: integer
      required: true
  outputs:
  - duplicated_spell
  - chosen_effect
  - revised_roll
  - feat_replacement
  - reality_resolution
  - stress_effect
  - wish_casting_loss
  events:
  - spell.cast_completed
  - rest.completed
  capabilities:
  - spellcasting
  - effects
  - damage
  - healing
  - conditions
  - movement
  - perception
  - inventory
  - summoning
  - choices
  - time
  - relationships
splat_conversion:
  status: specified
  category: spell
  source_sha256: 7b25c16f59313843562eec92395aceb8e8e1704cdc03b9f278b9868039f450b6
  source_path: builtin/srd-5.2.1/spell/wish.md
  pages:
  - 175
  engine_gaps: &id001
  - dnd2024:spell_wish
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_1.md
  specification_sha256: 24f8b7c9ed6c04b0239059d9d2bca141ac658e7f02cdf9f9d299481ffff07bb9
splat_clauses:
- key: source_rules
  text: '# Wish Level 9 Conjuration (Sorcerer, Wizard) Casting Time: Action Range: Self Components: V
    Duration: Instantaneous Wish is the mightiest spell a mortal can cast. By simply speaking aloud, you
    can alter reality itself. The basic use of this spell is to duplicate any other spell of level 8 or
    lower. If you use it this way, you don’t need to meet any requirements to cast that spell, including
    costly components. The spell simply takes effect. Alternatively, you can create one of the following
    effects of your choice: Object Creation. You create one object of up to 25,000 GP in value that isn’t
    a magic item. The object can be no more than 300 feet in any dimension, and it appears in an unoccupied
    space that you can see on the ground. Instant Health. You allow yourself and up to twenty creatures
    that you can see to regain all Hit Points, and you end all effects on them listed in the Greater Restoration
    spell. Resistance. You grant up to ten creatures that you can see Resistance to one damage type that
    you choose. This Resistance is permanent. Spell Immunity. You grant up to ten creatures you can see
    immunity to a single spell or other magical effect for 8 hours. Sudden Learning. You replace one of
    your feats with another feat for which you are eligible. You lose all the benefits of the old feat
    and gain the benefits of the new one. You can’t replace a feat that is a prerequisite for any of your
    other feats or features. Roll Redo. You undo a single recent event by forcing a reroll of any die
    roll made within the last round (including your last turn). Reality reshapes itself to accommodate
    the new result. For example, a Wish spell could undo an ally’s failed saving throw or a foe’s Critical
    Hit. You can force the reroll to be made with Advantage or Disadvantage, and you choose whether to
    use the reroll or the original roll. Reshape Reality. You may wish for something not included in any
    of the other effects. To do so, state your wish to the GM as precisely as possible. The GM has great
    latitude in ruling what occurs in such an instance; the greater the wish, the greater the likelihood
    that something goes wrong. This spell might simply fail, the effect you desire might be achieved only
    in part, or you might suffer an unforeseen consequence as a result of how you worded the wish. For
    example, wishing that a villain were dead might propel you forward in time to a period when that villain
    is no longer alive, effectively removing you from the game. Similarly, wishing for a Legendary magic
    item or an Artifact might instantly transport you to the presence of the item’s current owner. If
    your wish is granted and its effects have consequences for a whole community, region, or world, you
    are likely to attract powerful foes. If your wish would affect a god, the god’s divine servants might
    instantly intervene to prevent it or to encourage you to craft the wish in a particular way. If your
    wish would undo the multiverse itself, your wish fails. The stress of casting Wish to produce any
    effect other than duplicating another spell weakens you. After enduring that stress, each time you
    cast a spell until you finish a Long Rest, you take 1d10 Necrotic damage per level of that spell.
    This damage can’t be reduced or prevented in any way. In addition, your Strength score becomes 3 for
    2d4 days. For each of those days that you spend resting and doing nothing more than light activity,
    your remaining recovery time decreases by 2 days. Finally, there is a 33 percent chance that you are
    unable to cast Wish ever again if you suffer this stress.'
  sha256: 5fbdae3f5178de53efe8d8842fd5f38f39028b6a63a706b6aa5e1bff2e6b6ab2
  status: contract_specified
splat_requires_mechanics: *id001
---

# Wish

<!-- SRD page 175 -->

Level 9 Conjuration (Sorcerer, Wizard)\
Casting Time: Action\
Range: Self\
Components: V\
Duration: Instantaneous\
Wish is the mightiest spell a mortal can cast. By\
simply speaking aloud, you can alter reality itself.\
The basic use of this spell is to duplicate any other\
spell of level 8 or lower. If you use it this way, you\
don’t need to meet any requirements to cast that\
spell, including costly components. The spell simply\
takes effect.\
Alternatively, you can create one of the following\
effects of your choice:\
Object Creation. You create one object of up to\
25,000 GP in value that isn’t a magic item. The\
object can be no more than 300 feet in any dimen-\
sion, and it appears in an unoccupied space that\
you can see on the ground.\
Instant Health. You allow yourself and up to\
twenty creatures that you can see to regain all Hit\
Points, and you end all effects on them listed in\
the Greater Restoration spell.\
Resistance. You grant up to ten creatures that you\
can see Resistance to one damage type that you\
choose. This Resistance is permanent.\
Spell Immunity. You grant up to ten creatures you\
can see immunity to a single spell or other magi-\
cal effect for 8 hours.\
Sudden Learning. You replace one of your feats with\
another feat for which you are eligible. You lose all\
the benefits of the old feat and gain the benefits of\
the new one. You can’t replace a feat that is a pre-\
requisite for any of your other feats or features.\
Roll Redo. You undo a single recent event by forcing\
a reroll of any die roll made within the last round\
(including your last turn). Reality reshapes itself to\
accommodate the new result. For example, a Wish\
spell could undo an ally’s failed saving throw or a\
foe’s Critical Hit. You can force the reroll to be made\
with Advantage or Disadvantage, and you choose\
whether to use the reroll or the original roll.\
Reshape Reality. You may wish for something not\
included in any of the other effects. To do so, state\
your wish to the GM as precisely as possible. The\
GM has great latitude in ruling what occurs in\
such an instance; the greater the wish, the greater\
the likelihood that something goes wrong. This\
spell might simply fail, the effect you desire might\
be achieved only in part, or you might suffer an\
unforeseen consequence as a result of how you\
worded the wish. For example, wishing that a\
villain were dead might propel you forward in\
time to a period when that villain is no longer\
alive, effectively removing you from the game.\
Similarly, wishing for a Legendary magic item or\
an Artifact might instantly transport you to the\
presence of the item’s current owner. If your wish\
is granted and its effects have consequences for a\
whole community, region, or world, you are likely\
to attract powerful foes. If your wish would affect\
a god, the god’s divine servants might instantly in-\
tervene to prevent it or to encourage you to craft\
the wish in a particular way. If your wish would\
undo the multiverse itself, your wish fails.\
The stress of casting Wish to produce any effect\
other than duplicating another spell weakens you.\
After enduring that stress, each time you cast a\
spell until you finish a Long Rest, you take 1d10\
Necrotic damage per level of that spell. This damage\
can’t be reduced or prevented in any way. In addi-\
tion, your Strength score becomes 3 for 2d4 days.\
For each of those days that you spend resting and\
doing nothing more than light activity, your remain-\
ing recovery time decreases by 2 days. Finally, there\
is a 33 percent chance that you are unable to cast\
Wish ever again if you suffer this stress.\

Original layout: [[../../srd-5.2.1/pages/175|page 175]].

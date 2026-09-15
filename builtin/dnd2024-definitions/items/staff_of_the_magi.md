---
splat_schema: 1
splat_id: df0a375f-b01d-5b65-bd59-107144faadb7
splat_kind: item
splat_key: dnd2024:items/staff_of_the_magi
splat_name: Staff of the Magi
splat_origin:
  key: dnd2024:reference/magic_item/staff_of_the_magi
splat_sources:
- title: SRD 5.2.1
  locator: pp. 246, 247
  url: https://www.dndbeyond.com/srd
splat_item:
  category: staff
  rarity: legendary
  equipment:
    attunement: required
    attunement_requirement_text: a Sorcerer, Warlock, or Wizard
splat_rule:
  mechanic: dnd2024:items/staff_of_the_magi
  implementation: dnd2024:item_staff_of_the_magi
  parameters:
    weapon:
      key: dnd2024:equipment/quarterstaff
    attack_roll_bonus: 2
    damage_roll_bonus: 2
    scope: attacks_with_source_item
    while_held:
      spell_attack_bonus: 2
      save_advantage_against:
      - spells
    charges:
      maximum: 50
      initial: 50
      recovery:
        event: dawn
        amount:
          expression: 4d6+2
          scope: use
        cap: maximum
    spell_grants_require: held
    spell_save_dc: user_spell_save_dc
    spell_grants:
    - spell:
        key: dnd2024:spells/arcane_lock
      charge_cost: 0
      casting_time: spell_default
      method: item
      slot_cost: 0
      components: []
      level: lowest_possible
      range: spell_default
      duration: spell_default
      concentration: spell_default
    - spell:
        key: dnd2024:spells/conjure_elemental
      charge_cost: 7
      casting_time: spell_default
      method: item
      slot_cost: 0
      components: []
      level: lowest_possible
      range: spell_default
      duration: spell_default
      concentration: spell_default
    - spell:
        key: dnd2024:spells/detect_magic
      charge_cost: 0
      casting_time: spell_default
      method: item
      slot_cost: 0
      components: []
      level: lowest_possible
      range: spell_default
      duration: spell_default
      concentration: spell_default
    - spell:
        key: dnd2024:spells/dispel_magic
      charge_cost: 3
      casting_time: spell_default
      method: item
      slot_cost: 0
      components: []
      level: lowest_possible
      range: spell_default
      duration: spell_default
      concentration: spell_default
    - spell:
        key: dnd2024:spells/enlarge_reduce
      charge_cost: 0
      casting_time: spell_default
      method: item
      slot_cost: 0
      components: []
      level: lowest_possible
      range: spell_default
      duration: spell_default
      concentration: spell_default
    - spell:
        key: dnd2024:spells/fireball
      charge_cost: 7
      casting_time: spell_default
      method: item
      slot_cost: 0
      components: []
      level: 7
      range: spell_default
      duration: spell_default
      concentration: spell_default
    - spell:
        key: dnd2024:spells/flaming_sphere
      charge_cost: 2
      casting_time: spell_default
      method: item
      slot_cost: 0
      components: []
      level: lowest_possible
      range: spell_default
      duration: spell_default
      concentration: spell_default
    - spell:
        key: dnd2024:spells/ice_storm
      charge_cost: 4
      casting_time: spell_default
      method: item
      slot_cost: 0
      components: []
      level: lowest_possible
      range: spell_default
      duration: spell_default
      concentration: spell_default
    - spell:
        key: dnd2024:spells/invisibility
      charge_cost: 2
      casting_time: spell_default
      method: item
      slot_cost: 0
      components: []
      level: lowest_possible
      range: spell_default
      duration: spell_default
      concentration: spell_default
    - spell:
        key: dnd2024:spells/knock
      charge_cost: 2
      casting_time: spell_default
      method: item
      slot_cost: 0
      components: []
      level: lowest_possible
      range: spell_default
      duration: spell_default
      concentration: spell_default
    - spell:
        key: dnd2024:spells/light
      charge_cost: 0
      casting_time: spell_default
      method: item
      slot_cost: 0
      components: []
      level: lowest_possible
      range: spell_default
      duration: spell_default
      concentration: spell_default
    - spell:
        key: dnd2024:spells/lightning_bolt
      charge_cost: 7
      casting_time: spell_default
      method: item
      slot_cost: 0
      components: []
      level: 7
      range: spell_default
      duration: spell_default
      concentration: spell_default
    - spell:
        key: dnd2024:spells/mage_hand
      charge_cost: 0
      casting_time: spell_default
      method: item
      slot_cost: 0
      components: []
      level: lowest_possible
      range: spell_default
      duration: spell_default
      concentration: spell_default
    - spell:
        key: dnd2024:spells/passwall
      charge_cost: 5
      casting_time: spell_default
      method: item
      slot_cost: 0
      components: []
      level: lowest_possible
      range: spell_default
      duration: spell_default
      concentration: spell_default
    - spell:
        key: dnd2024:spells/plane_shift
      charge_cost: 7
      casting_time: spell_default
      method: item
      slot_cost: 0
      components: []
      level: lowest_possible
      range: spell_default
      duration: spell_default
      concentration: spell_default
    - spell:
        key: dnd2024:spells/protection_from_evil_and_good
      charge_cost: 0
      casting_time: spell_default
      method: item
      slot_cost: 0
      components: []
      level: lowest_possible
      range: spell_default
      duration: spell_default
      concentration: spell_default
    - spell:
        key: dnd2024:spells/telekinesis
      charge_cost: 5
      casting_time: spell_default
      method: item
      slot_cost: 0
      components: []
      level: lowest_possible
      range: spell_default
      duration: spell_default
      concentration: spell_default
    - spell:
        key: dnd2024:spells/wall_of_fire
      charge_cost: 4
      casting_time: spell_default
      method: item
      slot_cost: 0
      components: []
      level: lowest_possible
      range: spell_default
      duration: spell_default
      concentration: spell_default
    - spell:
        key: dnd2024:spells/web
      charge_cost: 2
      casting_time: spell_default
      method: item
      slot_cost: 0
      components: []
      level: lowest_possible
      range: spell_default
      duration: spell_default
      concentration: spell_default
    absorption:
      equipment_state: held
      action: reaction
      trigger: another_creature_casts_spell_targeting_only_user
      cancel_effect: true
      charge_gain: spell_level
      if_result_above_50:
        effect: retributive_strike
        trigger: automatic
        charge_basis: total_after_absorption
    last_charge:
      roll:
        expression: 1d20
        scope: use
      'on': 20
      restore_charges:
        expression: 1d12+1
        scope: use
    retributive_strike:
      action: magic
      method:
      - break_over_knee
      - break_against_solid_surface
      destroy_item: true
      area:
        shape: emanation
        radius_ft: 30
        origin: staff
      user:
        escape_chance_percent: 50
        on_escape:
          transport: random_plane
          damage: 0
        on_no_escape:
          damage:
            type: force
            amount:
              op: multiply
              args:
              - 16
              - binding: charges_before_destruction
      other_creatures:
        save:
          ability: dexterity
          dc: 17
          failure:
            damage:
              type: force
              amount:
                op: multiply
                args:
                - 6
                - binding: charges_before_destruction
          success:
            damage_scale:
              numerator: 1
              denominator: 2
    attunement:
      required: true
      eligible_classes:
      - sorcerer
      - warlock
      - wizard
    common_rules:
    - key: dnd2024:rules/magic_item_categories_staffs
    - key: dnd2024:rules/activating_a_magic_item_spells_cast_from_items
splat_contracts:
- id: dnd2024:item_staff_of_the_magi
  inputs:
    actor:
      type: participant
      required: true
    source_item:
      type: item_instance
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
    choice:
      required: false
      type: choice
    event:
      required: false
      type: event
    campaign:
      required: false
      type: campaign
    charges_before_destruction:
      required: false
      type: integer
  outputs:
  - item_state
  - effects
  - resolutions
  events:
  - spell.cast_pending
  - action.magic_pending
  - time.dawn
  - equipment.changed
  - attunement.changed
  capabilities:
  - inventory
  - effects
  - spellcasting
  - resources_and_time
  - roll_resolution
  - spatial_state
  - participant_state
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: b00be47f33e0f4fc1d1c478661346331646b33e77b8d030a1d15b137b4f4af84
  source_path: builtin/srd-5.2.1/magic_item/staff_of_the_magi.md
  pages:
  - 246
  - 247
  engine_gaps: &id001
  - dnd2024:item_staff_of_the_magi
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_2.md
  specification_sha256: e94349c65c5ee4a6c428c5eeff177a52c414b5f1afc054e518d77abe2686ec50
splat_clauses:
- key: source_rules
  text: '# Staff of the Magi Staff, Legendary (Requires Attunement by a Sorcerer, Warlock, or Wizard)
    This staff has 50 charges and can be wielded as a magic Quarterstaff that grants a +2 bonus to attack
    rolls and damage rolls made with it. While you hold it, you gain a +2 bonus to spell attack rolls.
    Spell Absorption. While holding the staff, you have Advantage on saving throws against spells. In
    addition, you can take a Reaction when another creature casts a spell that targets only you. If you
    do, the staff absorbs the magic of the spell, canceling its effect and gaining a number of charges
    equal to the absorbed spell’s level. However, if doing so brings the staff’s total number of charges
    above 50, the staff explodes as if you activated its Retributive Strike (see below). Spells. While
    holding the staff, you can cast one of the spells on the following table from it, using your spell
    save DC. The table indicates how many charges you must expend to cast the spell. Charge Spell Cost
    Arcane Lock 0 Conjure Elemental 7 Detect Magic 0 Dispel Magic 3 Enlarge/Reduce 0 Fireball (level 7
    version) 7 Flaming Sphere 2 Ice Storm 4 Invisibility 2 Knock 2 Light 0 Lightning Bolt (level 7 version)
    7 Charge Spell Cost Mage Hand 0 Passwall 5 Plane Shift 7 Protection from Evil and Good 0 Telekinesis
    5 Wall of Fire 4 Web 2 Regaining Charges. The staff regains 4d6 + 2 expended charges daily at dawn.
    If you expend the last charge, roll 1d20. On a 20, the staff regains 1d12 + 1 charges. Retributive
    Strike. You can take a Magic action to break the staff over your knee or against a solid surface.
    The staff is destroyed and releases its magic in an explosion that fills a 30-foot Emanation originating
    from itself. You have a 50 percent chance to instantly travel to a random plane of existence, avoiding
    the explosion. If you fail to avoid the effect, you take Force damage equal to 16 times the number
    of charges in the staff. Each other creature in the area makes a DC 17 Dexterity saving throw. On
    a failed save, a creature takes Force damage equal to 6 times the number of charges in the staff.
    On a successful save, a creature takes half as much damage.'
  sha256: 65cabf7684a75d12f8c754b6416918cc5ffcc0621fbe258bc29f699db1d0d29c
  status: contract_specified
splat_requires_mechanics: *id001
---

# Staff of the Magi

<!-- SRD page 246 -->

Staff, Legendary (Requires Attunement by a Sorcerer,\
Warlock, or Wizard)\
This staff has 50 charges and can be wielded as a\
magic Quarterstaff that grants a +2 bonus to attack\
rolls and damage rolls made with it. While you hold\
it, you gain a +2 bonus to spell attack rolls.\
Spell Absorption. While holding the staff, you\
have Advantage on saving throws against spells.\
In addition, you can take a Reaction when another\
creature casts a spell that targets only you. If you\
do, the staff absorbs the magic of the spell, cancel-\
ing its effect and gaining a number of charges equal\
to the absorbed spell’s level. However, if doing so\
brings the staff’s total number of charges above 50,\
the staff explodes as if you activated its Retributive\
Strike (see below).\
Spells. While holding the staff, you can cast one of\
the spells on the following table from it, using your\
spell save DC. The table indicates how many charges\
you must expend to cast the spell.\
Charge\
Spell Cost\
Arcane Lock 0\
Conjure Elemental 7\
Detect Magic 0\
Dispel Magic 3\
Enlarge/Reduce 0\
Fireball (level 7 version) 7\
Flaming Sphere 2\
Ice Storm 4\
Invisibility 2\
Knock 2\
Light 0\
Lightning Bolt (level 7 version) 7\

<!-- SRD page 247 -->

Charge\
Spell Cost\
Mage Hand 0\
Passwall 5\
Plane Shift 7\
Protection from Evil and Good 0\
Telekinesis 5\
Wall of Fire 4\
Web 2\
Regaining Charges. The staff regains 4d6 + 2 ex-\
pended charges daily at dawn. If you expend the last\
charge, roll 1d20. On a 20, the staff regains 1d12 + 1\
charges.\
Retributive Strike. You can take a Magic action to\
break the staff over your knee or against a solid sur-\
face. The staff is destroyed and releases its magic in\
an explosion that fills a 30-foot Emanation originat-\
ing from itself. You have a 50 percent chance to in-\
stantly travel to a random plane of existence, avoid-\
ing the explosion. If you fail to avoid the effect, you\
take Force damage equal to 16 times the number\
of charges in the staff. Each other creature in the\
area makes a DC 17 Dexterity saving throw. On a\
failed save, a creature takes Force damage equal to\
6 times the number of charges in the staff. On a suc-\
cessful save, a creature takes half as much damage.\

Original layout: [[../../srd-5.2.1/pages/246|page 246]], [[../../srd-5.2.1/pages/247|page 247]].

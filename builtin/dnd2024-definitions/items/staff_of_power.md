---
splat_schema: 1
splat_id: f07a4ba2-2fb0-59dc-a460-dd85f85e57aa
splat_kind: item
splat_key: dnd2024:items/staff_of_power
splat_name: Staff of Power
splat_origin:
  key: dnd2024:reference/magic_item/staff_of_power
splat_sources:
- title: SRD 5.2.1
  locator: pp. 245, 246
  url: https://www.dndbeyond.com/srd
splat_item:
  category: staff
  rarity: very_rare
  equipment:
    attunement: required
    attunement_requirement_text: a Sorcerer, Warlock, or Wizard
splat_rule:
  mechanic: dnd2024:items/staff_of_power
  implementation: dnd2024:item_staff_of_power
  parameters:
    weapon:
      key: dnd2024:equipment/quarterstaff
    attack_roll_bonus: 2
    damage_roll_bonus: 2
    scope: attacks_with_source_item
    while_held:
      ac_bonus: 2
      saving_throw_bonus: 2
      spell_attack_bonus: 2
    charges:
      maximum: 20
      initial: 20
      recovery:
        event: dawn
        amount:
          expression: 2d8+4
          scope: use
        cap: maximum
    spell_grants_require: held
    spell_save_dc: user_spell_save_dc
    spell_grants:
    - spell:
        key: dnd2024:spells/cone_of_cold
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
        key: dnd2024:spells/fireball
      charge_cost: 5
      casting_time: spell_default
      method: item
      slot_cost: 0
      components: []
      level: 5
      range: spell_default
      duration: spell_default
      concentration: spell_default
    - spell:
        key: dnd2024:spells/globe_of_invulnerability
      charge_cost: 6
      casting_time: spell_default
      method: item
      slot_cost: 0
      components: []
      level: lowest_possible
      range: spell_default
      duration: spell_default
      concentration: spell_default
    - spell:
        key: dnd2024:spells/hold_monster
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
        key: dnd2024:spells/levitate
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
        key: dnd2024:spells/lightning_bolt
      charge_cost: 5
      casting_time: spell_default
      method: item
      slot_cost: 0
      components: []
      level: 5
      range: spell_default
      duration: spell_default
      concentration: spell_default
    - spell:
        key: dnd2024:spells/magic_missile
      charge_cost: 1
      casting_time: spell_default
      method: item
      slot_cost: 0
      components: []
      level: lowest_possible
      range: spell_default
      duration: spell_default
      concentration: spell_default
    - spell:
        key: dnd2024:spells/ray_of_enfeeblement
      charge_cost: 1
      casting_time: spell_default
      method: item
      slot_cost: 0
      components: []
      level: lowest_possible
      range: spell_default
      duration: spell_default
      concentration: spell_default
    - spell:
        key: dnd2024:spells/wall_of_force
      charge_cost: 5
      casting_time: spell_default
      method: item
      slot_cost: 0
      components: []
      level: lowest_possible
      range: spell_default
      duration: spell_default
      concentration: spell_default
    last_charge:
      table:
        roll:
          expression: 1d20
          scope: use
        rows:
        - min: 1
          max: 1
          result:
            retain_only:
            - weapon_attack_bonus
            - weapon_damage_bonus
        - min: 2
          max: 19
          result:
            effect: none
        - min: 20
          max: 20
          result:
            restore_charges:
              expression: 1d8+2
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
                - 4
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
- id: dnd2024:item_staff_of_power
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
  source_sha256: 1c7b3f088d4b70322b76c26cc7780b4d01ec1a02b2e45d72bf86e8f331523d26
  source_path: builtin/srd-5.2.1/magic_item/staff_of_power.md
  pages:
  - 245
  - 246
  engine_gaps: &id001
  - dnd2024:item_staff_of_power
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_2.md
  specification_sha256: be72efd89eed1690d60ad9725e3fa7f88c19164f444319fb2c13cf76855530cc
splat_clauses:
- key: source_rules
  text: '# Staff of Power Staff, Very Rare (Requires Attunement by a Sorcerer, Warlock, or Wizard) This
    staff has 20 charges and can be wielded as a magic Quarterstaff that grants a +2 bonus to attack rolls
    and damage rolls made with it. While holding it, you gain a +2 bonus to Armor Class, saving throws,
    and spell attack rolls. Spells. While holding the staff, you can cast one of the spells on the following
    table from it, using your spell save DC. The table indicates how many charges you must expend to cast
    the spell. Charge Spell Cost Cone of Cold 5 Fireball (level 5 version) 5 Globe of Invulnerability
    6 Hold Monster 5 Levitate 2 Lightning Bolt (level 5 version) 5 Magic Missile 1 Ray of Enfeeblement
    1 Wall of Force 5 Regaining Charges. The staff regains 2d8 + 4 expended charges daily at dawn. If
    you expend the last charge, roll 1d20. On a 1, the staff retains its +2 bonus to attack rolls and
    damage rolls but loses all other properties. On a 20, the staff regains 1d8 + 2 charges. Retributive
    Strike. You can take a Magic action to break the staff over your knee or against a solid surface.
    The staff is destroyed and releases its magic in an explosion that fills a 30-foot Emanation originating
    from itself. You have a 50 percent chance to instantly travel to a random plane of existence, avoiding
    the explosion. If you fail to avoid the effect, you take Force damage equal to 16 times the number
    of charges in the staff. Each other creature in the area makes a DC 17 Dexterity saving throw. On
    a failed save, a creature takes Force damage equal to 4 times the number of charges in the staff.
    On a successful save, a creature takes half as much damage.'
  sha256: 4c97ae51ad9c72c1210b5b17d2961fcb46fd1f28d5273bce7e5dfb659ae78b10
  status: contract_specified
splat_requires_mechanics: *id001
---

# Staff of Power

<!-- SRD page 245 -->

Staff, Very Rare (Requires Attunement by a Sorcerer,\
Warlock, or Wizard)\
This staff has 20 charges and can be wielded as a\
magic Quarterstaff that grants a +2 bonus to attack\
rolls and damage rolls made with it. While hold-\
ing it, you gain a +2 bonus to Armor Class, saving\
throws, and spell attack rolls.\
Spells. While holding the staff, you can cast one of\
the spells on the following table from it, using your\
spell save DC. The table indicates how many charges\
you must expend to cast the spell.\

<!-- SRD page 246 -->

Charge\
Spell Cost\
Cone of Cold 5\
Fireball (level 5 version) 5\
Globe of Invulnerability 6\
Hold Monster 5\
Levitate 2\
Lightning Bolt (level 5 version) 5\
Magic Missile 1\
Ray of Enfeeblement 1\
Wall of Force 5\
Regaining Charges. The staff regains 2d8 + 4\
expended charges daily at dawn. If you expend the\
last charge, roll 1d20. On a 1, the staff retains its +2\
bonus to attack rolls and damage rolls but loses all\
other properties. On a 20, the staff regains 1d8 + 2\
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
4 times the number of charges in the staff. On a suc-\
cessful save, a creature takes half as much damage.\

Original layout: [[../../srd-5.2.1/pages/245|page 245]], [[../../srd-5.2.1/pages/246|page 246]].

---
splat_schema: 1
splat_id: 7dcd9ca9-ad7d-50da-87aa-c639841dd906
splat_kind: item
splat_key: dnd2024:items/luck_blade
splat_name: Luck Blade
splat_origin:
  key: dnd2024:reference/magic_item/luck_blade
splat_sources:
- title: SRD 5.2.1
  locator: pp. 229
  url: https://www.dndbeyond.com/srd
splat_item:
  category: weapon
  rarity: legendary
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/luck_blade
  implementation: dnd2024:item_luck_blade
  parameters:
    attack_roll_bonus: 1
    damage_roll_bonus: 1
    scope: attacks_with_source_item
    on_person:
      saving_throw_bonus: 1
      luck:
        action: none
        requires_not_condition: incapacitated
        trigger: failed_d20_test
        reroll: true
        must_use_second: true
        uses:
          maximum: 1
          initial: 1
          recovery:
            event: dawn
            amount:
              expression: '1'
              scope: use
            cap: maximum
    wish:
      equipment_state: held
      charges:
        initial:
          expression: 1d3
          scope: use
        recovery: null
      spell_grant:
        spell:
          key: dnd2024:spells/wish
        charge_cost: 1
        casting_time: spell_default
        method: item
        slot_cost: 0
        components: []
        level: lowest_possible
        range: spell_default
        duration: spell_default
        concentration: spell_default
      uses_per_dawn: 1
      on_zero_charges: lose_wish_property
    attunement:
      required: true
    common_rules:
    - key: dnd2024:rules/magic_item_categories_weapons
    - key: dnd2024:rules/activating_a_magic_item_spells_cast_from_items
splat_contracts:
- id: dnd2024:item_luck_blade
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
  outputs:
  - item_state
  - effects
  - resolutions
  events:
  - attack.roll_pending
  - saving_throw.pending
  - d20_test.failed
  - spell.cast_pending
  - time.dawn
  - attunement.changed
  capabilities:
  - inventory
  - effects
  - spellcasting
  - resources_and_time
  - roll_resolution
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 82fffd76b0511540a561c297ee40ced2beac3763bd4f7fbdeb3030acbd5452f0
  source_path: builtin/srd-5.2.1/magic_item/luck_blade.md
  pages:
  - 229
  engine_gaps: &id001
  - dnd2024:item_luck_blade
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_2.md
  specification_sha256: a92c530a56d87acc30982d2a0f9e055b1d476510807fe4f82f2e35826a9bbee6
splat_clauses:
- key: source_rules
  text: '# Luck Blade Weapon (Glaive, Greatsword, Longsword, Rapier, Scimitar, Sickle, or Shortsword),
    Legendary (Requires Attunement) You gain a +1 bonus to attack rolls and damage rolls made with this
    magic weapon. While the weapon is on your person, you also gain a +1 bonus to saving throws. Luck.
    If the weapon is on your person, you can call on its luck (no action required) to reroll one failed
    D20 Test if you don’t have the Incapacitated condition. You must use the second roll. Once used, this
    property can’t be used again until the next dawn. Wish. The weapon has 1d3 charges. While holding
    it, you can expend 1 charge and cast Wish from it. Once used, this property can’t be used again until
    the next dawn. The weapon loses this property if it has no charges.'
  sha256: 082c807dfd450e88650a28414b6757712a0670ae5bb8410cbe94c8469da1c4b4
  status: contract_specified
splat_requires_mechanics: *id001
---

# Luck Blade

<!-- SRD page 229 -->

Weapon (Glaive, Greatsword, Longsword, Rapier,\
Scimitar, Sickle, or Shortsword), Legendary (Requires\
Attunement)\
You gain a +1 bonus to attack rolls and damage rolls\
made with this magic weapon. While the weapon is\
on your person, you also gain a +1 bonus to saving\
throws.\
Luck. If the weapon is on your person, you can call\
on its luck (no action required) to reroll one failed\
D20 Test if you don’t have the Incapacitated condi-\
tion. You must use the second roll. Once used, this\
property can’t be used again until the next dawn.\
Wish. The weapon has 1d3 charges. While holding\
it, you can expend 1 charge and cast Wish from it.\
Once used, this property can’t be used again until\
the next dawn. The weapon loses this property if it\
has no charges.\

Original layout: [[../../srd-5.2.1/pages/229|page 229]].

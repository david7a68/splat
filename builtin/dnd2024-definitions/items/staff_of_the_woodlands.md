---
splat_schema: 1
splat_id: 967410a1-be27-5ac3-b0f8-f61e07e362d8
splat_kind: item
splat_key: dnd2024:items/staff_of_the_woodlands
splat_name: Staff of the Woodlands
splat_origin:
  key: dnd2024:reference/magic_item/staff_of_the_woodlands
splat_sources:
- title: SRD 5.2.1
  locator: pp. 247
  url: https://www.dndbeyond.com/srd
splat_item:
  category: staff
  rarity: rare
  equipment:
    attunement: required
    attunement_requirement_text: a Druid
splat_rule:
  mechanic: dnd2024:items/staff_of_the_woodlands
  implementation: dnd2024:item_staff_of_the_woodlands
  parameters:
    charges:
      maximum: 6
      initial: 6
      recovery:
        event: dawn
        amount:
          dice: 1
          sides: 6
          bonus: 0
    weapon:
      base:
        key: dnd2024:equipment/quarterstaff
      magical: true
      attack_bonus: 2
      damage_bonus: 2
    held:
      spell_attack_bonus: 2
    spells:
    - spell:
        key: dnd2024:spells/animal_friendship
      cost: 1
      save_dc: user_spell_save_dc
      requires: held
    - spell:
        key: dnd2024:spells/awaken
      cost: 5
      save_dc: user_spell_save_dc
      requires: held
    - spell:
        key: dnd2024:spells/barkskin
      cost: 2
      save_dc: user_spell_save_dc
      requires: held
    - spell:
        key: dnd2024:spells/locate_animals_or_plants
      cost: 2
      save_dc: user_spell_save_dc
      requires: held
    - spell:
        key: dnd2024:spells/pass_without_trace
      cost: 2
      save_dc: user_spell_save_dc
      requires: held
    - spell:
        key: dnd2024:spells/speak_with_animals
      cost: 1
      save_dc: user_spell_save_dc
      requires: held
    - spell:
        key: dnd2024:spells/speak_with_plants
      cost: 3
      save_dc: user_spell_save_dc
      requires: held
    - spell:
        key: dnd2024:spells/wall_of_thorns
      cost: 6
      save_dc: user_spell_save_dc
      requires: held
    tree_form:
      action: magic
      requires:
        plant_end_in: earth
        space: unoccupied
      cost: 1
      height_ft: 60
      trunk_diameter_ft: 5
      top_branch_radius_ft: 20
      appearance: ordinary_healthy_tree
      detect_magic:
        school: transmutation
        intensity: faint
      revert:
        action: magic
        requires: touch_tree
        occupants: fall
    last_charge:
      roll:
        dice: 1
        sides: 20
        bonus: 0
      on_value: 1
      effect: nonmagical_quarterstaff
    attunement:
      required: true
      eligibility:
        class_any:
        - druid
splat_contracts:
- id: dnd2024:item_staff_of_the_woodlands
  inputs:
    user:
      type: participant
      required: true
    item:
      type: item_instance
      required: true
    choice:
      required: false
      type: choice
    targets:
      required: false
      type: participant_list
  outputs:
  - effects
  - item_state
  events:
  - item.used
  - spell.cast_completed
  capabilities:
  - equipment
  - item_resources
  - effects
  - item_spellcasting
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: b210d65a99ce1d82fc259ab901ab91bddaccb287de5f273701d2f07a95164a2c
  source_path: builtin/srd-5.2.1/magic_item/staff_of_the_woodlands.md
  pages:
  - 247
  engine_gaps: &id001
  - dnd2024:item_staff_of_the_woodlands
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_1.md
  specification_sha256: c112b9758a4a3fcedb45182fe791d7950195cacff4d2f88bd348ce0605ddc8d9
splat_clauses:
- key: source_rules
  text: '# Staff of the Woodlands Staff, Rare (Requires Attunement by a Druid) This staff has 6 charges
    and can be wielded as a magic Quarterstaff that grants a +2 bonus to attack rolls and damage rolls
    made with it. While holding it, you have a +2 bonus to spell attack rolls. Spells. While holding the
    staff, you can cast one of the spells on the following table from it, using your spell save DC. The
    table indicates how many charges you must expend to cast the spell. Spell Charge Cost Animal Friendship
    1 Awaken 5 Barkskin 2 Locate Animals or Plants 2 Pass without Trace 2 Speak with Animals 1 Speak with
    Plants 3 Wall of Thorns 6 Tree Form. You can take a Magic action to plant one end of the staff in
    earth in an unoccupied space and expend 1 charge to transform the staff into a healthy tree. The tree
    is 60 feet tall and has a 5-foot-diameter trunk, and its branches at the top spread out in a 20-foot
    radius. The tree appears ordinary but radiates a faint aura of Transmutation magic that can be discerned
    with the Detect Magic spell. While touching the tree and using a Magic action, you return the staff
    to its normal form. Any creature in the tree falls when the tree reverts to a staff. Regaining Charges.
    The staff regains 1d6 expended charges daily at dawn. If you expend the last charge, roll 1d20. On
    a 1, the staff loses its properties and becomes a nonmagical Quarterstaff.'
  sha256: 36f5ea86b9dd46cb3b3fe0649fde12dc314e9140656a90ad63ea9cc29e6abb67
  status: contract_specified
splat_requires_mechanics: *id001
---

# Staff of the Woodlands

<!-- SRD page 247 -->

Staff, Rare (Requires Attunement by a Druid)\
This staff has 6 charges and can be wielded as a\
magic Quarterstaff that grants a +2 bonus to attack\
rolls and damage rolls made with it. While holding\
it, you have a +2 bonus to spell attack rolls.\
Spells. While holding the staff, you can cast one of\
the spells on the following table from it, using your\
spell save DC. The table indicates how many charges\
you must expend to cast the spell.\
Spell Charge\
Cost\
Animal Friendship 1\
Awaken 5\
Barkskin 2\
Locate Animals or Plants 2\
Pass without Trace 2\
Speak with Animals 1\
Speak with Plants 3\
Wall of Thorns 6\
Tree Form. You can take a Magic action to plant\
one end of the staff in earth in an unoccupied space\
and expend 1 charge to transform the staff into\
a healthy tree. The tree is 60 feet tall and has a\
5-foot-diameter trunk, and its branches at the top\
spread out in a 20-foot radius. The tree appears\
ordinary but radiates a faint aura of Transmutation\
magic that can be discerned with the Detect Magic\
spell. While touching the tree and using a Magic\
action, you return the staff to its normal form. Any\
creature in the tree falls when the tree reverts to\
a staff.\
Regaining Charges. The staff regains 1d6 ex-\
pended charges daily at dawn. If you expend the last\
charge, roll 1d20. On a 1, the staff loses its proper-\
ties and becomes a nonmagical Quarterstaff.\

Original layout: [[../../srd-5.2.1/pages/247|page 247]].

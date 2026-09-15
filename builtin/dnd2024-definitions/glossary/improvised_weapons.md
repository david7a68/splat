---
splat_schema: 1
splat_id: 6e5c3914-970e-52ef-b80e-96ff9880e804
splat_kind: rule
splat_key: dnd2024:glossary/improvised_weapons
splat_name: Improvised Weapons
splat_origin:
  key: dnd2024:reference/glossary/improvised_weapons
splat_sources:
- title: SRD 5.2.1
  locator: pp. 183
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/improvised_weapons
  parameters:
    qualifies_any:
    - object_used_as_makeshift_weapon
    - ranged_weapon_used_for_melee
    - melee_weapon_without_thrown_property_thrown
    - weapon_used_contrary_to_design
    proficiency_bonus_added: false
    damage:
      dice: 1d4
      type_choice:
        chooser: gm
        options:
        - acid
        - bludgeoning
        - cold
        - fire
        - force
        - lightning
        - necrotic
        - piercing
        - poison
        - psychic
        - radiant
        - slashing
        - thunder
        constraint: appropriate_for_object
    thrown_range_feet:
      normal: 20
      long: 60
    weapon_equivalent:
      optional: true
      chooser: gm
      requires_resemblance: true
      allowed_categories:
      - simple
      - martial
      replacement_rules: selected_weapon_all_rules
  implementation: dnd2024:glossary_improvised_weapons
splat_contracts:
- id: dnd2024:glossary_improvised_weapons
  inputs:
    subject:
      type: participant
      required: true
    weapon:
      type: item_instance
      required: true
    use_mode:
      type: choice
      required: true
    gm_damage_type:
      type: choice
      required: true
    equivalent_weapon:
      required: false
      type: definition
  outputs:
  - weapon_attack_profile
  events:
  - attack.pending
  capabilities:
  - equipment
  - attacks
  - damage
  - gm_choices
splat_conversion:
  status: specified
  category: glossary
  source_sha256: 0a1156a4b93fd42e11fd307a5551d9211a7fdd2cafb851cdd4b722a415a1a089
  source_path: builtin/srd-5.2.1/glossary/improvised_weapons.md
  pages:
  - 183
  engine_gaps: &id001
  - dnd2024:glossary_improvised_weapons
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: df8a06a6ffac4b6498cb1a372aa6e8cea0f07c8ce9035071ca41fe52488c2df2
splat_clauses:
- key: source_rules
  text: '# Improvised Weapons An improvised weapon is an object wielded as a makeshift weapon, such as
    broken glass, a table leg, or a frying pan. A Simple or Martial weapon also counts as an improvised
    weapon if it’s wielded in a way contrary to its design; if you use a Ranged weapon to make a melee
    attack or throw a Melee weapon that lacks the Thrown property, the weapon counts as an improvised
    weapon. An improvised weapon follows the rules below. Proficiency. Don’t add your Proficiency Bonus
    to attack rolls with an improvised weapon. Damage. On a hit, the weapon deals 1d4 damage of a type
    the GM thinks is appropriate for the object. Range. If you throw the weapon, it has a normal range
    of 20 feet and a long range of 60 feet. Weapon Equivalents. If an improvised weapon resembles a Simple
    or Martial weapon, the GM may say it functions as that weapon and uses that weapon’s rules. For example,
    the GM could treat a table leg as a Club.'
  sha256: 80c9a321ed52550ec351d2ee88a26b0959a1ebec1a315eed5329e9a05c6452a9
  status: contract_specified
splat_requires_mechanics: *id001
---

# Improvised Weapons

<!-- SRD page 183 -->

An improvised weapon is an object wielded as a\
makeshift weapon, such as broken glass, a table leg,\
or a frying pan. A Simple or Martial weapon also\
counts as an improvised weapon if it’s wielded in\
a way contrary to its design; if you use a Ranged\
weapon to make a melee attack or throw a Me-\
lee weapon that lacks the Thrown property, the\
weapon counts as an improvised weapon. An impro-\
vised weapon follows the rules below.\
Proficiency. Don’t add your Proficiency Bonus to\
attack rolls with an improvised weapon.\
Damage. On a hit, the weapon deals 1d4 damage\
of a type the GM thinks is appropriate for the object.\
Range. If you throw the weapon, it has a normal\
range of 20 feet and a long range of 60 feet.\
Weapon Equivalents. If an improvised weapon\
resembles a Simple or Martial weapon, the GM may\
say it functions as that weapon and uses that weap-\
on’s rules. For example, the GM could treat a table\
leg as a Club.\

Original layout: [[../../srd-5.2.1/pages/183|page 183]].

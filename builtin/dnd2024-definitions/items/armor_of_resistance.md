---
splat_schema: 1
splat_id: 74b311c4-16a8-53a2-a8bf-e375602b61e8
splat_kind: item
splat_key: dnd2024:items/armor_of_resistance
splat_name: Armor of Resistance
splat_origin:
  key: dnd2024:reference/magic_item/armor_of_resistance
splat_sources:
- title: SRD 5.2.1
  locator: pp. 210
  url: https://www.dndbeyond.com/srd
splat_item:
  category: armor
  rarity: rare
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/armor_of_resistance
  implementation: dnd2024:item_armor_of_resistance
  parameters:
    equipment_state: worn
    resistance:
      chooser: gm
      method:
      - choose
      - roll
      table:
        roll:
          expression: 1d10
          scope: use
        rows:
        - min: 1
          max: 1
          result:
            damage_type: acid
        - min: 2
          max: 2
          result:
            damage_type: cold
        - min: 3
          max: 3
          result:
            damage_type: fire
        - min: 4
          max: 4
          result:
            damage_type: force
        - min: 5
          max: 5
          result:
            damage_type: lightning
        - min: 6
          max: 6
          result:
            damage_type: necrotic
        - min: 7
          max: 7
          result:
            damage_type: poison
        - min: 8
          max: 8
          result:
            damage_type: psychic
        - min: 9
          max: 9
          result:
            damage_type: radiant
        - min: 10
          max: 10
          result:
            damage_type: thunder
    attunement:
      required: true
    common_rules:
    - key: dnd2024:rules/magic_item_categories_armor
splat_contracts:
- id: dnd2024:item_armor_of_resistance
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
  - equipment.changed
  - attunement.changed
  capabilities:
  - inventory
  - effects
  - roll_resolution
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: f7c4fe5099d53f0b693340873dc55825a6597ebbb9d8c0a02bd1bb93d177f5cb
  source_path: builtin/srd-5.2.1/magic_item/armor_of_resistance.md
  pages:
  - 210
  engine_gaps: &id001
  - dnd2024:item_armor_of_resistance
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_2.md
  specification_sha256: b6a65cc21953a90642ab57139f57efb270386acd90aa1ee9aa871d75993d5489
splat_clauses:
- key: source_rules
  text: '# Armor of Resistance Armor (Any Light, Medium, or Heavy), Rare (Requires Attunement) You have
    Resistance to one type of damage while you wear this armor. The GM chooses the type or determines
    it randomly by rolling on the following table. 1d10 Damage Type 1d10 Damage Type 1 Acid 6 Necrotic
    2 Cold 7 Poison 3 Fire 8 Psychic 4 Force 9 Radiant 5 Lightning 10 Thunder Down Speed to 0 and making
    it unable to benefit from bonuses to speed. Forward window shutter closes. Side window shutters close
    (two per side). The claws retract. Each extended claw makes the following melee attack: +8 to hit,
    reach 5 ft. Hit: The target has the Grappled condition (escape DC 15). The apparatus walks or swims
    backward provided its legs are extended. The apparatus turns 90 degrees clockwise provided its legs
    are extended. The light turns off. The apparatus rises up to 20 feet if it’s in liquid. The rear hatch
    closes and seals.'
  sha256: 1c5b3dec7eeb73002477d8b734b22fcdfe8c9a1ccbbb8af4358d0c753737aae4
  status: contract_specified
splat_requires_mechanics: *id001
---

# Armor of Resistance

<!-- SRD page 210 -->

Armor (Any Light, Medium, or Heavy), Rare (Requires\
Attunement)\
You have Resistance to one type of damage while\
you wear this armor. The GM chooses the type or\
determines it randomly by rolling on the following\
table.\
1d10 Damage Type 1d10 Damage Type\
1 Acid 6 Necrotic\
2 Cold 7 Poison\
3 Fire 8 Psychic\
4 Force 9 Radiant\
5 Lightning 10 Thunder\
Down\
Speed to 0 and making it unable to benefit from bo-\
nuses to speed.\
Forward window shutter closes.\
Side window shutters close (two per side).\
The claws retract.\
Each extended claw makes the following melee attack:\
+8 to hit, reach 5 ft. Hit: The target has the Grappled\
condition (escape DC 15).\
The apparatus walks or swims backward provided its\
legs are extended.\
The apparatus turns 90 degrees clockwise provided its\
legs are extended.\
The light turns off.\
The apparatus rises up to 20 feet if it’s in liquid.\
The rear hatch closes and seals.\

Original layout: [[../../srd-5.2.1/pages/210|page 210]].

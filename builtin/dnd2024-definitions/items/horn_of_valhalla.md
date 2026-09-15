---
splat_schema: 1
splat_id: 5f147a6f-13c7-5507-8da3-e355a113d025
splat_kind: item
splat_key: dnd2024:items/horn_of_valhalla
splat_name: Horn of Valhalla
splat_origin:
  key: dnd2024:reference/magic_item/horn_of_valhalla
splat_sources:
- title: SRD 5.2.1
  locator: pp. 226
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: rare
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/horn_of_valhalla
  implementation: dnd2024:item_horn_of_valhalla
  parameters:
    activation:
      action: magic
      operation: blow_horn
      recovery:
        value: 7
        unit: day
      summon:
        creature:
          key: dnd2024:creatures/berserker
        count_from: variant.spirits
        placement:
          unoccupied: true
          range_ft: 60
        origin_plane: ysgard
        duration:
          value: 1
          unit: hour
        return_on:
        - duration
        - hp_zero
        condition_immunities:
        - charmed
        - frightened
        requirements_met:
          friendly_to:
          - user
          - allies
          commands: obey
        requirements_not_met:
          attack: user
    variant_selection:
    - gm_choice
    - d100
    variants:
    - min: 1
      max: 40
      metal: silver
      rarity: rare
      spirits: 2
      requirement: null
    - min: 41
      max: 75
      metal: brass
      rarity: rare
      spirits: 3
      requirement:
        proficiency: all_simple_weapons
    - min: 76
      max: 90
      metal: bronze
      rarity: very_rare
      spirits: 4
      requirement:
        training: all_medium_armor
    - min: 91
      max: 100
      metal: iron
      rarity: legendary
      spirits: 5
      requirement:
        proficiency: all_martial_weapons
    attunement:
      required: false
splat_contracts:
- id: dnd2024:item_horn_of_valhalla
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
    locations:
      required: false
      type: location_list
    campaign:
      required: false
      type: campaign
  outputs:
  - summoned_creatures
  - item_state
  events:
  - item.used
  - turn.started
  - creature.died
  capabilities:
  - summoning
  - initiative
  - item_resources
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 1318ad2196fb09359c3297cde4e5252671e35004d2d870d935dd53689636a5d5
  source_path: builtin/srd-5.2.1/magic_item/horn_of_valhalla.md
  pages:
  - 226
  engine_gaps: &id001
  - dnd2024:item_horn_of_valhalla
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_1.md
  specification_sha256: 3c96b2de7d0a6a1f790be93ebf5a6921dfb40b69fe07a005651077febc870571
splat_clauses:
- key: source_rules
  text: '# Horn of Valhalla Wondrous Item, Rare (Silver or Brass), Very Rare (Bronze), or Legendary (Iron)
    You can take a Magic action to blow this horn. In response, warrior spirits from the plane of Ysgard
    appear in unoccupied spaces within 60 feet of you. Each spirit uses the Berserker stat block and returns
    to Ysgard after 1 hour or when it drops to 0 Hit Points. The spirits look like living, breathing warriors,
    and they have Immunity to the Charmed and Frightened conditions. Once you use the horn, it can’t be
    used again until 7 days have passed. Four types of Horn of Valhalla are known to exist, each made
    of a different metal. The horn’s type determines how many spirits it summons, as well as the requirement
    for its use. The GM chooses the horn’s type or determines it randomly by rolling on the following
    table. If you blow the horn without meeting its requirement, the summoned spirits attack you. If you
    meet the requirement, they are Friendly to you and your allies and follow your commands. 1d100 Horn
    Type Spirits Requirement 01–40 Silver 2 None 41–75 Brass 3 Proficiency with all Simple weapons 76–90
    Bronze 4 Training with all Medium armor 91–00 Iron 5 Proficiency with all Martial weapons'
  sha256: 2e7435f888409949d709d36850cc6a98c99cae3534e2abb5f3f019ae11c30d9b
  status: contract_specified
splat_requires_mechanics: *id001
---

# Horn of Valhalla

<!-- SRD page 226 -->

Wondrous Item, Rare (Silver or Brass), Very Rare\
(Bronze), or Legendary (Iron)\
You can take a Magic action to blow this horn. In\
response, warrior spirits from the plane of Ysgard\
appear in unoccupied spaces within 60 feet of you.\
Each spirit uses the Berserker stat block and re-\
turns to Ysgard after 1 hour or when it drops to 0\
Hit Points. The spirits look like living, breathing\
warriors, and they have Immunity to the Charmed\
and Frightened conditions. Once you use the horn, it\
can’t be used again until 7 days have passed.\
Four types of Horn of Valhalla are known to ex-\
ist, each made of a different metal. The horn’s type\
determines how many spirits it summons, as well\
as the requirement for its use. The GM chooses the\
horn’s type or determines it randomly by rolling on\
the following table.\
If you blow the horn without meeting its require-\
ment, the summoned spirits attack you. If you meet\
the requirement, they are Friendly to you and your\
allies and follow your commands.\
1d100 Horn Type Spirits Requirement\
01–40 Silver 2 None\
41–75 Brass 3 Proficiency with all\
Simple weapons\
76–90 Bronze 4 Training with all\
Medium armor\
91–00 Iron 5 Proficiency with all\
Martial weapons\

Original layout: [[../../srd-5.2.1/pages/226|page 226]].

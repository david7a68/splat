---
splat_schema: 1
splat_id: 04fc51cd-ffc5-5b16-81be-acdde63dd5eb
splat_kind: item
splat_key: dnd2024:items/ammunition_of_slaying
splat_name: Ammunition of Slaying
splat_origin:
  key: dnd2024:reference/magic_item/ammunition_of_slaying
splat_sources:
- title: SRD 5.2.1
  locator: pp. 209
  url: https://www.dndbeyond.com/srd
splat_item:
  category: weapon
  rarity: very_rare
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/ammunition_of_slaying
  implementation: dnd2024:item_ammunition_of_slaying
  parameters:
    trigger: item.damage_dealt
    target_type:
      selection: gm_choice_or_roll
      die: 100
      rows:
      - - 1
        - 10
        - aberration
      - - 11
        - 15
        - beast
      - - 16
        - 20
        - celestial
      - - 21
        - 25
        - construct
      - - 26
        - 35
        - dragon
      - - 36
        - 45
        - elemental
      - - 46
        - 50
        - humanoid
      - - 51
        - 60
        - fey
      - - 61
        - 70
        - fiend
      - - 71
        - 75
        - giant
      - - 76
        - 80
        - monstrosity
      - - 81
        - 85
        - ooze
      - - 86
        - 90
        - plant
      - - 91
        - 100
        - undead
      columns:
      - minimum_roll
      - maximum_roll
      - creature_type
    if_target_matches: true
    save:
      ability: constitution
      dc: 17
      failure:
        damage:
          amount:
            roll:
              count: 6
              sides: 10
          type: force
      success:
        damage_fraction:
        - 1
        - 2
    after_extra_damage:
      item_magic: false
    attunement_required: false
splat_contracts:
- id: dnd2024:item_ammunition_of_slaying
  inputs:
    actor:
      type: participant
      required: true
    item:
      type: item_instance
      required: true
    targets:
      required: false
      type: participant_list
    objects:
      required: false
      type: object_list
    destination:
      required: false
      type: location
    selection:
      required: false
      type: choice
    context:
      required: false
      type: resolution
  outputs:
  - effects
  - item_state_changes
  events:
  - damage.resolved
  capabilities:
  - item_effects
  - dice
  - damage
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 93923b7e5081315e87a9c45f525ef4643a7b5862719da9c9e95b6173387f9930
  source_path: builtin/srd-5.2.1/magic_item/ammunition_of_slaying.md
  pages:
  - 209
  engine_gaps: &id001
  - dnd2024:item_ammunition_of_slaying
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_3.md
  specification_sha256: cc51ee28fa7190988ac78504fa406563500423555903a538527e7246636ba0a0
splat_clauses:
- key: source_rules
  text: '# Ammunition of Slaying Weapon (Any Ammunition), Very Rare This magic ammunition is meant to
    slay creatures of a particular type, which the GM chooses or determines randomly by rolling on the
    table below. If a creature of that type takes damage from the ammunition, the creature makes a DC
    17 Constitution saving throw, taking an extra 6d10 Force damage on a failed save or half as much extra
    damage on a successful one. After dealing its extra damage to a creature, the ammunition becomes nonmagical.
    1d100 Creature Type 1d100 Creature Type 01–10 Aberrations 51–60 Fey 11–15 Beasts 61–70 Fiends 16–20
    Celestials 71–75 Giants 21–25 Constructs 76–80 Monstrosities 26–35 Dragons 81–85 Oozes 36–45 Elementals
    86–90 Plants 46–50 Humanoids 91–00 Undead'
  sha256: d9486fa776ac152ae62ca973387e754056c4e0c52de8aa92741316289a0debe8
  status: contract_specified
splat_requires_mechanics: *id001
---

# Ammunition of Slaying

<!-- SRD page 209 -->

Weapon (Any Ammunition), Very Rare\
This magic ammunition is meant to slay creatures\
of a particular type, which the GM chooses or de-\
termines randomly by rolling on the table below. If\
a creature of that type takes damage from the am-\
munition, the creature makes a DC 17 Constitution\
saving throw, taking an extra 6d10 Force damage\
on a failed save or half as much extra damage on a\
successful one.\
After dealing its extra damage to a creature, the\
ammunition becomes nonmagical.\
1d100 Creature Type 1d100 Creature Type\
01–10 Aberrations 51–60 Fey\
11–15 Beasts 61–70 Fiends\
16–20 Celestials 71–75 Giants\
21–25 Constructs 76–80 Monstrosities\
26–35 Dragons 81–85 Oozes\
36–45 Elementals 86–90 Plants\
46–50 Humanoids 91–00 Undead\

Original layout: [[../../srd-5.2.1/pages/209|page 209]].

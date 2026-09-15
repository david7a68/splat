---
splat_schema: 1
splat_id: 2c8c118c-af0e-5f3a-b75f-aab0d29ea5df
splat_kind: item
splat_key: dnd2024:items/wand_of_enemy_detection
splat_name: Wand of Enemy Detection
splat_origin:
  key: dnd2024:reference/magic_item/wand_of_enemy_detection
splat_sources:
- title: SRD 5.2.1
  locator: pp. 250
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wand
  rarity: rare
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/wand_of_enemy_detection
  implementation: dnd2024:item_wand_of_enemy_detection
  parameters:
    equipment_state: held
    charges:
      maximum: 7
      initial: 7
      recovery:
        event: dawn
        amount:
          dice: 1
          sides: 6
          bonus: 1
    activation:
      action: magic
      cost: 1
      duration:
        value: 1
        unit: minute
      end_on: stop_holding_wand
      detection:
        select: nearest_hostile_creature
        range_ft: 60
        output:
        - direction
        distance_revealed: false
        detect_despite:
        - invisible
        - ethereal
        - disguised
        - hidden
    last_charge:
      roll:
        dice: 1
        sides: 20
        bonus: 0
      on_value: 1
      effect: destroy_item
    attunement:
      required: true
splat_contracts:
- id: dnd2024:item_wand_of_enemy_detection
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
  capabilities:
  - equipment
  - item_resources
  - effects
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 6ccf92e07a3a37eb3dc02cdc5569d35d053e5dfdf2ef5471d4510a1eecd7a5f2
  source_path: builtin/srd-5.2.1/magic_item/wand_of_enemy_detection.md
  pages:
  - 250
  engine_gaps: &id001
  - dnd2024:item_wand_of_enemy_detection
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_1.md
  specification_sha256: e91893e204c781a3a7f7e639e8fcb4ab3b161695f9d4ec0b001752f958ed4597
splat_clauses:
- key: source_rules
  text: '# Wand of Enemy Detection Wand, Rare (Requires Attunement) This wand has 7 charges. While holding
    it, you can take a Magic action to expend 1 charge. For 1 minute, you know the direction of the nearest
    creature Hostile to you within 60 feet, but not its distance from you. The wand can sense the presence
    of Hostile creatures that are Invisible, ethereal, disguised, or hidden, as well as those in plain
    sight. The effect ends if you stop holding the wand. Regaining Charges. The wand regains 1d6 + 1 expended
    charges daily at dawn. If you expend the wand’s last charge, roll 1d20. On a 1, the wand crumbles
    into ashes and is destroyed.'
  sha256: 1b12f163e7615bb1681bc218e31da41362cc9f3b9e8560684c0bffb4c852eb51
  status: contract_specified
splat_requires_mechanics: *id001
---

# Wand of Enemy Detection

<!-- SRD page 250 -->

Wand, Rare (Requires Attunement)\
This wand has 7 charges. While holding it, you can\
take a Magic action to expend 1 charge. For 1 min-\
ute, you know the direction of the nearest creature\
Hostile to you within 60 feet, but not its distance\
from you. The wand can sense the presence of Hos-\
tile creatures that are Invisible, ethereal, disguised,\
or hidden, as well as those in plain sight. The effect\
ends if you stop holding the wand.\
Regaining Charges. The wand regains 1d6 +\
1 expended charges daily at dawn. If you expend\
the wand’s last charge, roll 1d20. On a 1, the wand\
crumbles into ashes and is destroyed.\

Original layout: [[../../srd-5.2.1/pages/250|page 250]].

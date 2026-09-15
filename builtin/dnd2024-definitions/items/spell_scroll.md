---
splat_schema: 1
splat_id: 6c43d977-5f9f-588b-98c0-de2cfe98c2dc
splat_kind: item
splat_key: dnd2024:items/spell_scroll
splat_name: Spell Scroll
splat_origin:
  key: dnd2024:reference/magic_item/spell_scroll
splat_sources:
- title: SRD 5.2.1
  locator: pp. 244
  url: https://www.dndbeyond.com/srd
splat_item:
  category: scroll
  rarity: rarity_varies
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/spell_scroll
  implementation: dnd2024:item_spell_scroll
  parameters:
    stored_spell:
      count: 1
      definition_type: spell
    read_and_cast:
      requires: spell_on_user_spell_list
      otherwise: unintelligible
      casting_time: normal_spell_casting_time
      material_components_required: false
      on_cast_completed: destroy_scroll
      on_cast_interrupted: preserve_scroll
      higher_than_normally_castable:
        check:
          ability: user_spellcasting_ability
          dc:
            sum:
            - 10
            - value_from: spell_level
        failure:
        - erase_spell
        - no_other_effect
    level_table:
    - spell_level: 0
      rarity: common
      save_dc: 13
      attack_bonus: 5
    - spell_level: 1
      rarity: common
      save_dc: 13
      attack_bonus: 5
    - spell_level: 2
      rarity: uncommon
      save_dc: 13
      attack_bonus: 5
    - spell_level: 3
      rarity: uncommon
      save_dc: 15
      attack_bonus: 7
    - spell_level: 4
      rarity: rare
      save_dc: 15
      attack_bonus: 7
    - spell_level: 5
      rarity: rare
      save_dc: 17
      attack_bonus: 9
    - spell_level: 6
      rarity: very_rare
      save_dc: 17
      attack_bonus: 9
    - spell_level: 7
      rarity: very_rare
      save_dc: 18
      attack_bonus: 10
    - spell_level: 8
      rarity: very_rare
      save_dc: 18
      attack_bonus: 10
    - spell_level: 9
      rarity: legendary
      save_dc: 19
      attack_bonus: 11
    copy_to_spellbook:
      requires: wizard_spell
      check:
        ability: intelligence
        skill: arcana
        dc:
          sum:
          - 10
          - value_from: spell_level
      success: copy_spell
      regardless_of_result: destroy_scroll
    attunement:
      required: false
splat_contracts:
- id: dnd2024:item_spell_scroll
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
    campaign:
      required: false
      type: campaign
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
  source_sha256: 5618ee8ae6384b1194d6fff3c195af72dd50d31952b9e27706a47cbf9ed51b66
  source_path: builtin/srd-5.2.1/magic_item/spell_scroll.md
  pages:
  - 244
  engine_gaps: &id001
  - dnd2024:item_spell_scroll
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_1.md
  specification_sha256: 266ac7da863b5124da67f0f892049f6325a63a30b284b11fdf1d378e51b5f2ba
splat_clauses:
- key: source_rules
  text: '# Spell Scroll Scroll, Rarity Varies A Spell Scroll bears the words of a single spell, written
    in a mystical cipher. If the spell is on your spell list, you can read the scroll and cast its spell
    without Material components. Otherwise, the scroll is unintelligible. Casting the spell by reading
    the scroll requires the spell’s normal casting time. Once the spell is cast, the scroll crumbles to
    dust. If the casting is interrupted, the scroll isn’t lost. If the spell is on your spell list but
    of a higher level than you can normally cast, you make an ability check using your spellcasting ability
    to determine whether you cast the spell. The DC equals 10 plus the spell’s level. On a failed check,
    the spell disappears from the scroll with no other effect. The level of the spell on the scroll determines
    the spell’s saving throw DC and attack bonus, as well as the scroll’s rarity, as shown in the following
    table. Spell Level Rarity Save DC Attack Bonus Cantrip Common 13 +5 1 Common 13 +5 2 Uncommon 13 +5
    3 Uncommon 15 +7 Spell Level Rarity Save DC Attack Bonus 4 Rare 15 +7 5 Rare 17 +9 6 Very Rare 17
    +9 7 Very Rare 18 +10 8 Very Rare 18 +10 9 Legendary 19 +11 Copying a Scroll into a Spellbook. A Wizard
    spell on a Spell Scroll can be copied into a spellbook. When a spell is copied in this way, the copier
    must succeed on an Intelligence (Arcana) check with a DC equal to 10 plus the spell’s level. On a
    successful check, the spell is copied. Whether the check succeeds or fails, the Spell Scroll is destroyed.'
  sha256: 7e8c3b846f79f11ed80c0f12520efdb117633e6d7df4163d253fb2864ad782bc
  status: contract_specified
splat_requires_mechanics: *id001
---

# Spell Scroll

<!-- SRD page 244 -->

Scroll, Rarity Varies\
A Spell Scroll bears the words of a single spell, writ-\
ten in a mystical cipher. If the spell is on your spell\
list, you can read the scroll and cast its spell with-\
out Material components. Otherwise, the scroll is\
unintelligible. Casting the spell by reading the scroll\
requires the spell’s normal casting time. Once the\
spell is cast, the scroll crumbles to dust. If the cast-\
ing is interrupted, the scroll isn’t lost.\
If the spell is on your spell list but of a higher level\
than you can normally cast, you make an ability\
check using your spellcasting ability to determine\
whether you cast the spell. The DC equals 10 plus\
the spell’s level. On a failed check, the spell disap-\
pears from the scroll with no other effect.\
The level of the spell on the scroll determines the\
spell’s saving throw DC and attack bonus, as well as\
the scroll’s rarity, as shown in the following table.\
Spell Level Rarity Save DC Attack Bonus\
Cantrip Common 13 +5\
1 Common 13 +5\
2 Uncommon 13 +5\
3 Uncommon 15 +7\
Spell Level Rarity Save DC Attack Bonus\
4 Rare 15 +7\
5 Rare 17 +9\
6 Very Rare 17 +9\
7 Very Rare 18 +10\
8 Very Rare 18 +10\
9 Legendary 19 +11\
Copying a Scroll into a Spellbook. A Wizard spell\
on a Spell Scroll can be copied into a spellbook.\
When a spell is copied in this way, the copier must\
succeed on an Intelligence (Arcana) check with a\
DC equal to 10 plus the spell’s level. On a successful\
check, the spell is copied. Whether the check suc-\
ceeds or fails, the Spell Scroll is destroyed.\

Original layout: [[../../srd-5.2.1/pages/244|page 244]].
